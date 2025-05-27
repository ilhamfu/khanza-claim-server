use chrono::NaiveDateTime;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HasilUsg {
    pub kd_dokter: Option<String>,
    pub nm_dokter: Option<String>,
    pub diagnosa_klinis: Option<String>,
    pub kiriman_dari: Option<String>,
    pub hta: Option<String>,
    pub kantong_gestasi: Option<String>,
    pub ukuran_bokongkepala: Option<String>,
    pub jenis_prestasi: Option<String>,
    pub diameter_biparietal: Option<String>,
    pub panjang_femur: Option<String>,
    pub lingkar_abdomen: Option<String>,
    pub tafsiran_berat_janin: Option<String>,
    pub usia_kehamilan: Option<String>,
    pub plasenta_berimplatansi: Option<String>,
    pub derajat_maturitas: Option<String>,
    pub jumlah_air_ketuban: Option<String>,
    pub indek_cairan_ketuban: Option<String>,
    pub kelainan_kongenital: Option<String>,
    pub peluang_sex: Option<String>,
    pub kesimpulan: Option<String>,
    pub tanggal: Option<NaiveDateTime>,
    pub gambar_usg: Option<String>,
}

pub async fn get_hasil_usg(
    pool: &sqlx::MySqlPool,
    no_rawat: &str,
) -> sqlx::Result<Option<HasilUsg>> {
    sqlx::query_file_as!(HasilUsg, "./src/dto/get-hasil-usg.sql", no_rawat)
        .fetch_optional(pool)
        .await
}
