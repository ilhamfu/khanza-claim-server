use chrono::{NaiveDate, NaiveTime};

use super::reg_periksa;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Soap {
    pub no_rawat: String,
    pub tgl_perawatan: NaiveDate,
    pub jam_rawat: NaiveTime,
    pub suhu_tubuh: String,
    pub tensi: String,
    pub nadi: String,
    pub respirasi: String,
    pub tinggi: String,
    pub berat: String,
    pub spo2: String,
    pub gcs: String,
    pub kesadaran: String,
    pub keluhan: String,
    pub pemeriksaan: String,
    pub alergi: String,
    pub lingkar_perut: String,
    pub rtl: String,
    pub penilaian: String,
    pub instruksi: String,
    pub evaluasi: String,
    pub kode_pegawai: String,
    pub nama_pegawai: String,
}

pub async fn get_soap(
    db: &sqlx::MySqlPool,
    reg_periksa: &reg_periksa::RegPeriksa,
) -> sqlx::Result<Vec<Soap>> {
    if reg_periksa.status_lanjut == "Ranap" {
        return Ok(Default::default());
    }

    sqlx::query_file_as!(Soap, "./src/dto/get-soap-ralan.sql", reg_periksa.no_rawat)
        .fetch_all(db)
        .await
}
