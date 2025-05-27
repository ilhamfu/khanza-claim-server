mod gambar;
mod hasil;
mod pemeriksaan;
use maud::{html, Markup, Render};

use crate::dto::radiologi::PemeriksaanRadiologi;

use super::TemplateContext;

pub fn render_radiologi(context: &TemplateContext, radiologi: &PemeriksaanRadiologi) -> Markup {
    if radiologi.is_empty() {
        return "".render();
    }
    html! {
        .radiologi {
            .radiologi__title { "Pemeriksaan Radiologi" }
            .radiologi__body {
                (pemeriksaan::render_pemeriksaan_radiologi_list(context, &radiologi.pemeriksaan))
                (hasil::render_hasil_radiologi_list(context, &radiologi.hasil))
                (gambar::render_gambar_radiologi_list(context, &radiologi.gambar))
            }
        }
    }
}
