mod berkas_digital;

mod awal_medis_igd_ralan;
mod billing;
mod hasil_usg;
mod kebidanan;
mod lab;
mod laporan_operasi;
mod molecule;
mod operasi;
mod radiologi;
mod resume;
mod rujuk_penuh;
mod sep;
mod signature;
mod soap;
mod spri;
mod triase;

use awal_medis_igd_ralan::render_awal_medis_ralan_igd;
use berkas_digital::render_berkas_digital_list;
use billing::render_billing;
use hasil_usg::render_hasil_usg;
use kebidanan::render_assesmen_kebidanan;
use lab::render_lab_list;
use laporan_operasi::render_laporan_operasi_list;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};
use molecule::tricolumn_colon;
use operasi::render_operasi_list;
use radiologi::render_radiologi;
use rujuk_penuh::render_rujuk_penuh;
use sep::render_sep;
use signature::render_signature;
use soap::render_soap_list;
use spri::render_spri;
use triase::render_triase;

use crate::{
    config::AppConfig,
    dto::{reg_periksa::RegPeriksa, DetailRawat},
};

pub struct TemplateContext<'a> {
    pub config: &'a AppConfig,
}

pub fn render_report(context: &TemplateContext, detail: &DetailRawat) -> Markup {
    let title = format!(
        "{} - {}",
        detail.sep.no_rm.as_deref().unwrap_or_default(),
        detail.sep.no_sep
    );

    html! {
        (DOCTYPE)
        html{
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                link rel="stylesheet" href="../dist/style.css";
            }
            body{
                (render_sep(&context,&detail.sep))
                (detail.spri.as_ref().map(|a|render_spri(context,a)).unwrap_or_default())
                (detail.rujukan_penuh.as_ref().map(|a|render_rujuk_penuh(context, a)).unwrap_or_default())
                (render_base(&context,&detail.reg_periksa))
                (render_soap_list(context,&detail.soap))
                (detail.assesmen_igd.as_ref().map(|item|render_awal_medis_ralan_igd(context, item)).unwrap_or_default())
                (render_triase(context, &detail.triase))
                (render_assesmen_kebidanan(context, &detail.assesmen_kebidanan))
                (render_operasi_list(context, &detail.operasi))
                (render_laporan_operasi_list(context, &detail.laporan_operasi))
                (detail.hasil_usg.as_ref().map(|hasil_usg|render_hasil_usg(context, hasil_usg)).unwrap_or_default())
                (render_radiologi(context, &detail.radiologi))
                (render_lab_list(context, &detail.lab))
                (render_berkas_digital_list(context, &detail.berkas_digital))
                (render_billing(context, &detail.billing))
                (render_signature(context, &detail))
            }
        }
    }
}

pub fn render_base(_context: &TemplateContext, reg: &RegPeriksa) -> Markup {
    let masuk_kamar_inap = reg
        .waktu_ranap
        .as_ref()
        .map(|e| tricolumn_colon("Masuk Ranap", &e.waktu_masuk).render())
        .unwrap_or_default();
    let keluar_kamar_inap = reg
        .waktu_ranap
        .as_ref()
        .map(|e| tricolumn_colon("Pulang Ranap", &e.waktu_keluar).render())
        .unwrap_or_default();
    html! {
        .section {
            .section__title {"Detail Rawat"}
            .section__body.basic-detail {
                (tricolumn_colon("No. Rawat", &reg.no_rawat))
                (tricolumn_colon("No. RM", &reg.no_rm))
                (tricolumn_colon("Tanggal Registrasi", reg.tgl_registrasi.format("%d-%m-%Y").to_string()))
                (tricolumn_colon("Umur saat Daftar", format!("{} {}",reg.umur_daftar,reg.status_umur)))
                (tricolumn_colon("Unit/Poliklinik", &reg.nama_poli))
                (tricolumn_colon("Dokter Poli", &reg.nama_dokter))
                (masuk_kamar_inap)
                (keluar_kamar_inap)
            }
        }
    }
}
