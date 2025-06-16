use std::sync::Arc;

use chrono::{NaiveDate, Timelike};
use tokio_util::sync::CancellationToken;

use crate::{
    bg::util::export_checkup,
    dto::{
        extra::{get_not_exported_completed_checkup, set_checkup_exported, ExportStatus},
        GetRawatError, GetRawatProcessError,
    },
    AppContext,
};

use super::util::{create_client, ExportProcessError};

#[derive(Debug, thiserror::Error)]
pub enum ExportRangeError {
    #[error(transparent)]
    GetNotExportedComplete(sqlx::Error),
    #[error("error when getting document {no_rawat} : {source}")]
    GetRawatProcess {
        source: GetRawatProcessError,
        no_rawat: String,
    },
    #[error("error when exporting document {no_rawat} : {source}")]
    ExportProcess {
        source: ExportProcessError,
        no_rawat: String,
    },
    #[error(transparent)]
    Marking(sqlx::Error),
}

async fn export_each(
    context: &AppContext,
    client: &fantoccini::Client,
    export_date: chrono::NaiveDateTime,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> Option<Result<(), ExportRangeError>> {
    let no_rawat_list =
        match get_not_exported_completed_checkup(&context.pool, export_date, from, to).await {
            Ok(ok) => ok,
            Err(err) => return Some(Err(ExportRangeError::GetNotExportedComplete(err))),
        };
    let len = no_rawat_list.len();

    for no_rawat in no_rawat_list {
        let status = 'a: {
            let reg_periksa = match crate::dto::get_rawat(&context.pool, &no_rawat).await {
                Ok(Some(val)) => val,
                Ok(None) => {
                    tracing::warn!(
                        missing = "document not found",
                        "Export aborted: missing required document"
                    );
                    break 'a ExportStatus::NotComplete;
                }
                Err(GetRawatError::Process(err)) => {
                    return Some(Err(ExportRangeError::GetRawatProcess {
                        source: err,
                        no_rawat: no_rawat.to_owned(),
                    }))
                }
                Err(GetRawatError::Constraint(err)) => {
                    tracing::warn!(missing = %err,no_rawat=%no_rawat,%export_date, "Export aborted: missing required document");
                    break 'a ExportStatus::NotComplete;
                }
            };
            match export_checkup(context, client, &reg_periksa).await {
                Err(err) => {
                    return Some(Err(ExportRangeError::ExportProcess {
                        source: err,
                        no_rawat: no_rawat.to_owned(),
                    }))
                }
                Ok(()) => {
                    tracing::info!( no_rawat=%no_rawat, "Export success");
                    break 'a ExportStatus::Success;
                }
            }
        };

        if let Err(err) = set_checkup_exported(&context.pool, &no_rawat, export_date, status).await
        {
            return Some(Err(ExportRangeError::Marking(err)));
        };
    }
    if len >= 10 {
        Some(Ok(()))
    } else {
        None
    }
}

pub async fn export_range(
    cancel_token: CancellationToken,
    context: Arc<AppContext>,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<(), fantoccini::error::NewSessionError> {
    let client = create_client().await?;
    let now = chrono::Local::now()
        .naive_utc()
        .with_nanosecond(0)
        .expect("error lur");

    let from_text = from.format("%d-%m-%Y");
    let to_text = to.format("%d-%m-%Y");
    let at_text = now.format("%d-%m-%Y %H:%M");

    tracing::info!("start exporting document from {from_text} to {to_text} at {at_text}");

    while let Some(val) = export_each(&context, &client, now, from, to).await {
        if let Err(err) = val {
            tracing::error!(%err,"exporting document failed");
        }

        if cancel_token.is_cancelled() {
            break;
        }
    }

    Ok(())
}
