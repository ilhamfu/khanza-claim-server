select
  databarang.nama_brng as "nama_barang!",
  detreturjual.h_retur as "harga!",
  sum(detreturjual.jml_retur) as "qty!",
  sum(detreturjual.subtotal * -1) as "total!"
from detreturjual 
inner join databarang on detreturjual.kode_brng=databarang.kode_brng
inner join returjual on returjual.no_retur_jual=detreturjual.no_retur_jual 
where returjual.no_retur_jual like ? 
group by databarang.kode_brng
