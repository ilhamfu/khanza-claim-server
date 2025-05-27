use maud::{html, Markup};

use crate::{dto::DetailRawat, util::base64qr};

use super::TemplateContext;

pub fn render_signature(_context: &TemplateContext, periksa: &DetailRawat) -> Markup {
    let qrs = match periksa.reg_periksa.status_lanjut.as_ref() {
        "Ralan" => {
            let text = format!(
                "Surat ini telah ditandatangani oleh {} (ID {}) di {} pada tanggal {} secara digital",
                periksa.reg_periksa.nama_dokter,
                periksa.reg_periksa.kode_dokter,
                _context.config.rs_name,
                periksa.reg_periksa.tgl_registrasi.format("%d-%m-%Y"),
            );
            let qr = base64qr(&text, (128, 128)).expect("error when creating qrcode");
            vec![(qr, &periksa.reg_periksa.nama_dokter)]
        }
        "Ranap" => {
            let v = periksa.dpjp_ranap.iter().map(|item|{
                let text = format!(
                    "Surat ini telah ditandatangani oleh {} (ID {}) di {} pada tanggal {} secara digital",
                    item.nama_dokter,
                    item.kode_dokter,
                    _context.config.rs_name,
                    periksa.reg_periksa.tgl_registrasi.format("%d-%m-%Y")
                );
                let qr = base64qr(&text, (128, 128)).expect("error when creating qrcode");
                (qr,&item.nama_dokter)
            }).collect();

            v
        }
        _ => return html! {},
    };

    html! {
        .dpjp-signature.section {
            .section__title {"Dokter DPJP"}
            .section__body.dpjp-signature__list {
                @for (qr,name) in &qrs {
                    div {
                        div { img src=(qr); }
                        div { (name) }
                    }
                }
            }
        }
    }
}
