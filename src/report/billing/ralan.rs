use crate::dto::billing::Total;
use maud::{html, Markup};

use crate::{
    dto::billing::BillingRalan,
    report::{
        billing::{format_money, format_qty},
        TemplateContext,
    },
};

pub fn render_billing_ralan(_context: &TemplateContext, ralan: &BillingRalan) -> Markup {
    html! {
        .billing.section {
            .section__title.billing__title{ "Billing" }
            .section__body.billing__body {
                .billing__header {
                    div { "Nama Item" }
                    div { "Jml" }
                    div { "Harga" }
                    div { "Tambahan" }
                    div { "Total" }

                }
                .billing__item {
                    .item{(ralan.reg.detail)}
                    .total {(format_money(ralan.reg.total))}
                }
                @for i in &ralan.pem {
                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Pemeriksaan dan Tindakan"}
                    }
                    .billing__item.billing__item--sub {
                        .item {(i.detail)}
                        .qty {(i.qty.map(format_qty).unwrap_or_default())}
                        .cost {(i.price.map(format_money).unwrap_or_default())}
                        .total {(format_money(i.total))}
                    }
                }

                .billing__item.billing__item--important {
                    .item.billing__item--important {"Subtotal : Pemeriksaan dan Tindakan"}
                    .total {(format_money(ralan.pem.subtotal()))}
                }
                @if !ralan.rad.is_empty(){
                    .billing__item.billing__item--important {
                        .item {"Pemeriksaan Radiologi"}
                    }
                    @for i in &ralan.rad {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .qty {(i.qty.map(format_qty).unwrap_or_default())}
                            .cost {(i.price.map(format_money).unwrap_or_default())}
                            .total {(format_money(i.total))}
                        }
                    }

                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Subtotal : Pemeriksaan Radiologi"}
                        .total {(format_money(ralan.rad.subtotal()))}
                    }
                }
                @if !ralan.lab.is_empty(){
                    .billing__item.billing__item--important {
                        .item {"Pemeriksaan Lab"}
                    }
                    @for i in &ralan.lab {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .qty {(i.qty.map(format_qty).unwrap_or_default())}
                            .cost {(i.price.map(format_money).unwrap_or_default())}
                            .extra {(i.extra.map(format_money).unwrap_or_default())}
                            .total {(format_money(i.total))}
                        }
                    }
                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Subtotal : Pemeriksaan Lab"}
                        .total {(format_money(ralan.lab.subtotal()))}
                    }
                }
                @if !ralan.operasi.is_empty(){
                    .billing__item.billing__item--important {
                        .item {"Operasi"}
                    }
                    @for i in &ralan.operasi {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .qty {(i.qty.map(format_qty).unwrap_or_default())}
                            .cost {(i.price.map(format_money).unwrap_or_default())}
                            .extra {(i.extra.map(format_money).unwrap_or_default())}
                            .total {(format_money(i.total))}
                        }
                    }
                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Subtotal : Operasi"}
                        .total {(format_money(ralan.operasi.subtotal()))}
                    }
                }
                @if !ralan.obat_op.is_empty(){
                    .billing__item.billing__item--important {
                        .item {"Obat & BHP Operasi"}
                    }
                    @for i in &ralan.obat_op {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .qty {(i.qty.map(format_qty).unwrap_or_default())}
                            .cost {(i.price.map(format_money).unwrap_or_default())}
                            .total {(format_money(i.total))}
                        }
                    }
                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Subtotal : Obat Operasi"}
                        .total {(format_money(ralan.obat_op.subtotal()))}
                    }
                }
                @if !ralan.obat.is_empty(){
                    .billing__item.billing__item--important {
                        .item {"Obat & BHP"}
                    }
                    @for i in &ralan.obat {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .qty {(i.qty.map(format_qty).unwrap_or_default())}
                            .cost {(i.price.map(format_money).unwrap_or_default())}
                            .total {(format_money(i.total))}
                        }
                    }
                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Subtotal : Obat & BHP"}
                        .total {(format_money(ralan.obat.subtotal()))}
                    }
                }
                .billing__item.billing__item--important style="margin-top:0.25rem;padding-top:0.25rem;" {
                    div style="grid-column: first / extra-end; border-top: solid 1px" {("Total")}
                    .total style="border-top: solid 1px" {(format_money(ralan.sum()))}
                }
            }
        }
    }
}
