select 
  rp.no_rawat as "no_rawat!",
  rp.tgl_registrasi as "tgl_registrasi!",
  rp.jam_reg as "jam_registrasi!",
  rp.no_rkm_medis as "no_rm!",
  rp.umurdaftar as "umur_daftar!",
  rp.sttsumur as "status_umur!",
  p.nm_pasien as "nama_pasien!",
  p.jk as "jk!",
  p.tgl_lahir as "tanggal_lahir!",
  rp.kd_dokter as "kode_dokter!",
  d.nm_dokter as "nama_dokter!",
  rp.kd_poli as "kode_poli!",
  pl.nm_poli as "nama_poli!",
  rp.kd_pj as "kode_pj!",
  pj.png_jawab as "nama_pj!",
  rp.status_lanjut as "status_lanjut!",
  rp.stts as "status!"
from reg_periksa rp
inner join pasien p on p.no_rkm_medis = rp.no_rkm_medis
inner join dokter d on d.kd_dokter = rp.kd_dokter
inner join poliklinik pl on pl.kd_poli = rp.kd_poli
inner join penjab pj on pj.kd_pj = rp.kd_pj
where rp.no_rawat = ?
