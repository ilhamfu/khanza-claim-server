use std::sync::Arc;

use chrono::NaiveDate;
use tokio_util::sync::CancellationToken;

use crate::{
    bg::util::{export_checkup, ExportCheckupError},
    dto::{
        extra::{get_not_exported_completed_checkup, set_checkup_exported, ExportStatus},
        GetRawatError, GetRawatProcessError,
    },
    AppContext,
};

use super::util::{create_client, ExportProcessError};

#[derive(Debug, thiserror::Error)]
pub enum ExportRangeError {
    #[error("error when getting non-exported medical checkup: ")]
    GetNotExportedComplete(#[source] sqlx::Error),
    #[error("error when getting non-exported medical checkup: ")]
    GetRawatProcess(#[from] GetRawatProcessError),
    #[error("error when exporting medical checkup: ")]
    ExportProcess(#[from] ExportProcessError),
    #[error("error when adding record to log: ")]
    Marking(#[source] sqlx::Error),
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
        let status = match export_checkup(context, client, &no_rawat).await {
            Err(ExportCheckupError::GetRawat(GetRawatError::Process(err))) => {
                return Some(Err(err.into()))
            }
            Err(ExportCheckupError::ExportProcess(err)) => return Some(Err(err.into())),
            Err(ExportCheckupError::GetRawat(GetRawatError::Constraint(err))) => {
                tracing::warn!(missing = %err,no_rawat=%no_rawat, "Export aborted: missing required document");
                ExportStatus::NotComplete
            }
            Err(ExportCheckupError::MedicalCheckupNotExist) => {
                tracing::warn!(missing = "document not found", no_rawat=%no_rawat, "Export Aborted: missing required document");
                ExportStatus::NotComplete
            }
            Ok(()) => {
                tracing::info!( no_rawat=%no_rawat, "Export success");
                ExportStatus::Success
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
    let now = chrono::Local::now().naive_utc();

    let from_text = from.format("%d-%m-%Y");
    let to_text = to.format("%d-%m-%Y");
    let at_text = now.format("%d-%m-%Y %H:%M");

    tracing::info!("start exporting document from {from_text} to {to_text} at {at_text}");

    while let Some(val) = export_each(&context, &client, now, from, to).await {
        if let Err(err) = val {
            println!("{err:?}");
            tracing::error!(%err,"exporting document failed");
        }

        if cancel_token.is_cancelled() {
            break;
        }
    }

    Ok(())
}
