use ralan::get_billing_ralan;
use ranap::get_billing_ranap;

use super::reg_periksa::RegPeriksa;

mod ralan;
mod ranap;
mod util;

pub use ralan::BillingRalan;
pub use ranap::BillingRanap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BillingFormat {
    pub detail: String,
    pub price: Option<f64>,
    pub qty: Option<f64>,
    pub extra: Option<f64>,
    pub total: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Billing {
    Ralan(ralan::BillingRalan),
    Ranap(ranap::BillingRanap),
}

#[derive(Debug, thiserror::Error)]
#[error("error when getting billing : {kind}")]
pub struct GetBillingError {
    pub source: sqlx::Error,
    pub kind: GetBillingErrorKind,
}

#[derive(Debug, derive_more::Display)]
pub enum GetBillingErrorKind {
    #[display("biaya registrasi")]
    BiayaReg,
    #[display("pemeriksaan perawat")]
    PemeriksaanPerawat,
    #[display("pemeriksaan dokter")]
    PemeriksaanDokter,
    #[display("pemeriksaan dokter & perawat")]
    PemeriksaanDokterPerawat,
    #[display("pemeriksaan perawat ranap")]
    PemeriksaanPerawatRanap,
    #[display("pemeriksaan dokter ranap")]
    PemeriksaanDokterRanap,
    #[display("pemeriksaan dokter & perawat ranap")]
    PemeriksaanDokterPerawatRanap,
    #[display("biaya kamar")]
    BiayaKamar,
    #[display("radiologi")]
    Radiologi,
    #[display("lab")]
    Lab,
    #[display("operasi")]
    Operasi,
    #[display("obat operasi")]
    ObatOperasi,
    #[display("obat")]
    Obat,
    #[display("retur obat")]
    ReturObat,
}

pub async fn get_billing(
    pool: &sqlx::MySqlPool,
    reg_periksa: &RegPeriksa,
) -> Result<Billing, GetBillingError> {
    if reg_periksa.status_lanjut == "Ranap" {
        return Ok(Billing::Ranap(
            get_billing_ranap(pool, &reg_periksa.no_rawat).await?,
        ));
    }

    Ok(Billing::Ralan(
        get_billing_ralan(pool, &reg_periksa.no_rawat).await?,
    ))
}
