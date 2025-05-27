select
  jns_perawatan_inap.nm_perawatan as "nama_perawatan!",
  rawat_inap_dr.biaya_rawat as "biaya!",
  sum(rawat_inap_dr.biaya_rawat) as "total!",
  count(rawat_inap_dr.kd_jenis_prw)  as qty
from rawat_inap_dr 
inner join jns_perawatan_inap on rawat_inap_dr.kd_jenis_prw=jns_perawatan_inap.kd_jenis_prw
where rawat_inap_dr.no_rawat = ?
group by jns_perawatan_inap.nm_perawatan;
