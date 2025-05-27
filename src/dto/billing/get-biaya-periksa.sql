select reg_periksa.biaya_reg from reg_periksa where reg_periksa.no_rawat = ?;

select
  jns_perawatan.nm_perawatan,
  rawat_jl_dr.biaya_rawat as total_byrdr,
  count(rawat_jl_dr.kd_jenis_prw) as jml,
  sum(rawat_jl_dr.biaya_rawat) as biaya
from rawat_jl_dr 
inner join jns_perawatan on rawat_jl_dr.kd_jenis_prw=jns_perawatan.kd_jenis_prw 
where rawat_jl_dr.no_rawat = ?
group by jns_perawatan.nm_perawatan;

select
  jns_perawatan.nm_perawatan,
  rawat_jl_pr.biaya_rawat as total_byrpr,
  count(rawat_jl_pr.kd_jenis_prw) as jml,
  sum(rawat_jl_pr.biaya_rawat) as biaya
from rawat_jl_pr inner join jns_perawatan on rawat_jl_pr.kd_jenis_prw=jns_perawatan.kd_jenis_prw 
where rawat_jl_pr.no_rawat = ?
group by jns_perawatan.nm_perawatan;

select 
  jns_perawatan.nm_perawatan,
  rawat_jl_drpr.biaya_rawat as total_byrdrpr,
  count(rawat_jl_drpr.kd_jenis_prw) as jml,
  sum(rawat_jl_drpr.biaya_rawat) as biaya
from rawat_jl_drpr inner join jns_perawatan on rawat_jl_drpr.kd_jenis_prw=jns_perawatan.kd_jenis_prw
where rawat_jl_drpr.no_rawat = ?
group by jns_perawatan.nm_perawatan;

select
  jns_perawatan_lab.nm_perawatan,
  jns_perawatan_lab.kd_jenis_prw,
  count(periksa_lab.kd_jenis_prw) as qty,
  periksa_lab.biaya as biaya,
  sum(detail_periksa_lab.biaya_item) as biaya_tambahan
from periksa_lab 
inner join jns_perawatan_lab on jns_perawatan_lab.kd_jenis_prw = periksa_lab.kd_jenis_prw 
inner join detail_periksa_lab on detail_periksa_lab.no_rawat = periksa_lab.no_rawat and detail_periksa_lab.kd_jenis_prw = periksa_lab.kd_jenis_prw
where periksa_lab.no_rawat=? group by periksa_lab.kd_jenis_prw;

select
  jns_perawatan_radiologi.nm_perawatan,
  count(periksa_radiologi.kd_jenis_prw) as jml,
  periksa_radiologi.biaya as biaya,
  sum(periksa_radiologi.biaya) as total
from periksa_radiologi
inner join jns_perawatan_radiologi on jns_perawatan_radiologi.kd_jenis_prw=periksa_radiologi.kd_jenis_prw
where periksa_radiologi.no_rawat=?
group by periksa_radiologi.kd_jenis_prw;

select tagihan_obat_langsung.besar_tagihan from tagihan_obat_langsung where tagihan_obat_langsung.no_rawat=?;

select 
  obatbhp_ok.nm_obat,
  beri_obat_operasi.hargasatuan,
  beri_obat_operasi.jumlah,
  (beri_obat_operasi.hargasatuan*beri_obat_operasi.jumlah) as total
from obatbhp_ok inner join beri_obat_operasi on beri_obat_operasi.kd_obat = obatbhp_ok.kd_obat
where beri_obat_operasi.no_rawat=? 
group by obatbhp_ok.nm_obat;

select
  databarang.nama_brng,
  jenis.nama_jns,
  detail_pemberian_obat.biaya_obat,
  sum(detail_pemberian_obat.jml) as jml,
  sum(detail_pemberian_obat.embalase+detail_pemberian_obat.tuslah) as tambahan,
  (sum(detail_pemberian_obat.total)-sum(detail_pemberian_obat.embalase+detail_pemberian_obat.tuslah)) as total
from detail_pemberian_obat
inner join databarang on databarang.kdjns=jenis.kdjns
inner join jenis on detail_pemberian_obat.kode_brng=databarang.kode_brng
where detail_pemberian_obat.no_rawat=?
group by detail_pemberian_obat.kode_brng
order by jenis.nama;
