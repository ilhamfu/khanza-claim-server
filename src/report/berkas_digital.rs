use maud::{html, Markup, Render};

use crate::dto::berkas_digital::BerkasDigial;

use super::TemplateContext;

pub fn render_berkas_digital_list(
    context: &TemplateContext,
    list_berkas_digital: &[BerkasDigial],
) -> Markup {
    if list_berkas_digital.is_empty() {
        return "".render();
    }
    html! {
        .berkas-digital-list {
            .berkas-digital-list__title { "Berkas Pendukung Lainnya" }
            .berkas-digital-list__item{
                @for i in list_berkas_digital.iter().flat_map(|item| render_berkas_digital(context, item)) {
                    (i)
                }
            }
        }
    }
}

fn render_berkas_digital(_context: &TemplateContext, bd: &BerkasDigial) -> Option<Markup> {
    let lokasi_file = bd.lokasi_file.as_deref()?;

    if lokasi_file.ends_with("pdf") {
        return None;
    }

    Some(html! {
        .berkas-digital {
            .berkas-digital__title {
                (bd.kategori.as_deref().unwrap_or_default())
            }
            .berkas-digital__body {
                img src=(format!("{}/berkasrawat/{}",_context.config.resource_location,lokasi_file));
            }

        }

    })
}
