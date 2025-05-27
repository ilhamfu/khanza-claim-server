use chrono::{NaiveDate, NaiveTime};
use sqlx::{types::Json, MySqlPool};

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Lab {
    pub tanggal_periksa: NaiveDate,
    pub jam: NaiveTime,
    pub no_rawat: String,
    pub nama_perawatan: Option<String>,
    pub nama_petugas: Option<String>,
    pub nama_dokter: Option<String>,
    pub detail: Option<Json<Vec<DetailLab>>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DetailLab {
    pub pemeriksaan: String,
    pub nilai: String,
    pub satuan: String,
    pub nilai_rujukan: String,
    pub keterangan: String,
}

pub async fn get_lab(db: &MySqlPool, no_rawat: &str) -> sqlx::Result<Vec<Lab>> {
    sqlx::query_file_as!(Lab, "./src/dto/get-lab.sql", no_rawat)
        .fetch_all(db)
        .await
}
