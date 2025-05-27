select 
  jns_perawatan_inap.nm_perawatan as "nama_perawatan!",
  rawat_inap_drpr.biaya_rawat as "biaya!",
  count(rawat_inap_drpr.kd_jenis_prw) as qty,
  sum(rawat_inap_drpr.biaya_rawat) as "total!"
from rawat_inap_drpr 
inner join jns_perawatan_inap on rawat_inap_drpr.kd_jenis_prw=jns_perawatan_inap.kd_jenis_prw
where rawat_inap_drpr.no_rawat = ?
group by jns_perawatan_inap.nm_perawatan;
