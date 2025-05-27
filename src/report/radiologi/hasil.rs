use maud::{html, Markup, Render};

use crate::{
    dto::radiologi::HasilPemeriksaanRadiologi,
    report::{molecule::tricolumn_colon, TemplateContext},
};

pub fn render_hasil_radiologi_list(
    context: &TemplateContext,
    radiologi: &[HasilPemeriksaanRadiologi],
) -> Markup {
    if radiologi.is_empty() {
        return "".render();
    }
    html! {
        .radiologi-hasil-list style=(format!("--row-length : {}",radiologi.len()+1)){
            .radiologi-hasil-list__title {
                "Hasil"
            }
            .radiologi-hasil-list__item {
                @for i in radiologi {
                    (render_hasil_radiologi(context, i))
                }
            }
        }
    }
}

fn render_hasil_radiologi(_context: &TemplateContext, rad: &HasilPemeriksaanRadiologi) -> Markup {
    let date = rad
        .tanggal_periksa
        .and_time(rad.jam_periksa)
        .format("%d-%m-%Y %H:%M")
        .to_string();
    html! {
        .radiologi-hasil {
            (tricolumn_colon("Tanggal & Waktu", date))
            .radiologi-hasil__hasil {
               .radiologi-hasil__hasil-title {"Pemeriksaan"}
               .radiologi-hasil__hasil-body {
                (rad.hasil)
               }
            }
        }
    }
}
