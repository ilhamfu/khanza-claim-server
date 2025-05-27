use sqlx::types::Json;

use super::PemeriksaanTriase;

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct BaseTriaseSekunder {
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
    pub anamnesa_singkat: Option<String>,
    pub catatan: Option<String>,
    pub plan: Option<String>,
    pub tanggal_triase: Option<chrono::NaiveDateTime>,
    pub nik: Option<String>,
    pub kode_kasus: Option<String>,
    pub macam_kasus: Option<String>,
    pub nama_pegawai: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PemeriksaanTriaseSekunder {
    pub skala3: Option<Vec<PemeriksaanTriase>>,
    pub skala4: Option<Vec<PemeriksaanTriase>>,
    pub skala5: Option<Vec<PemeriksaanTriase>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TriaseSekunder {
    #[serde(flatten)]
    base: BaseTriaseSekunder,
    pub pemeriksaan: PemeriksaanTriaseSekunder,
}

impl std::ops::Deref for TriaseSekunder {
    type Target = BaseTriaseSekunder;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

struct TempPemeriksaanTriaseSekunder {
    pemeriksaan: Option<Json<PemeriksaanTriaseSekunder>>,
}

pub async fn get_triase_sekunder(
    db: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Option<TriaseSekunder>> {
    let Some(base) = sqlx::query_file_as!(
        BaseTriaseSekunder,
        "./src/dto/triase/get-triase-sekunder.sql",
        no_rawat
    )
    .fetch_optional(db)
    .await?
    else {
        return Ok(None);
    };

    let Some(TempPemeriksaanTriaseSekunder {
        pemeriksaan: Some(Json(pemeriksaan)),
    }) = sqlx::query_file_as!(
        TempPemeriksaanTriaseSekunder,
        "./src/dto/triase/get-triase-sekunder-skala.sql",
        no_rawat,
        no_rawat,
        no_rawat
    )
    .fetch_optional(db)
    .await?
    else {
        return Ok(None);
    };

    Ok(Some(TriaseSekunder { base, pemeriksaan }))
}
