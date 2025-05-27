SELECT 
  pemeriksaan_ralan.no_rawat,
  pemeriksaan_ralan.tgl_perawatan,
  pemeriksaan_ralan.jam_rawat,
  pemeriksaan_ralan.suhu_tubuh as "suhu_tubuh!",
  pemeriksaan_ralan.tensi as "tensi!",
  pemeriksaan_ralan.nadi as "nadi!",
  pemeriksaan_ralan.respirasi as "respirasi!",
  pemeriksaan_ralan.tinggi as "tinggi!",
  pemeriksaan_ralan.berat as "berat!",
  pemeriksaan_ralan.spo2 as "spo2!",
  pemeriksaan_ralan.gcs as "gcs!",
  pemeriksaan_ralan.kesadaran as "kesadaran!",
  pemeriksaan_ralan.keluhan as "keluhan!",
  pemeriksaan_ralan.pemeriksaan as "pemeriksaan!",
  pemeriksaan_ralan.alergi as "alergi!",
  pemeriksaan_ralan.lingkar_perut as "lingkar_perut!",
  pemeriksaan_ralan.rtl as "rtl!",
  pemeriksaan_ralan.penilaian as "penilaian!",
  pemeriksaan_ralan.instruksi as "instruksi!",
  pemeriksaan_ralan.evaluasi as "evaluasi!",
  pemeriksaan_ralan.nip as "kode_pegawai",
  pegawai.nama as "nama_pegawai"
from pemeriksaan_ralan
inner join pegawai on pegawai.nik = pemeriksaan_ralan.nip
where pemeriksaan_ralan.no_rawat = ?
order by pemeriksaan_ralan.tgl_perawatan asc, pemeriksaan_ralan.jam_rawat asc
