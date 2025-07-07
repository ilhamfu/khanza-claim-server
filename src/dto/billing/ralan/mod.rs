use futures::TryFutureExt;

use crate::dto::billing::GetBillingError;
use crate::dto::billing::GetBillingErrorKind;

use super::util::*;
use super::BillingFormat;
use super::Total;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BillingRalan {
    pub reg: BillingFormat,
    pub pem: Vec<BillingFormat>,
    pub rad: Vec<BillingFormat>,
    pub lab: Vec<BillingFormat>,
    pub operasi: Vec<BillingFormat>,
    pub obat: Vec<BillingFormat>,
    pub obat_op: Vec<BillingFormat>,
}

impl BillingRalan {
    pub fn sum(&self) -> f64 {
        let mut res = 0f64;
        res += self.reg.total;
        res += self.pem.subtotal();
        res += self.rad.subtotal();
        res += self.lab.subtotal();
        res += self.operasi.subtotal();
        res += self.obat.subtotal();

        res + self.obat_op.subtotal()
    }
}

pub async fn get_billing_ralan(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> Result<BillingRalan, GetBillingError> {
    let (biaya, pr, dr, prdr, radioloig, lab, operasi, obat_bhp_op, obat_bhp) = tokio::try_join!(
        get_biaya_reg(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::BiayaReg
        }),
        get_pemeriksaan_perawat(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::PemeriksaanPerawat
        }),
        get_pemeriksaan_dokter(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::PemeriksaanDokter
        }),
        get_pemeriksaan_dokter_perawat(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::PemeriksaanDokterPerawat
        }),
        get_radiologi(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::Radiologi
        }),
        get_lab(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::Lab
        }),
        get_operasi(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::Operasi
        }),
        get_obat_bhp_operasi(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::ObatOperasi
        }),
        get_obat_bhp(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::Obat
        }),
    )?;

    Ok(BillingRalan {
        reg: biaya.into(),
        pem: pr
            .into_iter()
            .map(Into::into)
            .chain(dr.into_iter().map(Into::into))
            .chain(prdr.into_iter().map(Into::into))
            .collect(),
        rad: radioloig.into_iter().map(Into::into).collect(),
        lab: lab.into_iter().map(Into::into).collect(),
        operasi: operasi.into_iter().map(Into::into).collect(),
        obat: obat_bhp.into_iter().map(Into::into).collect(),
        obat_op: obat_bhp_op.into_iter().map(Into::into).collect(),
    })
}
