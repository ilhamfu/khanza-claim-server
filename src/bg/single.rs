use std::sync::Arc;

use anyhow::Context;
use chrono::Utc;
use inquire::Confirm;

use crate::{
    dto::{
        extra::{
            is_medical_checkup_exists, set_checkup_exported, ExportStatus,
            IsMedicalCheckupExistsError,
        },
        GetRawatError,
    },
    AppContext,
};

use super::util::{create_client, export_checkup};

pub async fn export_one(
    context: Arc<AppContext>,
    no_rawat: &str,
    force: bool,
) -> anyhow::Result<()> {
    let is_exists = is_medical_checkup_exists(&context.pool, no_rawat).await;
    let now = Utc::now().naive_utc();
    let run = match is_exists {
        Err(IsMedicalCheckupExistsError::NotBpjs) => {
            println!("Medical checkup number is not using BPJS, exporting document cancelled!");
            return Ok(());
        }
        Err(IsMedicalCheckupExistsError::NotFound) => {
            println!("Medical checkup number not found, exporting document cancelled!");
            return Ok(());
        }
        Err(IsMedicalCheckupExistsError::Exported(_naive_date_time)) => Confirm::new(&format!(
            "Medical checkup exported at {}, continue?",
            _naive_date_time
                .and_utc()
                .with_timezone(&chrono::Local)
                .format("%d-%m-%Y %H:%M")
        ))
        .with_default(false)
        .prompt()?,
        Err(IsMedicalCheckupExistsError::Error(err)) => return Err(err),
        Ok(_) => true,
    };

    if run || force {
        let client = create_client().await?;

        let status = 'a: {
            let reg_periksa = match crate::dto::get_rawat(&context.pool, no_rawat).await {
                Ok(Some(val)) => val,
                Ok(None) => {
                    tracing::warn!(
                        missing = "document not found",
                        "Export aborted: missing required document"
                    );
                    break 'a ExportStatus::NotComplete;
                }
                Err(GetRawatError::Process(_err)) => {
                    todo!("add handler")
                }
                Err(GetRawatError::Constraint(err)) => {
                    tracing::warn!(missing = %err,no_rawat=%no_rawat,%now, "Export aborted: missing required document");
                    break 'a ExportStatus::NotComplete;
                }
            };
            match export_checkup(&context, &client, &reg_periksa).await {
                Err(_err) => {
                    todo!("add handler")
                }
                Ok(()) => {
                    tracing::info!( no_rawat=%no_rawat, "Export success");
                    break 'a ExportStatus::Success;
                }
            }
        };
        set_checkup_exported(&context.pool, no_rawat, now, status)
            .await
            .context("error when updating exported checkup date")?;
    }
    Ok(())
}
