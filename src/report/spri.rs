use maud::{html, Markup};

use crate::{
    dto::spri::Spri,
    report::molecule::{tricolumn_colon, BPJS_LOGO},
    util::base64qr,
};

use super::TemplateContext;

pub fn render_spri(context: &TemplateContext, spri: &Spri) -> Markup {
    let dokter = [
        spri.nama_dokter.as_deref().unwrap_or_default(),
        spri.nama_poli.as_deref().unwrap_or_default(),
    ];

    let qr = base64qr(
        &format!(
            "Surat ini ditandatangani oleh {} (ID {}) di {} pada tanggal {} secara digital",
            spri.nama_dokter.as_deref().unwrap_or_default(),
            spri.kode_dokter.as_deref().unwrap_or_default(),
            context.config.rs_name,
            spri.tanggal_lahir
                .map(|item| item.format("%d-%m-%Y").to_string())
                .unwrap_or_default(),
        ),
        (96, 96),
    )
    .unwrap_or_default();

    html! {
        .spri{
            .spri__header{
                .spri__bpjs-logo{ (BPJS_LOGO) }
                .spri__title { "SURAT PERINTAH RAWAT INAP" }
                .spri__subtitle{ (context.config.rs_name) }
            }
            .spri__letter-info {
                .spri__letter-number{ "No. " (spri.no_surat.as_deref().unwrap_or_default()) }
                .spri__letter-at{ "Tgl. " (spri.tanggal_rencana.as_ref().map(|a|a.format("%A, %-d %B %C%y").to_string()).unwrap_or_default()) }
            }
            .spri__body {
                .spri__body-section.spri__body-section--left{
                    (tricolumn_colon("Kepada Yth.", &dokter[..]))
                    (tricolumn_colon("No Kartu", &spri.no_kartu))
                    (tricolumn_colon("Nama", &spri.nama_pasien))
                    (tricolumn_colon("Tanggal Lahir", &spri.tanggal_lahir.as_ref().map(|a|a.format("%A, %-d %B %C%y").to_string())))
                    (tricolumn_colon("Diagnosae", &spri.diagnosa))
                    (tricolumn_colon("Tanggal Surat", &spri.tanggal_surat.format("%A, %-d %B %C%y").to_string()))
                }
                .spri__body-section.spri__body-section--right{
                    div {"Mengetahui"}
                    div { img src=(qr); }
                    div { (spri.nama_dokter.as_deref().unwrap_or_default()) }
                }
            }
        }

    }
}
