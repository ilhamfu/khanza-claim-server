#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Operasi {
    pub no_rawat: Option<String>,
    pub nama_perawatan: Option<String>,
    pub tanggal_operasi: Option<chrono::NaiveDateTime>,
    pub jenis_anasthesi: Option<String>,
    pub kategori: Option<String>,
    pub diagnosa_preop: Option<String>,
    pub diagnosa_postop: Option<String>,
    pub jaringan_dieksekusi: Option<String>,
    pub selesai_operasi: Option<chrono::NaiveDateTime>,
    pub permintaan_pa: Option<String>,
    pub laporan_operasi: Option<String>,
    pub nama_op_1: Option<String>,
    pub nama_op_2: Option<String>,
    pub nama_op_3: Option<String>,
    pub nama_asop_1: Option<String>,
    pub nama_asop_2: Option<String>,
    pub nama_asop_3: Option<String>,
    pub nama_instrumen: Option<String>,
    pub nama_dokter_anak: Option<String>,
    pub nama_perawat_resusitas: Option<String>,
    pub nama_dokter_anasthesi: Option<String>,
    pub nama_asisten_anesthesi_1: Option<String>,
    pub nama_asisten_anesthesi_2: Option<String>,
    pub nama_bidan_1: Option<String>,
    pub nama_bidan_2: Option<String>,
    pub nama_bidan_3: Option<String>,
    pub nama_perawat_luar: Option<String>,
    pub nama_onloop_1: Option<String>,
    pub nama_onloop_2: Option<String>,
    pub nama_onloop_3: Option<String>,
    pub nama_onloop_4: Option<String>,
    pub nama_onloop_5: Option<String>,
    pub nama_dokter_pj_anak: Option<String>,
    pub nama_dokter_umum: Option<String>,
}

pub async fn get_operasi(db: &sqlx::MySqlPool, no_rawat: &str) -> sqlx::Result<Vec<Operasi>> {
    sqlx::query_file_as!(Operasi, "./src/dto/get-operasi.sql", no_rawat)
        .fetch_all(db)
        .await
}
