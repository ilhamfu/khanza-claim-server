use maud::{html, Markup};
use ranap::render_resume_ranap;

use crate::dto::resume::Resume;

use super::TemplateContext;

mod ranap;

pub fn render_resume(context: &TemplateContext, resume: &Resume) -> Markup {
    html! {
        @if let Some(ranap) = resume.ranap.as_ref() {
            (render_resume_ranap(context, ranap))
        }
    }
}
