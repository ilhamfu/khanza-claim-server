select 
  reg_periksa.no_rawat
from reg_periksa 
where 
  reg_periksa.no_rawat not in (select gmc_claim__docu_cache.no_rawat from gmc_claim__docu_cache where gmc_claim__docu_cache.last_update >= ? or gmc_claim__docu_cache.status = 'success')
  and kd_pj='BPJ'
  and stts='Sudah'
  and tgl_registrasi >= ?
  and tgl_registrasi <= ?
order by reg_periksa.tgl_registrasi asc
limit 10;
