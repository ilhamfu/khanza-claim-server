#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DpjpRanap {
    pub kode_dokter: String,
    pub nama_dokter: String,
}
pub async fn get_dpjp_ranap(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<DpjpRanap>> {
    sqlx::query_file_as!(DpjpRanap, "./src/dto/get-dpjp-ranap.sql", no_rawat)
        .fetch_all(pool)
        .await
}
