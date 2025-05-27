use maud::{html, Markup};

use crate::{
    dto::billing::BillingRanap,
    report::{
        billing::{format_money, format_qty},
        TemplateContext,
    },
};

pub fn render_billing_ranap(_context: &TemplateContext, ralan: &BillingRanap) -> Markup {
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
                @if !ralan.kamar.is_empty() {
                    .billing__item.billing__item--important{
                        .item { "Kamar Inap" }
                    }
                    @for i in &ralan.kamar {
                        .billing__item.billing__item--sub {
                            .item { (i.0.detail) }
                            .qty { (i.0.qty.map(format_qty).unwrap_or_default() )}
                            .cost { (i.0.price.map(format_money).unwrap_or_default() )}
                            .total { (format_money(i.0.total))}
                        }
                        @for j in &i.1{
                            .billing__item.billing__item--sub {
                                .item { (j.detail) }
                                .qty { (j.qty.map(format_qty).unwrap_or_default() )}
                                .cost { (j.price.map(format_money).unwrap_or_default() )}
                                .total { (format_money(j.total))}
                            }
                        }
                        @for j in &i.2{
                            .billing__item.billing__item--sub {
                                .item { (j.detail) }
                                .qty { (j.qty.map(format_qty).unwrap_or_default() )}
                                .cost { (j.price.map(format_money).unwrap_or_default() )}
                                .total { (format_money(j.total))}
                            }
                        }
                    }
                }

                @if !ralan.pem_ralan.is_empty() {
                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Pemeriksaan dan Tindakan Rawat Jalan"}
                    }
                    @for i in &ralan.pem_ralan {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .qty {(i.qty.map(format_qty).unwrap_or_default())}
                            .cost {(i.price.map(format_money).unwrap_or_default())}
                            .total {(format_money(i.total))}
                        }
                    }
                }
                @if !ralan.pem_ranap.is_empty() {
                    .billing__item.billing__item--important {
                        .item.billing__item--important {"Pemeriksaan dan Tindakan Rawat Inap"}
                    }
                    @for i in &ralan.pem_ranap {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .qty {(i.qty.map(format_qty).unwrap_or_default())}
                            .cost {(i.price.map(format_money).unwrap_or_default())}
                            .total {(format_money(i.total))}
                        }
                    }
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
                }
                @if !ralan.operasi.is_empty(){
                    .billing__item.billing__item--important {
                        .item {"Operasi"}
                    }
                    @for i in &ralan.operasi {
                        .billing__item.billing__item--sub {
                            .item {(i.detail)}
                            .total {(format_money(i.total))}
                        }
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
                }
                .billing__item.billing__item--important style="margin-top:0.25rem;padding-top:0.25rem;" {
                    div style="grid-column: first / extra-end; border-top: solid 1px" {("Total")}
                    .total style="border-top: solid 1px" {(format_money(ralan.sum()))}
                }
            }
        }
    }
}
