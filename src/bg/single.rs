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

use super::util::{create_client, export_checkup, ExportCheckupError};

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

        let status = match export_checkup(&context, &client, no_rawat).await {
            Err(ExportCheckupError::GetRawat(GetRawatError::Process(err))) => {
                // TODO: refactor to remove `anyhow`
                return Err(err).context("error disini");
            }
            Err(ExportCheckupError::ExportProcess(err)) => {
                // TODO: refactor to remove `anyhow`
                return Err(err).context("error disini");
            }
            Err(ExportCheckupError::GetRawat(GetRawatError::Constraint(err))) => {
                tracing::warn!(missing = %err, "Export aborted: missing required document");
                ExportStatus::NotComplete
            }
            Err(ExportCheckupError::MedicalCheckupNotExist) => {
                tracing::warn!(
                    missing = "document not found",
                    "Export aborted: missing required document"
                );
                ExportStatus::NotComplete
            }
            Ok(()) => ExportStatus::Success,
        };
        set_checkup_exported(&context.pool, no_rawat, now, status)
            .await
            .context("error when updating exported checkup date")?;
    }

    Ok(())
}
