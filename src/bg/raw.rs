use anyhow::Context;

use crate::{report::TemplateContext, AppContext};

pub async fn export_raw(context: &AppContext, no_rawat: &str) -> anyhow::Result<()> {
    let reg_periksa = crate::dto::get_rawat(&context.pool, no_rawat)
        .await
        .context("error when getting encounter document")?
        .context("encounter document not found")?;

    let template_context = TemplateContext {
        config: &context.config.app_config,
    };

    let html = crate::report::render_report(&template_context, &reg_periksa);

    println!("{}", html.into_string());
    Ok(())
}
