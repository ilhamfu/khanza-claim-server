use sqlx::MySqlPool;
use triase_sekunder::{get_triase_sekunder, TriaseSekunder};
use triase_utama::{get_triase_utama, TriaseUtama};

pub mod triase_sekunder;
pub mod triase_utama;
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PemeriksaanTriase {
    pub nama_pemeriksaan: String,
    pub pengkajian: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Triase {
    pub primer: Option<TriaseUtama>,
    pub sekunder: Option<TriaseSekunder>,
}

impl Triase {
    pub fn is_none(&self) -> bool {
        self.primer.is_none() && self.sekunder.is_none()
    }
}

pub async fn get_triase(db: &MySqlPool, no_rawat: &str) -> sqlx::Result<Triase> {
    Ok(Triase {
        primer: get_triase_utama(db, no_rawat).await?,
        sekunder: get_triase_sekunder(db, no_rawat).await?,
    })
}
