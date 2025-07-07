use maud::{html, Markup};

use crate::{
    dto::sep::Sep,
    report::molecule::{tricolumn_colon, BPJS_LOGO},
    util::base64qr,
};

use super::TemplateContext;

pub fn render_sep(context: &TemplateContext, sep: &Sep) -> Markup {
    let jenis_kunjungan = [&sep.flag_prosedur, &sep.tujuan_kunjungan];
    let qr = base64qr(
        &format!(
            "Surat ini telah ditandatangani oleh {} (ID {}) di {} pada tanggal {} secara digital",
            sep.nama_pasien.as_deref().unwrap_or_default(),
            sep.no_rm.as_deref().unwrap_or_default(),
            context.config.rs_name,
            sep.tanggal_sep
                .map(|e| e.format("%d-%m-%Y").to_string())
                .unwrap_or_default(),
        ),
        (128, 128),
    )
    .unwrap();

    html! {
        .sep {
            .sep__header{
                .sep__header-logo { (BPJS_LOGO) }
                .sep__header-title { "SURAT ELIGIBILITAS PESERTTA" }
                .sep__header-subtitle { (context.config.rs_name) }
            }
            .sep__body {
                .sep__body-section.sep__body-section--left{
                    (tricolumn_colon("No. SEP", &sep.no_sep))
                    (tricolumn_colon("Tgl. SEP", &sep.tanggal_sep.as_ref().map(|a|a.format("%A, %d %B %C%y").to_string())))
                    (tricolumn_colon("Nama Peserta", format!("{} ({})",sep.nama_pasien.as_deref().unwrap_or_default(),sep.jk.as_deref().unwrap_or_default())))
                    (tricolumn_colon("Tanggal Lahir", &sep.tanggal_lahir.as_ref().map(|a|a.format("%A, %-d %B %C%y").to_string())))
                    (tricolumn_colon("No. Telp", &sep.no_telp))
                    (tricolumn_colon("Sub/Spesialis", &sep.nama_poli_tujuan))
                    (tricolumn_colon("Dokter", &sep.nama_dpjp))
                    (tricolumn_colon("Faskes Perujuk", &sep.nama_ppk_rujukan))
                    (tricolumn_colon("Diagnosa Awal", &sep.nama_diagnosa_awal))
                    (tricolumn_colon("Catatan", &sep.catatan))
                }
                .sep__body-section.sep__body-section--right{
                    (tricolumn_colon("Peserta", &sep.peserta))
                    (tricolumn_colon("Jns. Rawat", &sep.jenis_pelayanan))
                    (tricolumn_colon("Jns. Kunjungan",&jenis_kunjungan[..]))
                    (tricolumn_colon("Poli Perujuk", "-"))
                    (tricolumn_colon("Kelas Hak", &sep.kelas_rawat))
                    (tricolumn_colon("Kelas Rawat", &sep.kelas_naik))
                    (tricolumn_colon("Penjamin", &sep.lakalantas))
                }
            }
            .sep__footer {
                table.sep__notes {
                    tbody{
                        tr{
                            td colspan="2"{ "*Saya menyetujui BPJS Kesehatan untuk :" }
                        }
                        tr{
                            td{"a."}
                            td{ "membuka dan atau menggunakan informasi medis Pasien untuk keperluan administrasi, pembayaran asuransi atau jaminan pembiayaan kesehatan" }
                        }
                        tr{
                            td{"b."}
                            td{ "memberikan akses informasi medis atau riwayat pelayanan kepada dokter/tenaga medis pada RSU GLADISH MEDICAL CENTER untuk kepentingan pemeliharaan kesehatan, pengobatan, penyembuhan, dan perawatan Pasien" }
                        }
                        tr{
                            td colspan="2"{ "*Saya mengetahui dan memahami :" }
                        }
                        tr{
                            td{"a."}
                            td{ "Rumah Sakit dapat melakukan koordinasi dengan PT Jasa Raharja / PT Taspen / PT ASABRI / BPJS Ketenagakerjaan atau Penjamin lainnya, jika Peserta merupakan pasien yang mengalami kecelakaan lalulintas dan / atau kecelakaan kerja" }
                        }
                        tr{
                            td{"b."}
                            td{"SEP bukan sebagai bukti penjaminan peserta"}
                        }
                        tr{
                            td colspan="2"{ "* Dengan tampilnya luaran SEP elektronik ini merupakan hasil validasi terhadap eligibilitas Pasien secara elektronik (validasi finger print atau biometrik / sistem validasi lain) dan selanjutnya Pasien dapat mengakses pelayanan kesehatan rujukan sesuai ketentuan berlaku. Kebenaran dan keaslian atas informasi data Pasien menjadi tanggung jawab penuh FKRTL" }
                        }
                    }
                }

                .sep__signature {
                    div {"Pasien / Keluarga Pasien"}
                    div { img src=(qr); }
                    div { (&sep.nama_pasien.as_deref().unwrap_or_default()) }
                }
            }
        }
    }
}
