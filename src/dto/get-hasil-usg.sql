select 
  hasil_pemeriksaan_usg.kd_dokter,
  dokter.nm_dokter,
  hasil_pemeriksaan_usg.diagnosa_klinis,
  hasil_pemeriksaan_usg.kiriman_dari,
  hasil_pemeriksaan_usg.hta,
  hasil_pemeriksaan_usg.kantong_gestasi,
  hasil_pemeriksaan_usg.ukuran_bokongkepala,
  hasil_pemeriksaan_usg.jenis_prestasi,
  hasil_pemeriksaan_usg.diameter_biparietal,
  hasil_pemeriksaan_usg.panjang_femur,
  hasil_pemeriksaan_usg.lingkar_abdomen,
  hasil_pemeriksaan_usg.tafsiran_berat_janin,
  hasil_pemeriksaan_usg.usia_kehamilan,
  hasil_pemeriksaan_usg.plasenta_berimplatansi,
  hasil_pemeriksaan_usg.derajat_maturitas,
  hasil_pemeriksaan_usg.jumlah_air_ketuban,
  hasil_pemeriksaan_usg.indek_cairan_ketuban,
  hasil_pemeriksaan_usg.kelainan_kongenital,
  hasil_pemeriksaan_usg.peluang_sex,
  hasil_pemeriksaan_usg.kesimpulan,
  hasil_pemeriksaan_usg.tanggal,
  hasil_pemeriksaan_usg_gambar.photo as gambar_usg
from hasil_pemeriksaan_usg 
inner join dokter on hasil_pemeriksaan_usg.kd_dokter=dokter.kd_dokter
left join hasil_pemeriksaan_usg_gambar on hasil_pemeriksaan_usg_gambar.no_rawat=hasil_pemeriksaan_usg.no_rawat
where hasil_pemeriksaan_usg.no_rawat=?
