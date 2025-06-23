use std::fmt::Display;

use assesmen_awal_igd::{get_assesmen_awal_igd, AssesmenAwalIGD};
use assesmen_kebidanan::{get_assesmen_bidan, AssesmenKebidanan};
use berkas_digital::{get_berkas_digital, BerkasDigial};
use billing::{get_billing, Billing, GetBillingError};
use dpjp_ranap::{get_dpjp_ranap, DpjpRanap};
use futures::TryFutureExt;
use hasil_usg::{get_hasil_usg, HasilUsg};
use lab::{get_lab, Lab};
use laporan_operasi::{get_laporan_operasi, LaporanOperasi};
use operasi::{get_operasi, Operasi};
use radiologi::{get_pemeriksaan_radiologi, PemeriksaanRadiologi};
use reg_periksa::{get_reg_periksa, RegPeriksa};
use resume::{get_resume, Resume};
use sep::{get_sep, Sep};
use soap::{get_soap, Soap};
use spri::{get_spri, Spri};
use sqlx::MySqlPool;
use triase::{get_triase, Triase};

pub mod assesmen_awal_igd;
pub mod assesmen_kebidanan;
pub mod berkas_digital;
pub mod billing;
pub mod dpjp_ranap;
pub mod extra;
pub mod hasil_usg;
pub mod lab;
pub mod laporan_operasi;
pub mod operasi;
pub mod radiologi;
pub mod reg_periksa;
pub mod resume;
pub mod sep;
pub mod soap;
pub mod spri;
pub mod triase;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DetailRawat {
    pub reg_periksa: RegPeriksa,
    pub lab: Vec<Lab>,
    pub operasi: Vec<Operasi>,
    pub laporan_operasi: Vec<LaporanOperasi>,
    pub resume: Resume,
    pub radiologi: PemeriksaanRadiologi,
    pub sep: Sep,
    pub spri: Option<Spri>,
    pub triase: Triase,
    pub assesmen_kebidanan: AssesmenKebidanan,
    pub berkas_digital: Vec<BerkasDigial>,
    pub billing: Billing,
    pub hasil_usg: Option<HasilUsg>,
    pub dpjp_ranap: Vec<DpjpRanap>,
    pub soap: Vec<Soap>,
    pub assesmen_igd: Option<AssesmenAwalIGD>,
}

impl Display for DetailRawat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetRawatError {
    #[error(transparent)]
    Process(#[from] GetRawatProcessError),
    #[error(transparent)]
    Constraint(#[from] GetRawatConstraintError),
}

#[derive(thiserror::Error, Debug)]
pub enum GetRawatConstraintError {
    #[error("medical checkup's SEP not found")]
    SepNotFound,
    #[error("medical checkup is inpatient but it's SPRI not found")]
    SpriNotFound,
    #[error("medical checkup is ER but it's triage not found")]
    TriageNotFound,
}

#[derive(thiserror::Error, Debug)]
pub enum GetRawatProcessError {
    #[error("failed to fetch reg_periksa : {0}")]
    RegPeriksa(#[source] sqlx::Error),
    #[error("failed to fetch resume : {0}")]
    Resume(#[source] sqlx::Error),
    #[error("failed to fetch resume : {0}")]
    Lab(#[source] sqlx::Error),
    #[error("failed to fetch operasi : {0}")]
    Operasi(#[source] sqlx::Error),
    #[error("failed to fetch sep : {0}")]
    Sep(#[source] sqlx::Error),
    #[error("failed to fetch spri : {0}")]
    Spri(#[source] sqlx::Error),
    #[error("failed to fetch triase : {0}")]
    Triase(#[source] sqlx::Error),
    #[error("failed to fetch assesmen_kebidanan : {0}")]
    AssesmenBidan(#[source] sqlx::Error),
    #[error("failed to fetch berkas_digital : {0}")]
    BerkasDigital(#[source] sqlx::Error),
    #[error("failed to fetch laporan_operasi : {0}")]
    LaporanOperasi(#[source] sqlx::Error),
    #[error("failed to fetch billing : {0}")]
    Billing(#[source] GetBillingError),
    #[error("failed to fetch hasil_usg : {0}")]
    HasilUsg(#[source] sqlx::Error),
    #[error("failed to fetch pemeriksaan_radiologi : {0}")]
    PemeriksaanRadiologi(#[source] sqlx::Error),
    #[error("failed to fetch dpjp_ranap : {0}")]
    DpjpRanap(#[source] sqlx::Error),
    #[error("failed to fetch soap : {0}")]
    Soap(#[source] sqlx::Error),
    #[error("failed to fetch assesmen_awal_igd : {0}")]
    AssesmenAwalIgd(#[source] sqlx::Error),
}

pub async fn get_rawat(
    db: &MySqlPool,
    no_rawat: &str,
) -> Result<Option<DetailRawat>, GetRawatError> {
    let Some(periksa) = get_reg_periksa(db, no_rawat)
        .await
        .map_err(GetRawatProcessError::RegPeriksa)?
    else {
        return Ok(None);
    };

    let resume = get_resume(db, &periksa).map_err(GetRawatProcessError::Resume);
    let lab = get_lab(db, no_rawat).map_err(GetRawatProcessError::Lab);
    let operasi = get_operasi(db, no_rawat).map_err(GetRawatProcessError::Operasi);
    let sep = get_sep(db, &periksa).map_err(GetRawatProcessError::Sep);
    let spri = get_spri(db, no_rawat).map_err(GetRawatProcessError::Spri);
    let triase = get_triase(db, no_rawat).map_err(GetRawatProcessError::Triase);
    let assesmen_kebidanan =
        get_assesmen_bidan(db, &periksa).map_err(GetRawatProcessError::AssesmenBidan);
    let berkas_digital =
        get_berkas_digital(db, no_rawat).map_err(GetRawatProcessError::BerkasDigital);
    let laporan_operasi =
        get_laporan_operasi(db, no_rawat).map_err(GetRawatProcessError::LaporanOperasi);
    let radiologi =
        get_pemeriksaan_radiologi(db, no_rawat).map_err(GetRawatProcessError::PemeriksaanRadiologi);
    let billing = get_billing(db, &periksa).map_err(GetRawatProcessError::Billing);
    let usg = get_hasil_usg(db, no_rawat).map_err(GetRawatProcessError::HasilUsg);

    let dpjp_ranap = get_dpjp_ranap(db, no_rawat).map_err(GetRawatProcessError::DpjpRanap);
    let soap = get_soap(db, &periksa).map_err(GetRawatProcessError::Soap);
    let assesmen_igd =
        get_assesmen_awal_igd(db, &periksa).map_err(GetRawatProcessError::AssesmenAwalIgd);

    let (
        sep,
        spri,
        triase,
        assesmen_kebidanan,
        berkas_digital,
        lab,
        operasi,
        resume,
        laporan_operasi,
        radiologi,
        billing,
        usg,
        dpjp_ranap,
        soap,
        assesmen_igd,
    ) = tokio::try_join!(
        sep,
        spri,
        triase,
        assesmen_kebidanan,
        berkas_digital,
        lab,
        operasi,
        resume,
        laporan_operasi,
        radiologi,
        billing,
        usg,
        dpjp_ranap,
        soap,
        assesmen_igd
    )?;

    let Some(sep) = sep else {
        return Err(GetRawatConstraintError::SepNotFound.into());
    };

    if periksa.status_lanjut == "Ranap" && spri.is_none() {
        return Err(GetRawatConstraintError::SpriNotFound.into());
    }

    if (periksa.kode_poli == "IGDK" || periksa.kode_poli == "PNK") && triase.is_none() {
        return Err(GetRawatConstraintError::TriageNotFound.into());
    }

    let rawat = DetailRawat {
        reg_periksa: periksa,
        lab,
        operasi,
        resume,
        spri,
        sep,
        triase,
        assesmen_kebidanan,
        berkas_digital,
        laporan_operasi,
        radiologi,
        billing,
        hasil_usg: usg,
        dpjp_ranap,
        soap,
        assesmen_igd,
    };

    Ok(Some(rawat))
}
