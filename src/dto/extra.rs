use anyhow::Context;
use chrono::{NaiveDate, NaiveDateTime};
pub async fn get_not_exported_completed_checkup(
    pool: &sqlx::MySqlPool,
    last_update: NaiveDateTime,
    from: NaiveDate,
    to: NaiveDate,
) -> sqlx::Result<Vec<String>> {
    let res = sqlx::query_file!(
        "./src/dto/get-not-exported-completed-checkup.sql",
        last_update,
        from,
        to
    )
    .fetch_all(pool)
    .await?;
    Ok(res.into_iter().map(|item| item.no_rawat).collect())
}

#[derive(Debug)]
pub enum ExportStatus {
    Success,
    NotComplete,
}

impl std::fmt::Display for ExportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExportStatus::Success => "success",
                ExportStatus::NotComplete => "not_complete",
            }
        )
    }
}

pub async fn set_checkup_exported(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
    update: chrono::NaiveDateTime,
    status: ExportStatus,
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query!(
        "insert into gmc_claim__docu_cache (no_rawat,value,status,last_update) values (?,?,?,?) on duplicate key update last_update=VALUES(last_update), status=VALUES(status)",
        no_rawat,
        "null",
        status.to_string(),
        update
    )
    .execute(&mut *tx).await?;

    sqlx::query!(
        "update reg_periksa set stts_cetak_sep = 'Sudah' where no_rawat = ?",
        no_rawat
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum IsMedicalCheckupExistsError {
    #[error("medical checkup is not using BPJS")]
    NotBpjs,
    #[error("medical checkup is already extracted at {}",.0.format("%d-%m-%Y"))]
    Exported(NaiveDateTime),
    #[error("medical checkup is not found")]
    NotFound,
    #[error(transparent)]
    Error(#[from] anyhow::Error),
}

pub async fn is_medical_checkup_exists(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> Result<(), IsMedicalCheckupExistsError> {
    let q = sqlx::query_file!("./src/dto/get-is-medical-checkup-exists.sql", no_rawat)
        .fetch_one(pool)
        .await
        .context("error when getting `is_medical_checkup_exists`")?;

    if q.exist == 0 {
        return Err(IsMedicalCheckupExistsError::NotFound);
    }

    if let Some(date) = q.exported {
        return Err(IsMedicalCheckupExistsError::Exported(date.naive_local()));
    }

    if q.is_bpjs != 1 {
        return Err(IsMedicalCheckupExistsError::NotBpjs);
    };

    Ok(())
}
