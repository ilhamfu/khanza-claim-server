#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct ResumeRanap {
    pub kode_dokter: Option<String>,
    pub nama_dokter: Option<String>,
    pub diagnosa_awal: Option<String>,
    pub alasan: Option<String>,
    pub keluhan_utama: Option<String>,
    pub pemeriksaan_fisik: Option<String>,
    pub jalannya_penyakit: Option<String>,
    pub pemeriksaan_penunjang: Option<String>,
    pub hasil_laborat: Option<String>,
    pub tindakan_dan_operasi: Option<String>,
    pub obat_rs: Option<String>,
    pub diagnosa_utama: Option<String>,
    pub kd_diagnosa_utama: Option<String>,
    pub diagnosa_sekunder: Option<String>,
    pub kd_diagnosa_sekunder: Option<String>,
    pub diagnosa_sekunder2: Option<String>,
    pub kd_diagnosa_sekunder2: Option<String>,
    pub diagnosa_sekunder3: Option<String>,
    pub kd_diagnosa_sekunder3: Option<String>,
    pub diagnosa_sekunder4: Option<String>,
    pub kd_diagnosa_sekunder4: Option<String>,
    pub prosedur_utama: Option<String>,
    pub kd_prosedur_utama: Option<String>,
    pub prosedur_sekunder: Option<String>,
    pub kd_prosedur_sekunder: Option<String>,
    pub prosedur_sekunder2: Option<String>,
    pub kd_prosedur_sekunder2: Option<String>,
    pub prosedur_sekunder3: Option<String>,
    pub kd_prosedur_sekunder3: Option<String>,
    pub alergi: Option<String>,
    pub diet: Option<String>,
    pub lab_belum: Option<String>,
    pub edukasi: Option<String>,
    pub cara_keluar: Option<String>,
    pub ket_keluar: Option<String>,
    pub keadaan: Option<String>,
    pub ket_keadaan: Option<String>,
    pub dilanjutkan: Option<String>,
    pub ket_dilanjutkan: Option<String>,
    pub kontrol: Option<chrono::NaiveDateTime>,
    pub obat_pulang: Option<String>,
}

pub async fn get_resume_ranap(
    db: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Option<ResumeRanap>> {
    sqlx::query_file_as!(
        ResumeRanap,
        "./src/dto/resume/get-resume-ranap.sql",
        no_rawat
    )
    .fetch_optional(db)
    .await
}
