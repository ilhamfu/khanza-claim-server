select 
  gambar_radiologi.tgl_periksa as tanggal_periksa,
  gambar_radiologi.jam as jam_periksa,
  gambar_radiologi.lokasi_gambar 
from gambar_radiologi 
where gambar_radiologi.no_rawat=? 
order by gambar_radiologi.tgl_periksa,gambar_radiologi.jam
