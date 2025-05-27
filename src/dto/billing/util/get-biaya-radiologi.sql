select
  jns_perawatan_radiologi.nm_perawatan as "nama_perawatan!",
  count(periksa_radiologi.kd_jenis_prw) as qty,
  periksa_radiologi.biaya as "biaya!",
  sum(periksa_radiologi.biaya) as "total!"
from periksa_radiologi
inner join jns_perawatan_radiologi on jns_perawatan_radiologi.kd_jenis_prw=periksa_radiologi.kd_jenis_prw
where periksa_radiologi.no_rawat=?
group by periksa_radiologi.kd_jenis_prw;
