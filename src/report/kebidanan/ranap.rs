use maud::{html, Markup};

use crate::{
    dto::assesmen_kebidanan::ranap::AssesmenKebidananRanap,
    report::{molecule::tricolumn_colon, TemplateContext},
};

pub fn render_assesmen_kebidanan_ranap(
    _context: &TemplateContext,
    p: &AssesmenKebidananRanap,
) -> Markup {
    html! {
        .assesmen-bidan {
            .assesmen-bidan__title{ "Penilaian Awal Keperawatan Rawat Inap Kandungan" }
            .assesmen-bidan__body{
                .assesmen-bidan__section.assesmen-bidan__section--two {
                    .assesmen-bidan__section-title {"Yang Melakukan Pengkajian"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Tanggal", p.tanggal.format("%d-%m-%Y %H:%M").to_string()))
                        (tricolumn_colon("Tiba di Ruang Rawat", &p.tiba_diruang_rawat))
                        (tricolumn_colon("Pengkaji", &[p.pengkaji1.as_deref(),p.pengkaji2.as_deref()][..]))
                        (tricolumn_colon("DPJP", &p.nm_dokter))
                    }
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Anamnesa", &p.informasi))
                        (tricolumn_colon("Riwayat Pembedahan", &p.cara_masuk))
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"I. Riwayat Kesehatan"}
                    .assesmen-bidan__section-body {
                        table style="margin-inline:0.25rem;grid-column:1/4"{
                            tr {
                                td colspan="2"  { "Keluhan Utama" }
                            }
                            tr {
                                td colspan="2" style="padding-left:1rem"{ (p.keluhan) }
                            }
                            tr {
                                td colspan="2"{ "Penyakit Selama Kehamilan" }
                            }
                            tr {
                                td colspan="2" style="padding-left:1rem"{ (p.psk) }
                            }
                            tr {
                                td {
                                    table {
                                        tr {
                                            td { "Riwayat Penyakit Keluarga" }
                                            td { " : " (p.rpk) }
                                        }
                                        tr {
                                            td { "Riwayat Alergi" }
                                            td { " : " (p.alergi) }
                                        }
                                    }
                                }
                                td {
                                    table {
                                        tr {
                                            td { "Riwayat Pembedahan" }
                                            td { " : " (p.rp) }
                                        }
                                        tr {
                                            td { "Komplikasi Kehamilan Sebelumnya" }
                                            td { " : " (p.komplikasi_sebelumnya) ", " (p.keterangan_komplikasi_sebelumnya)}
                                        }
                                    }
                                }
                            }
                        }
                        .block-display {
                            div { "Riwayat Menstruasi" }
                            div style="margin-left:1rem;display:grid;grid-template-columns:auto auto 1fr"{
                                (tricolumn_colon("Umur menarche", format!("{} tahun",p.riwayat_mens_umur)))
                                (tricolumn_colon("Lamanya", format!("{} hari",p.riwayat_mens_lamanya)))
                                (tricolumn_colon("Banyaknya", format!("{} pembalut",p.riwayat_mens_banyaknya)))
                                (tricolumn_colon("Siklus", format!("{} hari ({})",p.riwayat_mens_siklus,p.riwayat_mens_ket_siklus)))
                                (tricolumn_colon("Dirasakan saat Menstruasi", &p.riwayat_mens_dirasakan))
                            }
                        }
                        .block-display {
                            div { "Riwayat Persalinan" }
                            div style="margin-left:1rem;display:grid;grid-template-columns:auto auto 1fr"{
                                (tricolumn_colon("G/P/A", format!("{}/{}/{}",p.riwayat_persalinan_g,p.riwayat_persalinan_p,p.riwayat_persalinan_a)))
                                (tricolumn_colon("Anak yang Hidup", format!("{}",p.riwayat_persalinan_hidup)))
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

                            }
                        }
                        .block-display {
                            div { "Riwayat Hamil Sekarang" }
                            div style="margin-left:1rem;display:grid;grid-template-columns:auto auto 1fr"{
                                (tricolumn_colon("HPHT", p.riwayat_hamil_hpht.as_ref().map(|item|item.format("%d-%m-%Y").to_string()).unwrap_or_default()))
                                (tricolumn_colon("Usia Hamil", format!("{}",p.riwayat_hamil_usiahamil)))
                                (tricolumn_colon("Tanggal Perkiraan", p.riwayat_hamil_tp.as_ref().map(|item|item.format("%d-%m-%Y").to_string()).unwrap_or_default()))
                                (tricolumn_colon("Siklus", format!("{} hari ({})",p.riwayat_mens_siklus,p.riwayat_mens_ket_siklus)))
                                (tricolumn_colon("Dirasakan saat Menstruasi", &p.riwayat_mens_dirasakan))
                            }
                        }
                        (tricolumn_colon("Riwayat Ginekologi", &p.riwayat_genekologi))
                        .block-display {
                            div { "Riwayat Kebiasaan" }
                            div style="margin-left:1rem;display:grid;grid-template-columns:auto auto 1fr"{
                                (tricolumn_colon("Obat/Vitamin", format!("{} ({})",p.riwayat_kebiasaan_obat,p.riwayat_kebiasaan_ket_obat)))
                                (tricolumn_colon("Merokok", format!("{}, {} batang/hari",p.riwayat_kebiasaan_merokok,p.riwayat_kebiasaan_ket_merokok)))
                                (tricolumn_colon("Alkohol", format!("{}, {} gelas/hari",p.riwayat_kebiasaan_alkohol,p.riwayat_kebiasaan_ket_alkohol)))
                                (tricolumn_colon("Obat Tidur",&p.riwayat_kebiasaan_narkoba))
                            }
                        }
                    }
                }
                .assesmen-bidan__section.assesmen-bidan__section--two {
                    .assesmen-bidan__section-title {"II. Pemeriksaan Kebidanan"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Kesadaran Mental",&p.pemeriksaan_kebidanan_mental))
                        (tricolumn_colon("Keadaan Umum",&p.pemeriksaan_kebidanan_keadaan_umum))
                        (tricolumn_colon("GCS(E,V,M)",&p.pemeriksaan_kebidanan_gcs))
                        (tricolumn_colon("TD", format!("{} mmHg",&p.pemeriksaan_kebidanan_td)))
                        (tricolumn_colon("Nadi", format!("{}x/menit",&p.pemeriksaan_kebidanan_nadi)))
                        (tricolumn_colon("RR", format!("{}x/menit",&p.pemeriksaan_kebidanan_rr)))
                        (tricolumn_colon("Suhu", format!("{}°C",&p.pemeriksaan_kebidanan_td)))
                        (tricolumn_colon("SpO2", format!("{}%",&p.pemeriksaan_kebidanan_td)))
                        (tricolumn_colon("Kontraksi/HIS", format!("{}x/10",&p.pemeriksaan_kebidanan_his)))
                        (tricolumn_colon("Kekuatan", format!("{}x/10",&p.pemeriksaan_kebidanan_kekuatan)))
                        (tricolumn_colon("Lamanya", format!("{} detik",&p.pemeriksaan_kebidanan_lamanya)))
                        (tricolumn_colon("DJJ", format!("{}/menit ({})",&p.pemeriksaan_kebidanan_djj,p.pemeriksaan_kebidanan_ket_djj)))
                        (tricolumn_colon("Portion", &p.pemeriksaan_kebidanan_portio))
                        (tricolumn_colon("Pembukaan Serviks", format!("{} cm",&p.pemeriksaan_kebidanan_pembukaan)))
                    }
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("BB", format!("{} Kg",&p.pemeriksaan_kebidanan_bb)))
                        (tricolumn_colon("TB", format!("{} cm",&p.pemeriksaan_kebidanan_tb)))
                        (tricolumn_colon("LILA", format!("{} cm",&p.pemeriksaan_kebidanan_lila)))
                        (tricolumn_colon("TFU", format!("{} cm",&p.pemeriksaan_kebidanan_tfu)))
                        (tricolumn_colon("TBJ", format!("{} g",&p.pemeriksaan_kebidanan_tbj)))
                        (tricolumn_colon("Letak", &p.pemeriksaan_kebidanan_letak))
                        (tricolumn_colon("Presentasi", &p.pemeriksaan_kebidanan_presentasi))
                        (tricolumn_colon("Penurunan", &p.pemeriksaan_kebidanan_penurunan))
                        (tricolumn_colon("Ketuban", format!("{} keb/bok",&p.pemeriksaan_kebidanan_ketuban)))
                        (tricolumn_colon("Hodge", &p.pemeriksaan_kebidanan_hodge))
                        (tricolumn_colon("Panggul", &p.pemeriksaan_kebidanan_panggul))
                        (tricolumn_colon("Inspekulo", format!("{},{}",&p.pemeriksaan_kebidanan_inspekulo,&p.pemeriksaan_kebidanan_ket_inspekulo)))
                        (tricolumn_colon("Lakmus", &p.pemeriksaan_kebidanan_lakmus))
                        (tricolumn_colon("CTG", &p.pemeriksaan_kebidanan_ctg))
                    }
                }
                .assesmen-bidan__section.assesmen-bidan__section--two {
                    .assesmen-bidan__section-title {"III. Pemeriksaan Umum"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Kepala",&p.pemeriksaan_umum_kepala))
                        (tricolumn_colon("Muka",&p.pemeriksaan_umum_muka))
                        (tricolumn_colon("Mata",&p.pemeriksaan_umum_mata))
                        (tricolumn_colon("Hidung",&p.pemeriksaan_umum_hidung))
                        (tricolumn_colon("Telinga",&p.pemeriksaan_umum_telinga))
                        (tricolumn_colon("Mulut",&p.pemeriksaan_umum_mulut))
                    }
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Leher",&p.pemeriksaan_umum_leher))
                        (tricolumn_colon("Dada",&p.pemeriksaan_umum_dada))
                        (tricolumn_colon("Perut",&p.pemeriksaan_umum_perut))
                        (tricolumn_colon("Genitalia",&p.pemeriksaan_umum_genitalia))
                        (tricolumn_colon("Ekxtremitas",&p.pemeriksaan_umum_ekstrimitas))
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"IV. Pengkajian Fungsi"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Kemampuan Aktifitas Sehari-hari",&p.pengkajian_fungsi_kemampuan_aktifitas))
                        (tricolumn_colon("Berjalan",format!("{}, {}",&p.pengkajian_fungsi_berjalan,p.pengkajian_fungsi_ket_berjalan)))
                        (tricolumn_colon("Aktifitas",&p.pengkajian_fungsi_aktivitas))
                        (tricolumn_colon("Alat Ambulasi",&p.pengkajian_fungsi_ambulasi))
                        (tricolumn_colon("Ekstremitas Atas",format!("{}, {}",&p.pengkajian_fungsi_ekstrimitas_atas,&p.pengkajian_fungsi_ket_ekstrimitas_atas)))
                        (tricolumn_colon("Ekstremitas Bawah",format!("{}, {}",&p.pengkajian_fungsi_ekstrimitas_bawah,&p.pengkajian_fungsi_ket_ekstrimitas_bawah)))
                        (tricolumn_colon("Kemampuan Menggenggam",format!("{}, {}",&p.pengkajian_fungsi_kemampuan_menggenggam,&p.pengkajian_fungsi_ket_kemampuan_menggenggam)))
                        (tricolumn_colon("Kemampuan Koordinasi",format!("{}, {}",&p.pengkajian_fungsi_koordinasi,&p.pengkajian_fungsi_ket_koordinasi)))
                        (tricolumn_colon("Kesimpulan Gangguang Fungsi",&p.pemeriksaan_umum_mulut))
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"V. Riwayat Psikologis, Sosial, Ekonomi, Budaya, Spiritual"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Kondisi Psikologis",&p.riwayat_psiko_kondisipsiko))
                        (tricolumn_colon("Adakah Perilaku",format!("{}, {}",&p.riwayat_psiko_adakah_prilaku,&p.riwayat_psiko_ket_adakah_prilaku)))
                        (tricolumn_colon("Gangguan Jiwa di Masa Lalu",&p.riwayat_psiko_gangguan_jiwa))
                        (tricolumn_colon("Hubungan Pasien dengan Anggota Keluarga",&p.riwayat_psiko_hubungan_pasien))
                        (tricolumn_colon("Tinggal Dengan",format!("{}, {}",&p.riwayat_psiko_tinggal_dengan,p.riwayat_psiko_ket_tinggal_dengan)))
                        (tricolumn_colon("Nilai-nilai Kepercayaan/ Budaya yang Perlu Diperhatikan",format!("{}, {}",&p.riwayat_psiko_budaya,p.riwayat_psiko_ket_budaya)))
                        (tricolumn_colon("Pendidikan P.J",&p.riwayat_psiko_pend_pj))
                        (tricolumn_colon("Edukasi diberikan Kepada",format!("{}, {}",&p.riwayat_psiko_edukasi_pada,p.riwayat_psiko_ket_edukasi_pada)))
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"VI. Penilaian Tingkat Nyeri"}
                    .assesmen-bidan__section-body {
                        (tricolumn_colon("Tingkat Nyeri",format!("{}, Waktu / Durasi : {} menit",&p.penilaian_nyeri,p.penilaian_nyeri_waktu)))
                        (tricolumn_colon("Penyebab",format!("{}, {}",&p.penilaian_nyeri_penyebab,&p.penilaian_nyeri_ket_penyebab)))
                        (tricolumn_colon("Kualitas",format!("{}, {}",p.penilaian_nyeri_kualitas,p.penilaian_nyeri_ket_kualitas)))
                        (tricolumn_colon("Severity",format!("Skala Nyeri {}",p.penilaian_nyeri_skala)))
                        (tricolumn_colon("Wilayah",&p.penilaian_nyeri_lokasi))
                        (tricolumn_colon("Menyebar",&p.penilaian_nyeri_menyebar))
                        (tricolumn_colon("Nyeri Bilang Bila",format!("{}, {}",p.penilaian_nyeri_hilang,p.penilaian_nyeri_ket_hilang)))
                        (tricolumn_colon("Diberitahukan ke Dokter",format!("{}, Jam : {}",p.penilaian_nyeri_diberitahukan_dokter,p.penilaian_nyeri_jam_diberitahukan_dokter)))
                    }
                }
                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"VII. Penilaian Risiko Jatuh"}
                    .assesmen-bidan__section-body {
                        table style="grid-column:1/4;margin-inline:0.25rem" {
                            tr {
                                td {"1"}
                                td {"Riwayat Jatuh"}
                                td style="text-align:center"{(p.penilaian_jatuh_skala1)}
                                td style="text-align:center"{(p.penilaian_jatuh_nilai1)}
                            }
                            tr {
                                td {"2"}
                                td {"Diagnosis Sekunder (≥ 2 Diagnosis Medis)"}
                                td style="text-align:center"{(p.penilaian_jatuh_skala2)}
                                td style="text-align:center"{(p.penilaian_jatuh_nilai2)}
                            }
                            tr {
                                td {"3"}
                                td {"Alat Bantu"}
                                td style="text-align:center"{(p.penilaian_jatuh_skala3)}
                                td style="text-align:center"{(p.penilaian_jatuh_nilai3)}
                            }
                            tr {
                                td {"4"}
                                td {"Terpasang Infuse"}
                                td style="text-align:center"{(p.penilaian_jatuh_skala4)}
                                td style="text-align:center"{(p.penilaian_jatuh_nilai4)}
                            }
                            tr {
                                td {"5"}
                                td {"Gaya Berjalan"}
                                td style="text-align:center"{(p.penilaian_jatuh_skala5)}
                                td style="text-align:center"{(p.penilaian_jatuh_nilai5)}
                            }
                            tr {
                                td {"6"}
                                td {"Status Mental"}
                                td style="text-align:center"{(p.penilaian_jatuh_skala6)}
                                td style="text-align:center"{(p.penilaian_jatuh_nilai6)}
                            }
                            tr style="font-weight:bold" {
                                td colspan="3" style="text-align:right"{ "Hasil" }
                                td style="text-align:center"{(p.penilaian_jatuh_totalnilai)}
                            }
                            tr style="font-weight:bold" {
                                td colspan="4" style="text-align:center"{
                                    @match p.penilaian_jatuh_totalnilai {
                                        e if e < 25f64 => "Resiko Rendah (0-24), Tindakan Intervensi pencegahan risiko jatuh standar"
                                        e if e < 45f64 => "Tingkat Resiko : Risiko Sedang (25-44), Tindakan : Intervensi pencegahan risiko jatuh standar"
                                        _ => "Tingkat Resiko : Risiko Tinggi (> 45), Tindakan : Intervensi pencegahan risiko jatuh standar dan Intervensi risiko jatuh tinggi"
                                    }
                                }
                            }
                        }
                    }
                }

                .assesmen-bidan__section {
                    .assesmen-bidan__section-title {"VIII. Skrining GIzi"}
                    .assesmen-bidan__section-body {
                        table style="grid-column:1/4;margin-inline:0.25rem" {
                            tr {
                                td {"1"}
                                td {"Apakah ada penurunan BB yang tidak diinginkan selama 6 bulan terakhir"}
                                td style="text-align:center"{(p.skrining_gizi1)}
                                td style="text-align:center"{(p.nilai_gizi1)}
                            }
                            tr {
                                td {"2"}
                                td {"Apakah asupan makan berkurang karena tidak nafsu makan"}
                                td style="text-align:center"{(p.skrining_gizi2)}
                                td style="text-align:center"{(p.nilai_gizi2)}
                            }
                            tr {
                                td {"6"}
                                td {"Status Mental"}
                                td style="text-align:center"{(p.penilaian_jatuh_skala6)}
                                td style="text-align:center"{(p.penilaian_jatuh_nilai6)}
                            }
                            tr style="font-weight:bold" {
                                td colspan="3" style="text-align:left"{ "Total skor:" }
                                td style="text-align:center"{(p.nilai_total_gizi)}
                            }
                        }
                        (tricolumn_colon("Pasien dengan diagnosa khusus", format!("{}, {}",p.skrining_gizi_diagnosa_khusus,p.skrining_gizi_ket_diagnosa_khusus)))
                        (tricolumn_colon("Sudah dibaca dan diketahui Dietisen", format!("{}, Jam : {}",p.skrining_gizi_diketahui_dietisen,p.skrining_gizi_jam_diketahui_dietisen)))
                    }
                }
            }
        }
    }
}
