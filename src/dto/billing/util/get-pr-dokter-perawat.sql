select 
  jns_perawatan.nm_perawatan as "nama_perawatan!",
  rawat_jl_drpr.biaya_rawat as "biaya!",
  count(rawat_jl_drpr.kd_jenis_prw) as qty,
  sum(rawat_jl_drpr.biaya_rawat) as "total!"
from rawat_jl_drpr inner join jns_perawatan on rawat_jl_drpr.kd_jenis_prw=jns_perawatan.kd_jenis_prw
where rawat_jl_drpr.no_rawat = ?
group by jns_perawatan.nm_perawatan;
