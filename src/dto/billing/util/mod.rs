use super::BillingFormat;

pub struct TempBiayaReg {
    biaya_reg: f64,
}

pub async fn get_biaya_reg(pool: &sqlx::MySqlPool, no_rawat: &str) -> sqlx::Result<TempBiayaReg> {
    sqlx::query_file_as!(
        TempBiayaReg,
        "./src/dto/billing/util/get-biaya-reg.sql",
        no_rawat
    )
    .fetch_one(pool)
    .await
}

pub struct TempPerawatan {
    pub nama_perawatan: String,
    pub biaya: f64,
    pub qty: i64,
    pub total: f64,
}

pub async fn get_pemeriksaan_dokter(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempPerawatan>> {
    sqlx::query_file_as!(
        TempPerawatan,
        "./src/dto/billing/util/get-pr-dokter.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

pub async fn get_pemeriksaan_perawat(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempPerawatan>> {
    sqlx::query_file_as!(
        TempPerawatan,
        "./src/dto/billing/util/get-pr-perawat.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

pub async fn get_pemeriksaan_dokter_perawat(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempPerawatan>> {
    sqlx::query_file_as!(
        TempPerawatan,
        "./src/dto/billing/util/get-pr-dokter-perawat.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

pub struct TempBiayaLab {
    nama_perawatan: String,
    qty: i64,
    biaya: f64,
    biaya_tambahan: f64,
}

pub async fn get_lab(pool: &sqlx::MySqlPool, no_rawat: &str) -> sqlx::Result<Vec<TempBiayaLab>> {
    sqlx::query_file_as!(
        TempBiayaLab,
        "./src/dto/billing/util/get-biaya-lab.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

pub struct TempOperasi {
    nama_perawatan: String,
    biaya: f64,
}

pub async fn get_operasi(pool: &sqlx::MySqlPool, no_rawat: &str) -> sqlx::Result<Vec<TempOperasi>> {
    sqlx::query_file_as!(
        TempOperasi,
        "./src/dto/billing/util/get-biaya-operasi.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

pub struct TempBiayaRadiologi {
    nama_perawatan: String,
    qty: i64,
    biaya: f64,
    total: f64,
}

pub async fn get_radiologi(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempBiayaRadiologi>> {
    sqlx::query_file_as!(
        TempBiayaRadiologi,
        "./src/dto/billing/util/get-biaya-radiologi.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

pub struct TempObatBhpOperasi {
    nama_obat: String,
    harga: f64,
    qty: f64,
    total: f64,
}

pub async fn get_obat_bhp_operasi(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempObatBhpOperasi>> {
    sqlx::query_file_as!(
        TempObatBhpOperasi,
        "./src/dto/billing/util/get-bhp-operasi.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

pub struct TempObatBhp {
    nama_obat: String,
    jenis: Option<String>,
    harga: f64,
    qty: f64,
    tambahan: f64,
    total: f64,
}

pub async fn get_obat_bhp(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempObatBhp>> {
    sqlx::query_file_as!(TempObatBhp, "./src/dto/billing/util/get-bhp.sql", no_rawat)
        .fetch_all(pool)
        .await
}

impl From<TempPerawatan> for BillingFormat {
    fn from(value: TempPerawatan) -> Self {
        Self {
            detail: value.nama_perawatan,
            price: Some(value.biaya),
            qty: Some(value.qty as f64),
            extra: None,
            total: value.total,
        }
    }
}

impl From<TempBiayaReg> for BillingFormat {
    fn from(value: TempBiayaReg) -> Self {
        Self {
            detail: "Administrasi Rekam Medik".to_owned(),
            price: None,
            qty: None,
            extra: None,
            total: value.biaya_reg,
        }
    }
}

impl From<TempBiayaRadiologi> for BillingFormat {
    fn from(value: TempBiayaRadiologi) -> Self {
        Self {
            detail: value.nama_perawatan,
            price: Some(value.biaya),
            qty: Some(value.qty as f64),
            extra: None,
            total: value.total,
        }
    }
}

impl From<TempBiayaLab> for BillingFormat {
    fn from(value: TempBiayaLab) -> Self {
        Self {
            detail: value.nama_perawatan,
            price: Some(value.biaya),
            qty: Some(value.qty as f64),
            extra: Some(value.biaya_tambahan),
            total: (value.biaya * value.qty as f64) + value.biaya_tambahan,
        }
    }
}

impl From<TempOperasi> for BillingFormat {
    fn from(value: TempOperasi) -> Self {
        Self {
            detail: value.nama_perawatan,
            price: None,
            qty: None,
            extra: None,
            total: value.biaya,
        }
    }
}

impl From<TempObatBhpOperasi> for BillingFormat {
    fn from(value: TempObatBhpOperasi) -> Self {
        Self {
            detail: value.nama_obat,
            price: Some(value.harga),
            qty: Some(value.qty),
            extra: None,
            total: value.total,
        }
    }
}
impl From<TempObatBhp> for BillingFormat {
    fn from(value: TempObatBhp) -> Self {
        Self {
            detail: format!(
                "{}{}",
                value.nama_obat,
                value
                    .jenis
                    .map(|item| format!(" ({})", item))
                    .unwrap_or_default()
            ),
            price: Some(value.harga),
            qty: Some(value.qty),
            extra: Some(value.tambahan),
            total: value.total,
        }
    }
}
