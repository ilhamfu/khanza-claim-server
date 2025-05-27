use ranap::{get_resume_ranap, ResumeRanap};
use sqlx::MySqlPool;

use super::reg_periksa::RegPeriksa;

pub mod ranap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Resume {
    pub ranap: Option<ResumeRanap>,
}

pub async fn get_resume(db: &MySqlPool, reg_periksa: &RegPeriksa) -> sqlx::Result<Resume> {
    if reg_periksa.status_lanjut == "Ranap" {
        return Ok(Resume {
            ranap: get_resume_ranap(db, &reg_periksa.no_rawat).await?,
        });
    }

    Ok(Resume { ranap: None })
}
