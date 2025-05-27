select
  jns_perawatan_lab.nm_perawatan as "nama_perawatan!",
  count(DISTINCT periksa_lab.kd_jenis_prw,periksa_lab.tgl_periksa,periksa_lab.jam) as qty,
  periksa_lab.biaya as "biaya!",
  sum(detail_periksa_lab.biaya_item) as "biaya_tambahan!"
from periksa_lab 
inner join jns_perawatan_lab on jns_perawatan_lab.kd_jenis_prw = periksa_lab.kd_jenis_prw 
inner join detail_periksa_lab on detail_periksa_lab.no_rawat = periksa_lab.no_rawat and detail_periksa_lab.kd_jenis_prw = periksa_lab.kd_jenis_prw
where periksa_lab.no_rawat = ? group by periksa_lab.kd_jenis_prw;
