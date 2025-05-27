use maud::{html, Markup, Render};

use crate::{
    dto::radiologi::GambarRadiologi,
    report::{molecule::tricolumn_colon, TemplateContext},
};

pub fn render_gambar_radiologi_list(
    context: &TemplateContext,
    radiologi: &[GambarRadiologi],
) -> Markup {
    if radiologi.is_empty() {
        return "".render();
    }
    html! {
        .radiologi-gambar-list style=(format!("--row-length : {}",radiologi.len()+1)){
            .radiologi-gambar-list__title {
                "Gambar"
            }
            .radiologi-gambar-list__item {
                @for i in radiologi {
                    (render_gambar_radiologi(context, i))
                }
            }
        }
    }
}

fn render_gambar_radiologi(context: &TemplateContext, rad: &GambarRadiologi) -> Markup {
    let date = rad
        .tanggal_periksa
        .and_time(rad.jam_periksa)
        .format("%d-%m-%Y %H:%M")
        .to_string();
    html! {
        .radiologi-gambar {
            (tricolumn_colon("Tanggal & Waktu", date))
            .radiologi-gambar__gambar {
               .radiologi-gambar__gambar-title {"Gambar"}
               .radiologi-gambar__gambar-body {
                img src=(format!("{}/radiologi/{}",context.config.resource_location,rad.lokasi_gambar));
               }
            }
        }
    }
}
