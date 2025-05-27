use ralan::{get_assesmen_kebidan_ralan, AssesmenKebidananRalan};
use ranap::{get_assesmen_kebidanan_ranap, AssesmenKebidananRanap};
use sqlx::MySqlPool;

use super::reg_periksa::RegPeriksa;

pub mod ralan;
pub mod ranap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RiwayatPersalinan {
    pub tanggal_tahun: String,
    pub tempat_persalinan: String,
    pub usia_hamil: String,
    pub jenis_persalinan: String,
    pub penolong: String,
    pub penyulit: String,
    pub jk: String,
    pub bbpb: String,
    pub keadaan: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AssesmenKebidanan {
    pub ranap: Option<AssesmenKebidananRanap>,
    pub ralan: Option<AssesmenKebidananRalan>,
}

pub async fn get_assesmen_bidan(
    db: &MySqlPool,
    reg_periksa: &RegPeriksa,
) -> sqlx::Result<AssesmenKebidanan> {
    Ok(AssesmenKebidanan {
        ranap: get_assesmen_kebidanan_ranap(db, &reg_periksa.no_rawat).await?,
        ralan: get_assesmen_kebidan_ralan(db, &reg_periksa.no_rawat).await?,
    })
}
