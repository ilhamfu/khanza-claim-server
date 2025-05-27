use sqlx::MySqlPool;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct RegPeriksa {
    pub no_rawat: String,
    pub tgl_registrasi: chrono::NaiveDate,
    pub jam_registrasi: chrono::NaiveTime,
    pub no_rm: String,
    pub nama_pasien: String,
    pub jk: String,
    pub tanggal_lahir: chrono::NaiveDate,
    pub umur_daftar: i32,
    pub status_umur: String,
    pub kode_dokter: String,
    pub nama_dokter: String,
    pub kode_poli: String,
    pub nama_poli: String,
    pub kode_pj: String,
    pub nama_pj: String,
    pub status_lanjut: String,
    pub status: String,
}

pub async fn get_reg_periksa(db: &MySqlPool, no_rawat: &str) -> sqlx::Result<Option<RegPeriksa>> {
    sqlx::query_file_as!(RegPeriksa, "./src/dto/get-reg-periksa.sql", no_rawat)
        .fetch_optional(db)
        .await
}
