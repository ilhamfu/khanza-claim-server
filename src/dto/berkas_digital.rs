use sqlx::MySqlPool;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct BerkasDigial {
    pub kode: String,
    pub kategori: Option<String>,
    pub lokasi_file: Option<String>,
}

pub async fn get_berkas_digital(db: &MySqlPool, no_rawat: &str) -> sqlx::Result<Vec<BerkasDigial>> {
    sqlx::query_file_as!(BerkasDigial, "./src/dto/get-berkas-digital.sql", no_rawat)
        .fetch_all(db)
        .await
}
