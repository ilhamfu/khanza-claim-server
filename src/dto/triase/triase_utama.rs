use sqlx::types::Json;

use super::PemeriksaanTriase;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct BaseTriaseUtama {
    pub tekanan_darah: Option<String>,
    pub nadi: Option<String>,
    pub pernapasan: Option<String>,
    pub suhu: Option<String>,
    pub saturasi_o2: Option<String>,
    pub nyeri: Option<String>,
    pub cara_masuk: Option<String>,
    pub alat_transportasi: Option<String>,
    pub alasan_kedatangan: Option<String>,
    pub keterangan_kedatangan: Option<String>,
    pub keluhan_utama: Option<String>,
    pub kebutuhan_khusus: Option<String>,
    pub catatan: Option<String>,
    pub plan: Option<String>,
    pub tanggal_triase: Option<chrono::NaiveDateTime>,
    pub nik: Option<String>,
    pub kode_kasus: Option<String>,
    pub macam_kasus: Option<String>,
    pub nama_pegawai: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PemeriksaanTriaseUtama {
    pub skala1: Option<Vec<PemeriksaanTriase>>,
    pub skala2: Option<Vec<PemeriksaanTriase>>,
}

struct TempPemeriksaanTriaseUtama {
    pemeriksaan: Option<Json<PemeriksaanTriaseUtama>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TriaseUtama {
    #[serde(flatten)]
    pub base: BaseTriaseUtama,
    pub pemeriksaan: PemeriksaanTriaseUtama,
}

impl std::ops::Deref for TriaseUtama {
    type Target = BaseTriaseUtama;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

pub async fn get_triase_utama(
    db: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Option<TriaseUtama>> {
    let Some(base) = sqlx::query_file_as!(
        BaseTriaseUtama,
        "./src/dto/triase/get-triase-utama.sql",
        no_rawat
    )
    .fetch_optional(db)
    .await?
    else {
        return Ok(None);
    };

    let Some(TempPemeriksaanTriaseUtama {
        pemeriksaan: Some(Json(pemeriksaan)),
    }) = sqlx::query_file_as!(
        TempPemeriksaanTriaseUtama,
        "./src/dto/triase/get-triase-utama-skala.sql",
        no_rawat,
        no_rawat
    )
    .fetch_optional(db)
    .await?
    else {
        return Ok(None);
    };
    Ok(Some(TriaseUtama { base, pemeriksaan }))
}
