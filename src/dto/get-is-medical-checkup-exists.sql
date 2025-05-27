select 
  TRUE as exist,
  (kd_pj='BPJ') as is_bpjs,
  (gmc_claim__docu_cache.last_update) as exported
  from reg_periksa left join gmc_claim__docu_cache on gmc_claim__docu_cache.no_rawat = reg_periksa.no_rawat where reg_periksa.no_rawat = ?
