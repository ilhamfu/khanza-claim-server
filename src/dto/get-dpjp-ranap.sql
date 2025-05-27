select 
  dpjp_ranap.kd_dokter as "kode_dokter!",
  dokter.nm_dokter as "nama_dokter!"
from dpjp_ranap 
inner join dokter on dpjp_ranap.kd_dokter=dokter.kd_dokter 
where dpjp_ranap.no_rawat = ?
