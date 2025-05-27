use maud::{html, Markup, Render};

use crate::{
    dto::triase::{triase_sekunder::TriaseSekunder, triase_utama::TriaseUtama, Triase},
    report::molecule::tricolumn_colon,
};

use super::TemplateContext;

pub fn render_triase(context: &TemplateContext, triase: &Triase) -> Markup {
    if triase.sekunder.is_none() && triase.primer.is_none() {
        return "".render();
    }
    html! {
        .triase.section
        .triase--primer[triase.primer.is_some()]
        .triase--sekunder[triase.sekunder.is_some()]
        {
            .triase__title.section__title { "DATA TRIASE" }
            .triase__body.section__body {
                @if let Some(ref primer) = triase.primer {
                    (render_triase_primary(context, primer))
                }
                @if let Some(ref sekunder) = triase.sekunder {
                    (render_triase_sekunder(context, sekunder))
                }
            }

        }
    }
}

fn render_triase_primary(_context: &TemplateContext, triase_primer: &TriaseUtama) -> Markup {
    let tanda_vital = format!("Suhu (C): {}, Nyeri : {}, Tensi : {}, Nadi/Menit : {}, Saturasi O2: {}, Respirasi/menit: {}",
        triase_primer.suhu.as_deref().unwrap_or_default(),
        triase_primer.nyeri.as_deref().unwrap_or_default(),
        triase_primer.tekanan_darah.as_deref().unwrap_or_default(),
        triase_primer.nadi.as_deref().unwrap_or_default(),
        triase_primer.saturasi_o2.as_deref().unwrap_or_default(),
        triase_primer.pernapasan.as_deref().unwrap_or_default()
    );
    html! {
        (tricolumn_colon("Cara Masuk",&triase_primer.cara_masuk))
        (tricolumn_colon("Transportasi",&triase_primer.alat_transportasi))
        (tricolumn_colon("Alasan Kedatangan",&triase_primer.alasan_kedatangan))
        (tricolumn_colon("Macam Kasus",&triase_primer.macam_kasus))
        .triase__info { "Triase Primer" }
        (tricolumn_colon("Keluhan Utama",&triase_primer.keluhan_utama))
        (tricolumn_colon("Tanda Vital", tanda_vital))
        (tricolumn_colon("Kebutuhan Khusus", &triase_primer.kebutuhan_khusus))
        @if let Some(ref skala1) = triase_primer.pemeriksaan.skala1 {
            .triase__pemeriksaan.triase__pemeriksaan--immediate {
                (tricolumn_colon("Pemeriksaan", "Immediate/Segera"))
                .triase__subpemeriksaan {
                    @for i in skala1{
                        (tricolumn_colon(&i.nama_pemeriksaan, &i.pengkajian[..]))
                    }
                }
                (tricolumn_colon("Plan/Keputusan", &triase_primer.plan))
            }
        }
        @if let Some(ref skala2) = triase_primer.pemeriksaan.skala2 {
            .triase__pemeriksaan.triase__pemeriksaan--emergency {
                (tricolumn_colon("Pemeriksaan", "Emergensi"))
                .triase__subpemeriksaan {
                    @for i in skala2{
                        (tricolumn_colon(&i.nama_pemeriksaan, &i.pengkajian[..]))
                    }
                    (tricolumn_colon("Plan/Keputusan", &triase_primer.plan))
                }
            }
        }
        .triase__info { "Petugas Triase Primer" }
        (tricolumn_colon("Tanggal & Jam", triase_primer.tanggal_triase.as_ref().map(|e|e.format("%d-%m-%Y %H:%M").to_string()).unwrap_or_default()))
        (tricolumn_colon("Catatan", &triase_primer.catatan))
        (tricolumn_colon("Dokter / Petugas Triase", &triase_primer.nama_pegawai))
    }
}

fn render_triase_sekunder(_context: &TemplateContext, triase: &TriaseSekunder) -> Markup {
    let tanda_vital = format!("Suhu : {} °C, Nyeri : {}, Tensi : {} mmHg, nadi : {}x/menit, Saturasi O₂: {} %, Respirasi: {}x/menit",
        triase.suhu.as_deref().unwrap_or_default(),
        triase.nyeri.as_deref().unwrap_or_default(),
        triase.tekanan_darah.as_deref().unwrap_or_default(),
        triase.nadi.as_deref().unwrap_or_default(),
        triase.saturasi_o2.as_deref().unwrap_or_default(),
        triase.pernapasan.as_deref().unwrap_or_default()
    );
    html! {
        (tricolumn_colon("Cara Masuk",&triase.cara_masuk))
        (tricolumn_colon("Transportasi",&triase.alat_transportasi))
        (tricolumn_colon("Alasan Kedatangan",&triase.alasan_kedatangan))
        (tricolumn_colon("Keterangan Kedatangan",&triase.keterangan_kedatangan))
        (tricolumn_colon("Macam Kasus",&triase.macam_kasus))
        .triase__info { "Triase Sekunder" }
        (tricolumn_colon("Anamnesa Singkat", &triase.anamnesa_singkat))
        (tricolumn_colon("Tanda Vital", tanda_vital))

        @if let Some(ref skala3) = triase.pemeriksaan.skala3 {
            .triase__pemeriksaan.triase__pemeriksaan--urgent {
                (tricolumn_colon("Pemeriksaan", "Urgensi"))
                .triase__subpemeriksaan {
                    @for i in skala3{
                        (tricolumn_colon(&i.nama_pemeriksaan, &i.pengkajian[..]))
                    }
                }
                (tricolumn_colon("Plan/Keputusan", &triase.plan))
            }
        }
        @if let Some(ref skala4) = triase.pemeriksaan.skala4 {
            .triase__pemeriksaan.triase__pemeriksaan--semi-urgent{
                (tricolumn_colon("Pemeriksaan", "Semi Urgensi"))
                .triase__subpemeriksaan {
                    @for i in skala4{
                        (tricolumn_colon(&i.nama_pemeriksaan, &i.pengkajian[..]))
                    }
                }
                (tricolumn_colon("Plan/Keputusan", &triase.plan))
            }
        }

        @if let Some(ref skala5) = triase.pemeriksaan.skala5 {
            .triase__pemeriksaan.triase__pemeriksaan--non-urgent {
                (tricolumn_colon("Pemeriksaan", "Non Urgensi"))
                .triase__subpemeriksaan {
                    @for i in skala5{
                        (tricolumn_colon(&i.nama_pemeriksaan, &i.pengkajian[..]))
                    }
                    (tricolumn_colon("Plan/Keputusan", &triase.plan))
                }
            }
        }
        .triase__info { "Petugas Triase Sekunder" }
        (tricolumn_colon("Tanggal & Jam", triase.tanggal_triase.as_ref().map(|e|e.format("%d-%m-%Y %H:%M").to_string()).unwrap_or_default()))
        (tricolumn_colon("Catatan", &triase.catatan))
        (tricolumn_colon("Dokter / Petugas Triase", &triase.nama_pegawai))
    }
}
