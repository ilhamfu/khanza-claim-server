use maud::{html, Markup, Render};

use crate::{
    dto::operasi::Operasi,
    report::molecule::{some_and_not_strip, tricolumn_colon},
};

use super::TemplateContext;

pub fn render_operasi_list(context: &TemplateContext, operasi_list: &[Operasi]) -> Markup {
    if operasi_list.is_empty() {
        return "".render();
    }
    html! {
        .operasi-list{
            .operasi-list__title{("Operasi / VK")}
            .operasi-list__body{
                @for i in operasi_list{
                    (render_operasi(context, &i))
                }
            }
        }
    }
}

fn render_operasi(_context: &TemplateContext, operasi: &Operasi) -> Markup {
    let operator = [
        operasi.nama_op_1.as_deref(),
        operasi.nama_op_2.as_deref(),
        operasi.nama_op_3.as_deref(),
    ]
    .map(|e| e.and_then(|item| if item == "-" { None } else { Some(item) }));
    let asisten_op = [
        operasi.nama_asop_1.as_deref(),
        operasi.nama_asop_2.as_deref(),
        operasi.nama_asop_3.as_deref(),
    ]
    .map(|e| e.and_then(|item| if item == "-" { None } else { Some(item) }));
    let asisten_anastesi = [
        operasi.nama_asisten_anesthesi_1.as_deref(),
        operasi.nama_asisten_anesthesi_2.as_deref(),
    ]
    .map(|e| e.and_then(|item| if item == "-" { None } else { Some(item) }));
    let bidan = [
        operasi.nama_bidan_1.as_deref(),
        operasi.nama_bidan_2.as_deref(),
        operasi.nama_bidan_3.as_deref(),
    ]
    .map(|e| e.and_then(|item| if item == "-" { None } else { Some(item) }));
    let onloop = [
        operasi.nama_onloop_1.as_deref(),
        operasi.nama_onloop_2.as_deref(),
        operasi.nama_onloop_3.as_deref(),
        operasi.nama_onloop_4.as_deref(),
        operasi.nama_onloop_5.as_deref(),
    ]
    .map(|e| e.and_then(|item| if item == "-" { None } else { Some(item) }));

    html! {
        .operasi {
            (tricolumn_colon("Tanggal & Waktu", operasi.tanggal_operasi.map(|item|item.format("%d-%m-%Y %H:%M").to_string()).unwrap_or_default()))
            (tricolumn_colon("Nama Operasi", &operasi.nama_perawatan))
            (tricolumn_colon("Anastesi", &operasi.jenis_anasthesi))
            .operasi--petugas {
                .operasi--petugas-title{"Petugas Operasi"}
                (tricolumn_colon("Operator", &operator[..]))
                (tricolumn_colon("Asisten Operator",&asisten_op[..]))
                (tricolumn_colon("Petugas Instrumen",some_and_not_strip(operasi.nama_instrumen.as_deref())))
                (tricolumn_colon("Dokter Anak",some_and_not_strip(operasi.nama_dokter_anak.as_deref())))
                (tricolumn_colon("Perawat Resusitas",some_and_not_strip(operasi.nama_perawat_resusitas.as_deref())))
                (tricolumn_colon("Dokter Anastesi",some_and_not_strip(operasi.nama_dokter_anasthesi.as_deref())))
                (tricolumn_colon("Asisten Anastesi",&asisten_anastesi[..]))
                (tricolumn_colon("Bidan",&bidan[..]))
                (tricolumn_colon("Perawat Luar",some_and_not_strip(operasi.nama_perawat_luar.as_deref())))
                (tricolumn_colon("Onloop",&onloop[..]))
                (tricolumn_colon("Dokter PJ Anak",some_and_not_strip(operasi.nama_dokter_pj_anak.as_deref())))
                (tricolumn_colon("Dokter Umum",some_and_not_strip(operasi.nama_dokter_anak.as_deref())))
            }
        }
    }
}
