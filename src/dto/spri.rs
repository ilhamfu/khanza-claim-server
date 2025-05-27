use sqlx::FromRow;
#[derive(Debug, FromRow, Default, serde::Serialize, serde::Deserialize)]
pub struct Spri {
    pub kode_dokter: Option<String>,
    pub nama_dokter: Option<String>,
    pub nama_poli: Option<String>,
    pub no_kartu: Option<String>,
    pub nama_pasien: Option<String>,
    pub tanggal_lahir: Option<chrono::NaiveDate>,
    pub diagnosa: String,
    pub tanggal_surat: chrono::NaiveDate,
    pub no_surat: Option<String>,
    pub tanggal_rencana: Option<chrono::NaiveDate>,
    pub jenis_kel: Option<String>,
}

pub async fn get_spri(db: &sqlx::MySqlPool, no_rawat: &str) -> sqlx::Result<Option<Spri>> {
    sqlx::query_file_as!(Spri, "./src/dto/get-spri.sql", no_rawat)
        .fetch_optional(db)
        .await
}
