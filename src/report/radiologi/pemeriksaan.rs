use maud::{html, Markup, Render};

use crate::{
    dto::radiologi::BasePemeriksaanRadiologi,
    report::{molecule::tricolumn_colon, TemplateContext},
};

pub fn render_pemeriksaan_radiologi_list(
    context: &TemplateContext,
    rads: &[BasePemeriksaanRadiologi],
) -> Markup {
    if rads.is_empty() {
        return "".render();
    }
    html! {
        .radiologi-pemeriksaan-list style=(format!("--row-length: {}",rads.len()+1)){
            .radiologi-pemeriksaan-list__title {
                "Pemeriksaan"
            }
            .radiologi-pemeriksaan-list__item {
                @for i in rads {
                    (render_pemeriksaan_radiologi(context, i))
                }
            }
        }
    }
}
fn render_pemeriksaan_radiologi(
    _context: &TemplateContext,
    rad: &BasePemeriksaanRadiologi,
) -> Markup {
    html! {
        .radiologi-pemeriksaan {
            (tricolumn_colon("Tanggal",rad.tanggal_periksa.format("%d-%m-%Y").to_string()))
            (tricolumn_colon("Dokter PJ",&rad.nama_dokter))
            (tricolumn_colon("Petugas",&rad.nama_petugas))
            .radiologi-pemeriksaan__pemeriksaan {
               .radiologi-pemeriksaan__pemeriksaan-title {"Pemeriksaan"}
               .radiologi-pemeriksaan__pemeriksaan-body {
                    (tricolumn_colon("Nama", &rad.nama_perawatan))
                    (tricolumn_colon("Proyeksi", &rad.proyeksi))
                    (tricolumn_colon("kV", &rad.kv))
                    (tricolumn_colon("mAS", &rad.mas))
                    (tricolumn_colon("FFD", &rad.ffd))
                    (tricolumn_colon("BSF", &rad.bfs))
                    (tricolumn_colon("Inak", &rad.inak))
                    (tricolumn_colon("Jumlah Penyinaran", &rad.jumlah_penyinaran))
                    (tricolumn_colon("Dosis Radiasi", &rad.dosis))
                }
            }
        }
    }
}
