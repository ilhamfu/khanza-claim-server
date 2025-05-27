select
  jns_perawatan.nm_perawatan as "nama_perawatan!",
  rawat_jl_dr.biaya_rawat as "biaya!",
  sum(rawat_jl_dr.biaya_rawat) as "total!",
  count(rawat_jl_dr.kd_jenis_prw)  as qty
from rawat_jl_dr 
inner join jns_perawatan on rawat_jl_dr.kd_jenis_prw=jns_perawatan.kd_jenis_prw
where rawat_jl_dr.no_rawat = ?
group by jns_perawatan.nm_perawatan;
