use maud::{html, Markup};

use crate::{dto::hasil_usg::HasilUsg, report::molecule::tricolumn_colon};

use super::TemplateContext;

pub fn render_hasil_usg(context: &TemplateContext, usg: &HasilUsg) -> Markup {
    html! {
        .hasil-usg.section{
            .hasil-usg__title.section__title{ "Hasil USG" }
            .hasil-usg__body.section__body {
                (tricolumn_colon("Tanggal", usg.tanggal.as_ref().map(|i|i.format("%d-%m-%Y %H:%M").to_string())))
                (tricolumn_colon("Dokter", &usg.nm_dokter))
                (tricolumn_colon("Kiriman Dari", &usg.kiriman_dari))
                (tricolumn_colon("Jenis Prestasi", &usg.jenis_prestasi))
                (tricolumn_colon("HTA", &usg.hta))
                (tricolumn_colon("Diagnosa Klinis", &usg.diagnosa_klinis))
                @if let Some(ref gambar) = usg.gambar_usg {img src=(format!("{}/hasilpemeriksaanusg/{gambar}",context.config.resource_location));}
                (tricolumn_colon("Ukuran Kantong Gestasi (GS)", &usg.kantong_gestasi))
                (tricolumn_colon("Ukuran Bokong - Kepala (CRL)", &usg.ukuran_bokongkepala))
                (tricolumn_colon("Diameter Biparietal (DBP)", &usg.diameter_biparietal))
                (tricolumn_colon("Panjang Femur (FL)", &usg.panjang_femur))
                (tricolumn_colon("Lingkar Abdomen (AC)", &usg.lingkar_abdomen))
                (tricolumn_colon("Tafsiran Berat Janin (TBJ)", &usg.tafsiran_berat_janin))
                (tricolumn_colon("Usia Kehamilan", &usg.usia_kehamilan))
                (tricolumn_colon("Plasenta Berimplatansi di", &usg.plasenta_berimplatansi))
                (tricolumn_colon("Derajat Maturitas Plasenta", &usg.derajat_maturitas))
                (tricolumn_colon("Jumlah Air Ketuban", &usg.jumlah_air_ketuban))
                (tricolumn_colon("Peluang Sex", &usg.peluang_sex))
                (tricolumn_colon("Indeks Cairan Ketuban (ICK)", &usg.indek_cairan_ketuban))
                (tricolumn_colon("Kelainan Kongenital Mayor", &usg.kelainan_kongenital))
                (tricolumn_colon("Kesimpulan", &usg.kesimpulan))
            }
        }
    }
}
