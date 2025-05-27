use chrono::NaiveDate;
use sqlx::FromRow;

use super::reg_periksa::RegPeriksa;

#[derive(Debug, FromRow, Default, serde::Serialize, serde::Deserialize)]
pub struct Sep {
    pub no_sep: String,
    pub tanggal_sep: Option<NaiveDate>,
    pub no_rm: Option<String>,
    pub no_kartu: Option<String>,
    pub nama_pasien: Option<String>,
    pub tanggal_lahir: Option<NaiveDate>,
    pub jk: Option<String>,
    pub no_telp: String,
    pub nama_poli_tujuan: Option<String>,
    pub nama_dpjp: String,
    pub nama_ppk_rujukan: Option<String>,
    pub nama_diagnosa_awal: Option<String>,
    pub catatan: Option<String>,
    pub peserta: Option<String>,
    pub jenis_pelayanan: Option<String>,
    pub tujuan_kunjungan: String,
    pub flag_prosedur: String,
    pub kelas_rawat: Option<String>,
    pub kelas_naik: String,
    pub lakalantas: Option<String>,
}

pub async fn get_sep(
    db: &sqlx::MySqlPool,
    no_rawat: &RegPeriksa,
) -> Result<Option<Sep>, sqlx::Error> {
    let is_ranap = if no_rawat.status_lanjut == "Ranap" {
        "1"
    } else {
        "2"
    };
    sqlx::query_file_as!(Sep, "./src/dto/get-sep.sql", &no_rawat.no_rawat, is_ranap)
        .fetch_optional(db)
        .await
}
