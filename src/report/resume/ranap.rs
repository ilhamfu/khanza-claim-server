use maud::{html, Markup};

use crate::{
    dto::resume::ranap::ResumeRanap,
    report::{
        molecule::{tricolumn_colon, OtherRenderable},
        TemplateContext,
    },
};

pub fn render_resume_ranap(_context: &TemplateContext, resume: &ResumeRanap) -> Markup {
    let prosedur = [
        (
            "Prosedur Utama",
            resume.kd_prosedur_utama.as_deref(),
            resume.prosedur_utama.as_deref(),
        ),
        (
            "Diagnosa Sekunder",
            resume.kd_prosedur_sekunder.as_deref(),
            resume.prosedur_sekunder.as_deref(),
        ),
        (
            "Diagnosa Sekunder 2",
            resume.kd_prosedur_sekunder2.as_deref(),
            resume.prosedur_sekunder2.as_deref(),
        ),
        (
            "Diagnosa Sekunder 3",
            resume.kd_diagnosa_sekunder3.as_deref(),
            resume.prosedur_sekunder3.as_deref(),
        ),
    ];
    let diagnosa_akhir = [
        (
            "Diagnosa Utama",
            resume.kd_diagnosa_utama.as_deref(),
            resume.diagnosa_utama.as_deref(),
        ),
        (
            "Diagnosa Sekunder",
            resume.kd_diagnosa_sekunder.as_deref(),
            resume.diagnosa_sekunder.as_deref(),
        ),
        (
            "Diagnosa Sekunder 2",
            resume.kd_diagnosa_sekunder2.as_deref(),
            resume.diagnosa_sekunder2.as_deref(),
        ),
        (
            "Diagnosa Sekunder 3",
            resume.kd_diagnosa_sekunder3.as_deref(),
            resume.diagnosa_sekunder3.as_deref(),
        ),
        (
            "Diagnosa Sekunder 4",
            resume.kd_diagnosa_sekunder4.as_deref(),
            resume.diagnosa_sekunder4.as_deref(),
        ),
    ];
    html! {
        .resume.section{
            .resume__title.section__title{
                "RESUME PERAWATAN"
            }
            .resume__body.section__body {
                (tricolumn_colon("Status","Rawat Inap"))
                (tricolumn_colon("Nama Dokter", &resume.nama_dokter))
                (tricolumn_colon("Keadaan",&[resume.keadaan.as_deref(),resume.ket_keadaan.as_deref()][..] ))
                (tricolumn_colon("Cara Keluar",&[resume.cara_keluar.as_deref(),resume.ket_keluar.as_deref()][..] ))
                (tricolumn_colon("Dilanjutkan",&[resume.dilanjutkan.as_deref(),resume.ket_dilanjutkan.as_deref()][..]))
                (tricolumn_colon("Tanggal Kontrol",&resume.kontrol.map(|e|e.format("%d-%m-%Y").to_string())))
            }
            .resume__detail-list {
                (render_detail("Alasan Masuk Dirawat : ", &resume.alasan))
                (render_detail("Keluhan Utama Riwayat Penyakit : ", &resume.keluhan_utama))
                (render_detail("Pemeriksaan Fisik : ", &resume.pemeriksaan_fisik))
                (render_detail("Jalannya Penyakit Selama Perawatan : ", &resume.jalannya_penyakit))
                (render_detail("Pemeriksaan Penunjang Rad Terpenting : ", &resume.pemeriksaan_penunjang))
                (render_detail("Pemeriksaan Penunjang Lab Terpenting ", &resume.hasil_laborat))
                (render_detail("Tindakan/Operasi Selama Perawatan : ", &resume.tindakan_dan_operasi))
                (render_detail("Obat-obatan Selama Perawatan : ", &resume.obat_rs))
                (render_details("Diagnosa Akhir", &diagnosa_akhir[..]))
                (render_details("Prosedur dan Tindakan Medis", &prosedur[..]))
                (render_detail("Alergi Obat : ", &resume.alergi))
                (render_detail("Diet : ", &resume.diet))
                (render_detail("Hasil Lab Pending : ", &resume.lab_belum))
                (render_detail("Instruksi/Anjuran dan Edukasi (Follow Up) : ", &resume.edukasi))
                (render_detail("Obat-Obatan Waktu Pulang : ", &resume.obat_pulang))
            }
        }
    }
}

fn render_detail<A>(l: &str, val: A) -> Markup
where
    A: OtherRenderable,
{
    html! {
        .resume__detail.section__body {
            .resume__detail-title{(l)}
            .resume__detail-body{
                .resume__detail-text{
                    (val.tricolumn_render())
                }
    }
        }
    }
}
fn render_details(l: &str, val: &[(&str, Option<&str>, Option<&str>)]) -> Markup {
    html! {
        .resume__detail.section__body {
            .resume__detail-title{(l)}
            .resume__detail-body{
                @for item in val {
                    .resume__detail-many {
                        div{ (item.0) }
                        div{ (item.1.tricolumn_render()) }
                        div{ (item.2.tricolumn_render()) }
                    }
                }

            }
        }
    }
}
