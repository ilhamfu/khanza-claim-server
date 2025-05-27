use futures::TryFutureExt;
use sqlx::types::Json;

use crate::dto::billing::{
    util::{
        get_biaya_reg, get_lab, get_obat_bhp, get_obat_bhp_operasi, get_operasi,
        get_pemeriksaan_dokter, get_pemeriksaan_dokter_perawat, get_pemeriksaan_perawat,
        get_radiologi, TempPerawatan,
    },
    GetBillingErrorKind,
};

use super::{BillingFormat, GetBillingError};

async fn get_pemeriksaan_dokter_ranap(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempPerawatan>> {
    sqlx::query_file_as!(
        TempPerawatan,
        "./src/dto/billing/ranap/get-biaya-tindakan-dr-ranap.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

async fn get_pemeriksaan_perawat_ranap(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempPerawatan>> {
    sqlx::query_file_as!(
        TempPerawatan,
        "./src/dto/billing/ranap/get-biaya-tindakan-pr-ranap.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

async fn get_pemeriksaan_dokter_perawat_ranap(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempPerawatan>> {
    sqlx::query_file_as!(
        TempPerawatan,
        "./src/dto/billing/ranap/get-biaya-tindakan-drpr-ranap.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

struct TempReturObat {
    nama_barang: String,
    harga: f64,
    qty: f64,
    total: f64,
}

impl From<TempReturObat> for BillingFormat {
    fn from(value: TempReturObat) -> Self {
        Self {
            detail: value.nama_barang,
            price: Some(value.harga),
            qty: Some(value.qty),
            extra: None,
            total: value.total,
        }
    }
}

async fn get_retur_obat(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempReturObat>> {
    sqlx::query_file_as!(
        TempReturObat,
        "./src/dto/billing/ranap/get-retur-obat-ranap.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TempBiayaSekali {
    nama: String,
    biaya: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct TempBiayaHarian {
    nama: String,
    biaya: f64,
    jumlah: f64,
}

#[derive(sqlx::FromRow)]
struct TempBiayaKamar {
    pub kode_kamar: String,
    pub nama_bangsal: String,
    pub tarif_kamar: f64,
    pub lama: f64,
    pub total: f64,
    #[sqlx(default)]
    pub bs: Json<Vec<TempBiayaSekali>>,
    #[sqlx(default)]
    pub bh: Json<Vec<TempBiayaHarian>>,
}

impl From<TempBiayaKamar> for (BillingFormat, Vec<BillingFormat>, Vec<BillingFormat>) {
    fn from(value: TempBiayaKamar) -> Self {
        (
            BillingFormat {
                detail: format!("{}, {}", value.kode_kamar, value.nama_bangsal),
                price: Some(value.tarif_kamar),
                qty: Some(value.lama),
                extra: None,
                total: value.total,
            },
            value
                .bs
                .0
                .into_iter()
                .map(|item| BillingFormat {
                    detail: item.nama,
                    price: Some(item.biaya),
                    qty: None,
                    extra: None,
                    total: item.biaya,
                })
                .collect(),
            value
                .bh
                .0
                .into_iter()
                .map(|item| BillingFormat {
                    detail: item.nama,
                    price: Some(item.biaya),
                    qty: Some(value.lama),
                    extra: None,
                    total: item.biaya * value.lama,
                })
                .collect(),
        )
    }
}

async fn get_biaya_kamar(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Vec<TempBiayaKamar>> {
    sqlx::query_file_as!(
        TempBiayaKamar,
        "./src/dto/billing/ranap/get-biaya-kamar.sql",
        no_rawat
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BillingRanap {
    pub reg: BillingFormat,
    pub kamar: Vec<(BillingFormat, Vec<BillingFormat>, Vec<BillingFormat>)>,
    pub pem_ralan: Vec<BillingFormat>,
    pub pem_ranap: Vec<BillingFormat>,
    pub rad: Vec<BillingFormat>,
    pub lab: Vec<BillingFormat>,
    pub operasi: Vec<BillingFormat>,
    pub obat: Vec<BillingFormat>,
    pub obat_op: Vec<BillingFormat>,
    pub retur_obat: Vec<BillingFormat>,
}

impl BillingRanap {
    pub fn sum(&self) -> f64 {
        let mut res = 0f64;
        res += self.reg.total;
        res = self.kamar.iter().fold(res, |acc, pre| {
            acc + pre.0.total
                + pre.1.iter().fold(0f64, |a, b| a + b.total)
                + pre.2.iter().fold(0f64, |a, b| a + b.total)
        });
        res = self.pem_ralan.iter().fold(res, |acc, val| acc + val.total);
        res = self.pem_ranap.iter().fold(res, |acc, val| acc + val.total);
        res = self.rad.iter().fold(res, |acc, val| acc + val.total);
        res = self.lab.iter().fold(res, |acc, val| acc + val.total);
        res = self.operasi.iter().fold(res, |acc, val| acc + val.total);
        res = self.obat.iter().fold(res, |acc, val| acc + val.total);
        res = self.retur_obat.iter().fold(res, |acc, val| acc + val.total);

        self.obat_op.iter().fold(res, |acc, val| acc + val.total)
    }
}

pub async fn get_billing_ranap(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> Result<BillingRanap, GetBillingError> {
    let (
        biaya,
        pr_ralan,
        dr_ralan,
        prdr_ralan,
        pr_ranap,
        dr_ranap,
        prdr_ranap,
        kamar,
        radioloig,
        lab,
        operasi,
        obat_bhp_op,
        obat_bhp,
        retur_obat,
    ) = tokio::try_join!(
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
        get_pemeriksaan_perawat_ranap(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::PemeriksaanPerawatRanap
        }),
        get_pemeriksaan_dokter_ranap(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::PemeriksaanDokterRanap
        }),
        get_pemeriksaan_dokter_perawat_ranap(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::PemeriksaanDokterPerawatRanap
        }),
        get_biaya_kamar(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::BiayaKamar
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
        get_retur_obat(pool, no_rawat).map_err(|err| GetBillingError {
            source: err,
            kind: GetBillingErrorKind::ReturObat
        }),
    )?;

    Ok(BillingRanap {
        reg: biaya.into(),
        kamar: kamar.into_iter().map(Into::into).collect(),
        pem_ralan: pr_ralan
            .into_iter()
            .map(Into::into)
            .chain(dr_ralan.into_iter().map(Into::into))
            .chain(prdr_ralan.into_iter().map(Into::into))
            .collect(),
        pem_ranap: pr_ranap
            .into_iter()
            .map(Into::into)
            .chain(dr_ranap.into_iter().map(Into::into))
            .chain(prdr_ranap.into_iter().map(Into::into))
            .collect(),
        rad: radioloig.into_iter().map(Into::into).collect(),
        lab: lab.into_iter().map(Into::into).collect(),
        operasi: operasi.into_iter().map(Into::into).collect(),
        obat: obat_bhp.into_iter().map(Into::into).collect(),
        obat_op: obat_bhp_op.into_iter().map(Into::into).collect(),
        retur_obat: retur_obat.into_iter().map(Into::into).collect(),
    })
}
