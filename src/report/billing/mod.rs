mod ralan;
mod ranap;

use maud::Markup;
use ralan::render_billing_ralan;
use ranap::render_billing_ranap;

use crate::dto::billing::Billing;

use super::TemplateContext;

fn format_money(val: f64) -> String {
    format_num::format_num!(",.2", val)
}

fn format_qty(val: impl std::fmt::Display) -> String {
    format!("{val:>3}")
}

pub fn render_billing(context: &TemplateContext, billing: &Billing) -> Markup {
    match billing {
        Billing::Ralan(ref billing_ralan) => render_billing_ralan(context, billing_ralan),
        Billing::Ranap(ref billing_ranap) => render_billing_ranap(context, billing_ranap),
    }
}
