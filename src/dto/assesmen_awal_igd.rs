use super::reg_periksa::RegPeriksa;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AssesmenAwalIGD {
    pub tanggal: chrono::NaiveDateTime,
    pub anamnesis: String,
    pub hubungan: String,
    pub keluhan_utama: String,
    pub rps: String,
    pub rpk: String,
    pub rpd: String,
    pub rpo: String,
    pub alergi: String,
    pub keadaan: String,
    pub gcs: String,
    pub kesadaran: String,
    pub td: String,
    pub nadi: String,
    pub rr: String,
    pub suhu: String,
    pub spo: String,
    pub bb: String,
    pub tb: String,
    pub kepala: String,
    pub mata: String,
    pub gigi: String,
    pub leher: String,
    pub thoraks: String,
    pub abdomen: String,
    pub ekstremitas: String,
    pub genital: String,
    pub ket_fisik: String,
    pub ket_lokalis: String,
    pub ekg: String,
    pub rad: String,
    pub lab: String,
    pub diagnosis: String,
    pub tata: String,
    pub kode_dokter: String,
    pub nama_dokter: String,
}

pub async fn get_assesmen_awal_igd(
    pool: &sqlx::MySqlPool,
    rp: &RegPeriksa,
) -> sqlx::Result<Option<AssesmenAwalIGD>> {
    if rp.status_lanjut == "Ranap" && rp.kode_poli != "IGDK" {
        return Ok(None);
    }
    sqlx::query_file_as!(
        AssesmenAwalIGD,
        "./src/dto/get-assesmen-ralan-igd.sql",
        rp.no_rawat
    )
    .fetch_optional(pool)
    .await
}
