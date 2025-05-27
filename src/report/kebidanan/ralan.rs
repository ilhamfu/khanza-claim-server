use maud::{html, Markup};

use crate::{
    dto::assesmen_kebidanan::ralan::AssesmenKebidananRalan,
    report::{
        kebidanan::some_and_wrap,
        molecule::{block_display, some_and_not_strip, tricolumn_colon},
        TemplateContext,
    },
};

pub fn render_assesmen_kebidanan_ralan(
    _context: &TemplateContext,
    p: &AssesmenKebidananRalan,
) -> Markup {
    let kebiasaan = {
        let mut res = "Obat & Vitamin : ".to_owned();
        res.push_str(p.kebiasaan.as_deref().unwrap_or_default());
        push_some_and_wrap(&mut res, " (", ")", p.ket_kebiasaan.as_deref());
        res.push_str(", Merokok : ");
        res.push_str(p.kebiasaan1.as_deref().unwrap_or_default());
        push_some_and_wrap(&mut res, " (", " batang/hari)", p.ket_kebiasaan1.as_deref());
        res.push_str(", Alkohol : ");
        res.push_str(p.kebiasaan2.as_deref().unwrap_or_default());
        push_some_and_wrap(&mut res, " (", " gelas/hari)", p.ket_kebiasaan2.as_deref());
        res.push_str(", Obat Tidur/Narkoba : ");
        res.push_str(p.kebiasaan3.as_deref().unwrap_or_default());
        res
    };

    html! {
        .assesmen-bidan {
            .assesmen-bidan__title{ "Penilaian Awal Keperawatan Rawat Jalan Kandungan" }
            .assesmen-bidan__body{
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"Yang Melakukan Pengkajian"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Tanggal", p.tanggal.format("%d-%m-%Y %H:%M").to_string()))
                        (tricolumn_colon("Petugas", &p.nama))
                        (tricolumn_colon("Pemberi Informasi", &p.informasi))
                    }
                }
                .assesmen-bidan__section.assesmen-bidan__section--two {
                    .assesmen-bidan__section-title {"I. Keadaan Umum"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("TD", &p.td.as_deref().map(|e|format!("{e} mmHg"))))
                        (tricolumn_colon("Nadi", &p.nadi.as_deref().map(|e|format!("{e}x/menit"))))
                        (tricolumn_colon("RR", &p.rr.as_deref().map(|e|format!("{e}x/menit"))))
                        (tricolumn_colon("Suhu", &p.rr.as_deref().map(|e|format!("{e}°C"))))
                        (tricolumn_colon("GCS(E,V,M)", &p.gcs))

                    }
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Berat Badan", &p.bb.as_deref().map(|e|format!("{e} Kg"))))
                        (tricolumn_colon("Tinggi Badan", &p.tb.as_deref().map(|e|format!("{e} cm"))))
                        (tricolumn_colon("LILA", &p.rr.as_deref().map(|e|format!("{e} cm"))))
                        (tricolumn_colon("BMI", &p.bmi.as_deref().map(|e|format!("{e} Kg/m²"))))
                    }
                }

                .assesmen-bidan__section.assesmen-bidan__section--two {
                    .assesmen-bidan__section-title {"II. Pemeriksaan Kebidanan"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("TFU", &p.tfu.as_deref().map(|e|format!("{e} cm"))))
                        (tricolumn_colon("TBJ", &p.nadi))
                        (tricolumn_colon("Letak", &p.letak))
                        (tricolumn_colon("Presentasi", &p.presentasi))
                        (tricolumn_colon("Penurunan", &p.penurunan))
                    }
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Kontraksi/HIS", &p.his.as_deref().map(|e|format!("{e}x/10"))))
                        (tricolumn_colon("Kekuatan", &p.kekuatan))
                        (tricolumn_colon("Lamanya", &p.lamanya))
                        (tricolumn_colon("Gerak Janin per 30 menit, DJJ", all_option_help(p.bjj.as_deref(), p.ket_bjj.as_deref(), |a,b| format!("{a} {b}"))))
                    }
                }

                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"III. Pemeriksaan Kebidanan Penunjang"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Inspekulo", no_or_keterangan(p.inspekulo.as_deref(), p.ket_inspekulo.as_deref())))
                        (tricolumn_colon("Laboratorium", no_or_keterangan(p.lab.as_deref(), p.ket_lab.as_deref())))
                        (tricolumn_colon("CTG", no_or_keterangan(p.ctg.as_deref(), p.ket_ctg.as_deref())))
                        (tricolumn_colon("Lakmus", no_or_keterangan(p.lakmus.as_deref(), p.ket_lakmus.as_deref())))
                        (tricolumn_colon("USG", no_or_keterangan(p.usg.as_deref(), p.ket_usg.as_deref())))
                        (tricolumn_colon("Pemeriksaan Panggul", no_or_keterangan(p.panggul.as_deref(), p.panggul.as_deref())))
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"IV. Riwayat Kesehatan"}
                    .assesmen-bidan__section-body {
                        (block_display("Keluhan Utama", &p.keluhan_utama))
                        .block-display {
                            div {"Riwayat Menstruasi"}
                            div {
                                table style="width:100%" {
                                    tr {
                                        td { "Umur Menarche : " (p.umur.as_deref().unwrap_or_default()) " tahun"}
                                        td { "Lamanya : " (p.lamanya.as_deref().unwrap_or_default()) " hari" }
                                        td { "Jumlah : " (p.banyaknya.as_deref().unwrap_or_default()) " pembalut"}
                                    }
                                    tr {
                                        td { "Haid Terakhir : " (p.haid.as_deref().unwrap_or_default())}
                                        td colspan="2" { "Siklus : " (p.lamanya.as_deref().unwrap_or_default()) " hari " (some_and_wrap("(", p.ket_siklus.as_deref(), ")")) (some_and_wrap(", ", p.ket_siklus1.as_deref(), "")) }

                                    }
                                }
                            }
                        }
                        .block-display {
                            div {"Riwayat Perkawinan"}
                            div {
                                table style="width:100%" {
                                    tr {
                                        td { "Status Menikah"}
                                        td { " : " (p.kali.as_deref().unwrap_or_default()) " kali"}
                                    }
                                    tr {
                                        td { "Usia Perkawinan 1" }
                                        td { " : " (p.usia1.as_deref().unwrap_or_default()) " tahun, ket: " (p.ket1.as_deref().unwrap_or_default()) }
                                    }
                                    tr {
                                        td { "Usia Perkawinan 2" }
                                        td { " : " (p.usia2.as_deref().unwrap_or_default()) " tahun, ket: " (p.ket2.as_deref().unwrap_or_default()) }
                                    }
                                    tr {
                                        td { "Usia Perkawinan 3" }
                                        td { " : " (p.usia3.as_deref().unwrap_or_default()) " tahun, ket: " (p.ket3.as_deref().unwrap_or_default()) }
                                    }
                                }
                            }
                        }
                        .block-display {
                            div {"Riwayat Kehamilan Tetap"}
                            div {
                                table style="width:100%" {
                                    tr {
                                        td { "HPHT : " (p.hpht.as_ref().map(|e| e.format("%d-%m-%Y").to_string()).unwrap_or_default()) }
                                        td { "Usia Kehamilan : " (p.usia_kehamilan.as_deref().unwrap_or_default()) " mgg/bulan" }
                                        td { "TP : " (p.tp.as_ref().map(|e| e.format("%d-%m-%Y").to_string()).unwrap_or_default())}
                                    }
                                    tr {
                                        td { "Riwayat Imunisasi : " (p.imunisasi.as_deref().unwrap_or_default()) (some_and_wrap(" (", p.ket_imunisasi.as_deref(), "kali)")) }
                                        td { "G/P/A : " (some_and_not_strip(p.g.as_deref()).unwrap_or("-")) "/" (some_and_not_strip(p.p.as_deref()).unwrap_or("-")) "/"(some_and_not_strip(p.a.as_deref()).unwrap_or("-"))}
                                        td { "Hidup : " (p.hidup.as_deref().unwrap_or_default()) }
                                    }
                                }
                            }
                        }
                        table.riwayat-persalinan {
                            thead{
                                tr {
                                    td rowspan="2" { "No" }
                                    td rowspan="2" { "Tgl/Thn Persalinan" }
                                    td rowspan="2" { "Tempat Persalinan" }
                                    td rowspan="2" { "Usia Hamil" }
                                    td rowspan="2" { "Jenis Persalinan" }
                                    td rowspan="2" { "Penolong" }
                                    td rowspan="2" { "Penyulit" }
                                    td colspan="3" { "Anak" }
                                }
                                tr {
                                    td { "JK" }
                                    td { "BB" }
                                    td { "Keadaan" }
                                }
                            }
                            tbody{
                                @match p.riwayat_persalinan {
                                    Some(ref ee) if !ee.is_empty() => {
                                        @for (index,i) in ee.0.iter().enumerate(){
                                            tr {
                                                td {(index+1)}
                                                td {(i.tanggal_tahun)}
                                                td {(i.tempat_persalinan)}
                                                td {(i.usia_hamil)}
                                                td {(i.jenis_persalinan)}
                                                td {(i.penolong)}
                                                td {(i.penyulit)}
                                                td {(i.jk)}
                                                td {(i.bbpb)}
                                                td {(i.keadaan)}
                                            }
                                        }
                                    }
                                    _ => {
                                        tr.empty {
                                            td colspan="10" {"Belum ada riwayat persalinan"}
                                        }
                                    }
                                }
                            }
                        }

                        (block_display("Riwayat Ginekologi", &p.ginekologi))
                        (block_display("Riwayat Kebiasaan", kebiasaan))
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"V. Fungsional"}
                    .assesmen-bidan__section-body {
                        table style="grid-column:1/4;margin-left:0.25rem" {
                            tr {
                                td { "Alat Bantu : " (p.alat_bantu.as_deref().unwrap_or_default()) (some_and_wrap(" (", p.ket_bantu.as_deref(), ")"))}
                                td { "Prosthesa : " (p.prothesa.as_deref().unwrap_or_default()) (some_and_wrap(" (", p.ket_pro.as_deref(), ")"))}
                                td { "Aktivitas Sehari-hari (ADL) : " (p.adl.as_deref().unwrap_or_default()) }
                            }
                        }
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"VI. Riwayat Psiko-Sosial, Spiritual dan Budaya"}
                    .assesmen-bidan__section-body {
                        table style="grid-column:1/4;margin-left:0.25rem" {
                            tr {
                                td colspan="2" {"Status Psikologis"}
                                td {" : " (p.status_psiko.as_deref().unwrap_or_default()) (some_and_wrap(" (", p.ket_psiko.as_deref(), ")"))}
                            }
                            tr {
                                td colspan="3" {"Status Sosial & Ekonomi"}
                            }
                            tr {
                                td style="padding-left:1rem" {"a."}
                                td { "Hubungan pasien dengan anggota keluarga"}
                                td {" : " (p.hub_keluarga.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td style="padding-left:1rem" {"b."}
                                td {"Tempat dengan"}
                                td {" : " (p.tinggal_dengan.as_deref().unwrap_or_default()) (some_and_wrap(" (",p.ket_tinggal.as_deref(),")"))}
                            }
                            tr {
                                td style="padding-left:1rem" {"c."}
                                td {"Ekonomi"}
                                td {" : " (p.ekonomi.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td colspan="2" {"Kepercayaan / Budaya / Nilai-nilai khusus yang perlu diperhatikan"}
                                td {" : " (p.budaya.as_deref().unwrap_or_default()) (some_and_wrap(" (",p.ket_budaya.as_deref(),")"))}
                            }
                            tr {
                                td colspan="2" {"Edukasi diberikan kepada"}
                                td {" : " (p.edukasi.as_deref().unwrap_or_default()) (some_and_wrap(" (",p.ket_edukasi.as_deref(),")"))}
                            }
                        }
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"VII. Penilaian Risiko Jatuh"}
                    .assesmen-bidan__section-body {
                        table style="grid-column:1/4;margin-inline:0.25rem;vertical-align:top;" {
                            tr {
                                td rowspan="3" style="vertical-align:top" { "1." }
                                td colspan="3" { "Cara Berjalan" }
                            }
                            tr {
                                td style="vertical-align:top"{ "a." }
                                td { "Tidak seimbang / sempoyongan / limbung" }
                                td style="vertical-align:top;white-space:nowrap"{ " : " (p.berjalan_a.as_deref().unwrap_or_default()) }
                            }
                            tr {
                                td style="vertical-align:top"{ "b." }
                                td { "Jalan dengan menggunakan alat bantu (kruk, tripod, kursi roda, orang lain)" }
                                td style="vertical-align:top;white-space:nowrap"{ " : " (p.berjalan_b.as_deref().unwrap_or_default()) }
                            }
                            tr {
                                td style="vertical-align:top"{ "2." }
                                td colspan="2" { "Menopang saat akan duduk, tampak memegang pinggiran kursi atau meja /benda lain sebagai penopang" }
                                td style="vertical-align:top;white-space:nowrap"{ " : " (p.berjalan_c.as_deref().unwrap_or_default()) }
                            }
                        }
                        table style="grid-column:1/4;margin-inline:0.25rem;vertical-align:top;" {
                            tr {
                                td  { "Hasil" }
                                td { " : " (p.hasil.as_deref().unwrap_or_default()) }
                            }
                            tr {
                                td { "Dilaporkan Kepada Dokter" }
                                td {" : " (p.lapor.as_deref().unwrap_or_default()) (some_and_wrap(" (Jam Dilaporkan : ",p.ket_lapor.as_deref(),")"))}
                            }
                        }
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"VIII. Skrining Gizi"}
                    .assesmen-bidan__section-body {
                        table style="grid-column:1/4;margin-inline:0.25rem"{
                            tr {
                                td style="vertical-align:top"{ "1." }
                                td { "Apakah ada  penurunan berat badan yang tidak diinginkan selama enam bulan terakhir" }
                                td style="vertical-align:top;white-space:nowrap"{ " : " (p.sg1.as_deref().unwrap_or_default())  }
                                td style="vertical-align:top;padding-left:1rem" {(p.nilai1.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td style="vertical-align:top"{ "2." }
                                td { "Apakan nafsu makan berkurang kaerna tidak nafsu makan" }
                                td style="vertical-align:top;white-space:nowrap"{ " : " (p.sg2.as_deref().unwrap_or_default()) }
                                td style="vertical-align:top;padding-left:1rem" {(p.nilai2.as_deref().unwrap_or_default())}
                            }
                            tr style="font-weight:bold" {
                                td colspan="3"{"Hasil"}
                                td style="padding-left:1rem" {(p.total_hasil.as_deref().unwrap_or_default())}
                            }
                        }
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"IX. Penilaian Tingkat Nyeri"}
                    .assesmen-bidan__section-body {
                        table style="grid-column:1/4;margin-inline:0.25rem"{
                            tr {
                                td {"Tingkat Nyeri"}
                                td {" : " (p.nyeri.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td {"Waktu / Durasi"}
                                td {" : " (p.durasi.as_deref().unwrap_or_default()) " menit"}
                            }
                            tr {
                                td {"Penyebab"}
                                td {" : " (p.provokes.as_deref().unwrap_or_default()) (some_and_wrap(" (",p.ket_provokes.as_deref(),")"))}
                            }
                            tr {
                                td {"Kualitas"}
                                td {" : " (p.quality.as_deref().unwrap_or_default()) (some_and_wrap(" (",p.ket_quality.as_deref(),")"))}
                            }
                            tr {
                                td {"Severity"}
                                td {" : Skala Nyeri " (p.skala_nyeri.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td style="padding-left:1rem"{"Wilayah"}
                                td {" : " (p.lokasi.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td style="padding-left:1rem"{"Menyebar"}
                                td {" : " (p.menyebar.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td {"Nyeri Hilang bila"}
                                td {" : " (p.nyeri_hilang.as_deref().unwrap_or_default())}
                            }
                            tr {
                                td {"Diberitahukan Dokter"}
                                td {" : " (p.pada_dokter.as_deref().unwrap_or_default()) (some_and_wrap(" Jam : ",p.ket_quality.as_deref(),""))}
                            }
                        }
                    }
                }
            }
        }
    }
}

// Extra {{{
fn all_option_help(
    opt1: Option<&str>,
    opt2: Option<&str>,
    cb: impl Fn(&str, &str) -> String,
) -> Option<String> {
    Some(cb(opt1?, opt2?))
}

fn no_or_keterangan<'a>(status: Option<&'a str>, text: Option<&'a str>) -> Option<&'a str> {
    if status? != "Tidak" {
        text
    } else {
        Some("Tidak")
    }
}

fn push_some_and_wrap(base: &mut String, prefix: &str, postfix: &str, text: Option<&str>) {
    match text {
        Some(e) if !e.is_empty() && e != "-" => {
            base.push_str(prefix);
            base.push_str(e);
            base.push_str(postfix);
        }
        _ => {}
    };
}

// }}}
