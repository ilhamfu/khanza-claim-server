use maud::{html, Markup, Render};

use crate::{dto::laporan_operasi::LaporanOperasi, report::molecule::tricolumn_colon};

use super::TemplateContext;

pub fn render_laporan_operasi_list(
    context: &TemplateContext,
    laporan_operasi_list: &[LaporanOperasi],
) -> Markup {
    if laporan_operasi_list.is_empty() {
        return "".render();
    }
    html! {
        .laporan-operasi-list{
            .laporan-operasi-list__title{("Laporan Operasi")}
            .laporan-operasi-list__body{
                @for i in laporan_operasi_list{
                    (render_laporan_operasi(context, &i))
                }
            }
        }
    }
}
fn render_laporan_operasi(_context: &TemplateContext, laporan_operasi: &LaporanOperasi) -> Markup {
    html! {
        .laporan-operasi {
            (tricolumn_colon("Mulai Operasi", laporan_operasi.tanggal.as_ref().map(|item| item.format("%d-%m-%Y %H:%M").to_string())))
            (tricolumn_colon("Diagnosa Pra-operasi",&laporan_operasi.diagnosa_preop))
            (tricolumn_colon("Jaringan yang di Eksisi",&laporan_operasi.jaringan_dieksekusi))
            (tricolumn_colon("Diagnosa Post-operasi",&laporan_operasi.diagnosa_postop))
            (tricolumn_colon("Selesai Operasi", laporan_operasi.selesai_operasi.as_ref().map(|item| item.format("%d-%m-%Y %H:%M").to_string())))
            (tricolumn_colon("Dikirim Untuk Pemeriksaan", &laporan_operasi.permintaan_pa))
            .laporan_operasi__laporan {
                div { "Laporan" }
                div {(laporan_operasi.laporan_operasi.as_deref().unwrap_or_default())}
            }
        }
    }
}
