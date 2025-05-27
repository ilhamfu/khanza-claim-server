use maud::{html, Markup, Render};

use crate::{dto::lab::Lab, report::molecule::tricolumn_colon};

use super::TemplateContext;

pub fn render_lab_list(context: &TemplateContext, labs: &[Lab]) -> Markup {
    if labs.is_empty() {
        return "".render();
    }
    html! {
        .lab-list {
            .lab-list__title { "Pemeriksaan Laboratorium" }
            .lab-list__body{
                @for i in labs {
                    (render_lab(context, i))
                }
            }
        }
    }
}

fn render_lab(_context: &TemplateContext, lab: &Lab) -> Markup {
    let time = lab
        .tanggal_periksa
        .and_time(lab.jam)
        .format("%d-%m-%Y %H:%M")
        .to_string();
    html! {
        .lab {
            (tricolumn_colon("Tanggal & Waktu",time))
            (tricolumn_colon("Nama Pemeriksaan",&lab.nama_perawatan ))
            (tricolumn_colon("Dokter Penanggung Jawab",&lab.nama_dokter ))
            (tricolumn_colon("Petugas",&lab.nama_petugas ))
            .lab-detail {
                .lab-detail__title { "Detail Pemeriksaan" }
                .lab-detail__body {
                    @for i in lab.detail.as_ref().map(|e|&e.0[..]).unwrap_or(&[]) {
                        (tricolumn_colon(&i.pemeriksaan,format!("{}{} ({})",&i.nilai,&i.satuan,&i.nilai_rujukan) ))
                    }
            }
        }
    }
    }
}
