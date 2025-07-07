use chrono::NaiveDate;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RujukanPenuh {
    pub no_sep: String,
    pub no_rawat: String,
    pub no_rm: String,
    pub no_kartu: String,
    pub nm_pasien: String,
    pub nm_dokter: String,
    pub kd_dokter: String,
    pub jk: String,
    pub tgl_surat_rujuk: NaiveDate,
    pub no_surat_rujuk: String,
    pub kd_ppk_rujuk: String,
    pub nm_ppk_rujuk: String,
    pub jns_pelayanan: String,
    pub tgl_rujukan: String,
    pub catatan: String,
    pub kd_diagnosa_rujuk: String,
    pub nm_diagnosa_rujuk: String,
    pub kd_poli_rujuk: String,
    pub nm_poli_rujuk: String,
    pub tgl_rencana_rujuk: NaiveDate,
}

pub async fn get_rujukan_penuh(
    pool: &sqlx::MySqlPool,
    no_sep: &str,
) -> sqlx::Result<Option<RujukanPenuh>> {
    sqlx::query_file_as!(RujukanPenuh, "./src/dto/get-rujuk-penuh.sql", no_sep)
        .fetch_optional(pool)
        .await
}
