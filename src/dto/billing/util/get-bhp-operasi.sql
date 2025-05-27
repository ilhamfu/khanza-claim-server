
select 
  obatbhp_ok.nm_obat as "nama_obat!",
  beri_obat_operasi.hargasatuan as "harga!",
  beri_obat_operasi.jumlah as "qty!",
  (beri_obat_operasi.hargasatuan*beri_obat_operasi.jumlah) as total
from obatbhp_ok inner join beri_obat_operasi on beri_obat_operasi.kd_obat = obatbhp_ok.kd_obat
where beri_obat_operasi.no_rawat=? 
group by obatbhp_ok.nm_obat;
