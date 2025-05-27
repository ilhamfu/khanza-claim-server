use maud::{html, Markup};

use crate::dto::assesmen_kebidanan::AssesmenKebidanan;

use super::TemplateContext;

mod ralan;
mod ranap;

pub fn render_assesmen_kebidanan(
    context: &TemplateContext,
    AssesmenKebidanan { ranap, ralan }: &AssesmenKebidanan,
) -> Markup {
    html! {
        (ralan.as_ref().map(|e|ralan::render_assesmen_kebidanan_ralan(context, e)).unwrap_or_default())
        (ranap.as_ref().map(|e|ranap::render_assesmen_kebidanan_ranap(context, e)).unwrap_or_default())
    }
}
fn some_and_wrap(prefix: &str, text: Option<&str>, postfix: &str) -> String {
    match text {
        Some(e) if !e.is_empty() && e != "-" => {
            format!("{prefix}{e}{postfix}")
        }
        _ => "".to_owned(),
    }
}
