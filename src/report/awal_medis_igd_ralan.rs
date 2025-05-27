use maud::{html, Markup};

use crate::{dto::assesmen_awal_igd::AssesmenAwalIGD, report::molecule::tricolumn_colon};

use super::TemplateContext;

pub fn render_awal_medis_ralan_igd(_context: &TemplateContext, data: &AssesmenAwalIGD) -> Markup {
    html! {
        .section.penilaian-awal-igd {
            .section__title { "Penilaian Awal Medis IGD" }
            .section__body.penilaian-awal-igd__body {
                .penilaian-awal-igd__info { "I. Yang Melakukan Pengkajian" }
                (tricolumn_colon("Tanggal", data.tanggal.format("%d-%m-%Y %H:%M").to_string()))
                (tricolumn_colon("Dokter", &data.nama_dokter))
                (tricolumn_colon("Anamnesis", &data.anamnesis))
                .penilaian-awal-igd__info { "II. Riwayat Kesehatan" }
                (tricolumn_colon("Keluhan Utama", &data.keluhan_utama))
                .penilaian-awal-igd__detail{
                    div {
                        (tricolumn_colon("Riwayat Penyakit Dahulu", &data.rpd))
                        (tricolumn_colon("Riwayat Penyakit Keluarga", &data.rpk))
                    }
                    div {
                        (tricolumn_colon("Riwayat Alergi", &data.alergi))
                        (tricolumn_colon("Riwayat Penggunaan Obat", &data.rpo))
                    }
                }
                .penilaian-awal-igd__info { "III. Pemeriksaan Fisik" }
                .penilaian-awal-igd__detail{
                    div {
                        (tricolumn_colon("Keadaan Umum", &data.keluhan_utama))
                        (tricolumn_colon("Berat Badan",format!("{} Kg" ,&data.bb)))
                        (tricolumn_colon("Suhu", format!("{} °C",&data.suhu)))
                        (tricolumn_colon("Gigi & Mulut", &data.gigi))
                        (tricolumn_colon("Genital & Anus", &data.genital))
                        (tricolumn_colon("Kesadaran", &data.kesadaran))
                        (tricolumn_colon("Tensi", format!("{} mmHg",&data.td)))
                        (tricolumn_colon("SpO₂", format!("{}%",&data.spo)))
                        (tricolumn_colon("Leher", &data.leher))
                        (tricolumn_colon("Ekstermitas", &data.ekstremitas))
                    }
                    div {
                        (tricolumn_colon("GCS(E,V,M)", &data.gcs))
                        (tricolumn_colon("Nadi", format!("{}x/menit",&data.nadi)))
                        (tricolumn_colon("Kepala", &data.kepala))
                        (tricolumn_colon("Thoraks", &data.thoraks))
                        (tricolumn_colon("Keterangan Fisik", &data.ket_fisik))
                        (tricolumn_colon("Tinggi Badan", format!("{} cm",&data.tb)))
                        (tricolumn_colon("RR", format!("{}x/menit",&data.rr)))
                        (tricolumn_colon("Mata", &data.mata))
                        (tricolumn_colon("Abdomen", &data.abdomen))
                    }
                }
                .penilaian-awal-igd__info { "IV. Status Lokalis" }
                (tricolumn_colon("Keterangan", &data.ket_lokalis))
                .penilaian-awal-igd__info { "V. Pemeriksaan Penunjang" }
                (tricolumn_colon("EKG", &data.ekg))
                (tricolumn_colon("Radiologi", &data.rad))
                (tricolumn_colon("Laborat", &data.lab))
                .penilaian-awal-igd__info { "VI. Diagnosis / Asesmen" }
                .penilaian-awal-igd__full{(data.diagnosis)}
                .penilaian-awal-igd__info { "VII. Tata Laksanan" }
                .penilaian-awal-igd__full{(data.tata)}
            }
        }
    }
}
