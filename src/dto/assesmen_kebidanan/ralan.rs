use chrono::NaiveDate;
use sqlx::MySqlPool;

use super::RiwayatPersalinan;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct AssesmenKebidananRalan {
    pub tanggal: chrono::NaiveDateTime,
    pub informasi: Option<String>,
    pub td: Option<String>,
    pub nadi: Option<String>,
    pub rr: Option<String>,
    pub suhu: Option<String>,
    pub bb: Option<String>,
    pub gcs: Option<String>,
    pub tb: Option<String>,
    pub bmi: Option<String>,
    pub lila: Option<String>,
    pub tfu: Option<String>,
    pub tbj: Option<String>,
    pub letak: Option<String>,
    pub presentasi: Option<String>,
    pub penurunan: Option<String>,
    pub his: Option<String>,
    pub kekuatan: Option<String>,
    pub lamanya: Option<String>,
    pub bjj: Option<String>,
    pub ket_bjj: Option<String>,
    pub portio: Option<String>,
    pub serviks: Option<String>,
    pub ketuban: Option<String>,
    pub hodge: Option<String>,
    pub inspekulo: Option<String>,
    pub ket_inspekulo: Option<String>,
    pub ctg: Option<String>,
    pub ket_ctg: Option<String>,
    pub usg: Option<String>,
    pub ket_usg: Option<String>,
    pub lab: Option<String>,
    pub ket_lab: Option<String>,
    pub lakmus: Option<String>,
    pub ket_lakmus: Option<String>,
    pub panggul: Option<String>,
    pub keluhan_utama: Option<String>,
    pub umur: Option<String>,
    pub lama: Option<String>,
    pub banyaknya: Option<String>,
    pub haid: Option<String>,
    pub siklus: Option<String>,
    pub ket_siklus: Option<String>,
    pub ket_siklus1: Option<String>,
    pub status: Option<String>,
    pub kali: Option<String>,
    pub usia1: Option<String>,
    pub ket1: Option<String>,
    pub usia2: Option<String>,
    pub ket2: Option<String>,
    pub usia3: Option<String>,
    pub ket3: Option<String>,
    pub hpht: Option<NaiveDate>,
    pub usia_kehamilan: Option<String>,
    pub tp: Option<NaiveDate>,
    pub imunisasi: Option<String>,
    pub ket_imunisasi: Option<String>,
    pub g: Option<String>,
    pub p: Option<String>,
    pub a: Option<String>,
    pub hidup: Option<String>,
    pub ginekologi: Option<String>,
    pub kebiasaan: Option<String>,
    pub ket_kebiasaan: Option<String>,
    pub kebiasaan1: Option<String>,
    pub ket_kebiasaan1: Option<String>,
    pub kebiasaan2: Option<String>,
    pub ket_kebiasaan2: Option<String>,
    pub kebiasaan3: Option<String>,
    pub kb: Option<String>,
    pub ket_kb: Option<String>,
    pub komplikasi: Option<String>,
    pub ket_komplikasi: Option<String>,
    pub berhenti: Option<String>,
    pub alasan: Option<String>,
    pub alat_bantu: Option<String>,
    pub ket_bantu: Option<String>,
    pub prothesa: Option<String>,
    pub ket_pro: Option<String>,
    pub adl: Option<String>,
    pub status_psiko: Option<String>,
    pub ket_psiko: Option<String>,
    pub hub_keluarga: Option<String>,
    pub tinggal_dengan: Option<String>,
    pub ket_tinggal: Option<String>,
    pub ekonomi: Option<String>,
    pub budaya: Option<String>,
    pub ket_budaya: Option<String>,
    pub edukasi: Option<String>,
    pub ket_edukasi: Option<String>,
    pub berjalan_a: Option<String>,
    pub berjalan_b: Option<String>,
    pub berjalan_c: Option<String>,
    pub hasil: Option<String>,
    pub lapor: Option<String>,
    pub ket_lapor: Option<String>,
    pub sg1: Option<String>,
    pub nilai1: Option<String>,
    pub sg2: Option<String>,
    pub nilai2: Option<String>,
    pub total_hasil: Option<String>,
    pub nyeri: Option<String>,
    pub provokes: Option<String>,
    pub ket_provokes: Option<String>,
    pub quality: Option<String>,
    pub ket_quality: Option<String>,
    pub lokasi: Option<String>,
    pub menyebar: Option<String>,
    pub skala_nyeri: Option<String>,
    pub durasi: Option<String>,
    pub nyeri_hilang: Option<String>,
    pub ket_nyeri: Option<String>,
    pub pada_dokter: Option<String>,
    pub ket_dokter: Option<String>,
    pub masalah: Option<String>,
    pub tindakan: Option<String>,
    pub nip: Option<String>,
    pub nama: Option<String>,
    pub riwayat_persalinan: Option<sqlx::types::Json<Vec<RiwayatPersalinan>>>,
}

pub async fn get_assesmen_kebidan_ralan(
    db: &MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Option<AssesmenKebidananRalan>> {
    sqlx::query_file_as!(
        AssesmenKebidananRalan,
        "./src/dto/assesmen_kebidanan/get-assesmen-bidan-ralan.sql",
        no_rawat
    )
    .fetch_optional(db)
    .await
}
