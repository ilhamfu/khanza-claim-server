select
  hasil_radiologi.tgl_periksa as tanggal_periksa,
  hasil_radiologi.jam as jam_periksa,
  hasil_radiologi.hasil as hasil 
from hasil_radiologi 
where hasil_radiologi.no_rawat=? 
order by hasil_radiologi.tgl_periksa,hasil_radiologi.jam
