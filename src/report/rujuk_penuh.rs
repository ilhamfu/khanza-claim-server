use maud::html;

use crate::{dto::rujukan_penuh::RujukanPenuh, report::molecule::tricolumn_colon, util::base64qr};

use super::TemplateContext;

pub fn render_rujuk_penuh(context: &TemplateContext, rujuk_penuh: &RujukanPenuh) -> maud::Markup {
    let qr = base64qr(
        &format!(
            "Surat ini ditandatangani oleh {} (ID {}) di {} pada tanggal {} secara digital",
            rujuk_penuh.nm_dokter,
            rujuk_penuh.kd_dokter,
            context.config.rs_name,
            rujuk_penuh.tgl_surat_rujuk.format("%d-%m-%Y")
        ),
        (96, 96),
    )
    .unwrap_or_default();

    let sampai = rujuk_penuh.tgl_rencana_rujuk.format("%d %B %Y").to_string();
    let rencana = rujuk_penuh
        .tgl_rencana_rujuk
        .checked_add_days(chrono::Days::new(90))
        .unwrap()
        .format("%d %B %Y")
        .to_string();

    html! {
        .surat-rujuk {
            .surat-rujuk__header {
                .surat-rujuk__title { "SURAT RUJUKAN RUMAH SAKIT" }
                .surat-rujuk__subtitle{ (context.config.rs_name) }
            }
            .surat-rujuk__top {
                .surat-rujuk__dest  { "Kepada Yth. " (rujuk_penuh.nm_ppk_rujuk) }
                .surat-rujuk__detail {
                    (tricolumn_colon("No. Rujukan", &rujuk_penuh.no_surat_rujuk))
                    (tricolumn_colon("Rujukan Penuh", "Rujukan Penuh"))
                }
            }
            p { "Mohon pemeriksaan dan penanganan lebih lanjut penderita: " }
            .surat-rujuk__patient {
                div{
                    (tricolumn_colon("Nama", &rujuk_penuh.nm_pasien))
                    (tricolumn_colon("No. BPJS", &rujuk_penuh.no_kartu))
                    (tricolumn_colon("Diagnosa", format!("{} - {}",&rujuk_penuh.kd_diagnosa_rujuk,&rujuk_penuh.nm_diagnosa_rujuk)))
                    (tricolumn_colon("Keterangan", &rujuk_penuh.catatan))
                }
                div {
                    (tricolumn_colon("Kelamin", &rujuk_penuh.jk))
                    (tricolumn_colon("Rawat", &rujuk_penuh.jns_pelayanan))
                }
            }
            p{ "Demikian atas bantuannya terima kasih" }
            .surat-rujuk__footer {
                .surat-rujuk__extra {
                    div { (format!("*) Rujukan Berlaku Sampai Dengan {}",sampai)) }
                    div { (format!("**) Tanggal Rencana Berkunjung {}",rencana)) }

                }
                .surat-rujuk__signature {
                    div { "Pesawaran, " (rujuk_penuh.tgl_surat_rujuk.format("%-d %B %C%y"))}
                    div { "Mengetahui" }
                    div { img src=(qr); }
                    div { (rujuk_penuh.nm_dokter) }

                }
            }
        }

    }
}
