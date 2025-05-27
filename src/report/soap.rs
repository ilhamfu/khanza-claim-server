use maud::{html, Markup};

use crate::{dto::soap::Soap, report::molecule::tricolumn_colon};

use super::TemplateContext;

pub fn render_soap_list(context: &TemplateContext, soap: &[Soap]) -> Markup {
    if soap.is_empty() {
        return html! {};
    }
    html! {
        .section {
            .section__title { "PEMERIKSAAN RAWAT JALAN" }
            .soap-list {
                @for s in soap {
                    (render_soap(context,&s))
                }
            }
        }
    }
}

pub fn render_soap(_context: &TemplateContext, soap: &Soap) -> Markup {
    let time = soap
        .tgl_perawatan
        .and_time(soap.jam_rawat)
        .format("%d-%m-%Y %H:%M")
        .to_string();

    html! {
        .soap{
            (tricolumn_colon("Waktu Pemeriksaan",time ))
            (tricolumn_colon("Nama Petugas",&soap.nama_pegawai ))
            (tricolumn_colon("Subjek",&soap.keluhan ))
            (tricolumn_colon("Objek",&soap.pemeriksaan ))
            .soap__obj{
                .soap__obj-group {
                    (tricolumn_colon("Suhu",format!("{} C",soap.suhu_tubuh) ))
                    (tricolumn_colon("Tensi",&soap.tensi ))
                    (tricolumn_colon("Nadi",format!("{}/menit",soap.nadi) ))
                    (tricolumn_colon("Respirasi",format!("{}/menit",soap.respirasi) ))
                    (tricolumn_colon("Tinggi",format!("{} cm",soap.tinggi) ))
                }
                .soap__obj-group {
                    (tricolumn_colon("Berat",format!("{} Kg",soap.berat) ))
                    (tricolumn_colon("SpO2",format!("{}%",soap.spo2) ))
                    (tricolumn_colon("GCS(E,V,M)",&soap.gcs ))
                    (tricolumn_colon("Kesadaran",&soap.kesadaran ))
                    (tricolumn_colon("Lingkar Perut",format!("{} cm",&soap.lingkar_perut) ))
                }

            }
            (tricolumn_colon("Alergi",&soap.alergi ))
            (tricolumn_colon("Asesmen",&soap.penilaian ))
            (tricolumn_colon("Plan",&soap.rtl ))
            (tricolumn_colon("Instruksi",&soap.instruksi ))
        }
    }
}
