use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, MySqlPool};

#[derive(Debug, Deserialize, Serialize)]
pub struct PemeriksaanRadiologi {
    pub pemeriksaan: Vec<BasePemeriksaanRadiologi>,
    pub hasil: Vec<HasilPemeriksaanRadiologi>,
    pub gambar: Vec<GambarRadiologi>,
}

impl PemeriksaanRadiologi {
    pub fn is_empty(&self) -> bool {
        self.pemeriksaan.is_empty() && self.hasil.is_empty() && self.hasil.is_empty()
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct GambarRadiologi {
    pub tanggal_periksa: chrono::NaiveDate,
    pub jam_periksa: chrono::NaiveTime,
    pub lokasi_gambar: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct HasilPemeriksaanRadiologi {
    pub tanggal_periksa: chrono::NaiveDate,
    pub jam_periksa: chrono::NaiveTime,
    pub hasil: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BasePemeriksaanRadiologi {
    pub no_rawat: String,
    pub kode_petugas: String,
    pub nama_petugas: Option<String>,
    pub kode_perawatan: String,
    pub nama_perawatan: Option<String>,
    pub tanggal_periksa: chrono::NaiveDate,
    pub jam_periksa: chrono::NaiveTime,
    pub kode_dokter: String,
    pub nama_dokter: Option<String>,
    pub status: Option<String>,
    pub proyeksi: String,
    pub kv: String,
    pub mas: String,
    pub ffd: String,
    pub bfs: String,
    pub inak: String,
    pub jumlah_penyinaran: String,
    pub dosis: String,
}

pub async fn get_pemeriksaan_radiologi(
    db: &MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<PemeriksaanRadiologi> {
    let pemeriksaan = sqlx::query_file_as!(
        BasePemeriksaanRadiologi,
        "./src/dto/get-pemeriksaan-radiologi.sql",
        no_rawat
    )
    .fetch_all(db)
    .await?;
    if pemeriksaan.is_empty() {
        return Ok(PemeriksaanRadiologi {
            pemeriksaan,
            hasil: Vec::default(),
            gambar: Vec::default(),
        });
    };

    let hasil = sqlx::query_file_as!(
        HasilPemeriksaanRadiologi,
        "./src/dto/get-hasil-radiologi.sql",
        no_rawat
    )
    .fetch_all(db)
    .await?;

    let gambar = sqlx::query_file_as!(
        GambarRadiologi,
        "./src/dto/get-gambar-radiologi.sql",
        no_rawat
    )
    .fetch_all(db)
    .await?;
    Ok(PemeriksaanRadiologi {
        pemeriksaan,
        hasil,
        gambar,
    })
}
