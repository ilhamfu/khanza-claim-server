
select
  databarang.nama_brng as "nama_obat!",
  jenis.nama as jenis,
  detail_pemberian_obat.biaya_obat as "harga!",
  sum(detail_pemberian_obat.jml) as "qty!",
  sum(detail_pemberian_obat.embalase+detail_pemberian_obat.tuslah) as "tambahan!",
  (sum(detail_pemberian_obat.total)-sum(detail_pemberian_obat.embalase+detail_pemberian_obat.tuslah)) as "total!"
from detail_pemberian_obat
inner join databarang on detail_pemberian_obat.kode_brng=databarang.kode_brng
inner join jenis on databarang.kdjns=jenis.kdjns
where detail_pemberian_obat.no_rawat=?
group by detail_pemberian_obat.kode_brng
order by jenis.nama;
