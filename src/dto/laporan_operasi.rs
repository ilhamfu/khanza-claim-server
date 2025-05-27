#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct LaporanOperasi {
    pub tanggal: Option<chrono::NaiveDateTime>,
    pub diagnosa_preop: Option<String>,
    pub diagnosa_postop: Option<String>,
    pub jaringan_dieksekusi: Option<String>,
    pub selesai_operasi: Option<chrono::NaiveDateTime>,
    pub permintaan_pa: Option<String>,
    pub laporan_operasi: Option<String>,
}

pub async fn get_laporan_operasi(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<LaporanOperasi>> {
    sqlx::query_file_as!(
        LaporanOperasi,
        "./src/dto/get-laporan-operasi.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}
