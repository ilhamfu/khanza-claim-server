select 
  bridging_rujukan_bpjs.no_sep as "no_sep!",
  bridging_sep.no_rawat as "no_rawat!",
  bridging_sep.nomr as "no_rm!",
  bridging_sep.nama_pasien as "nm_pasien!",
  bridging_sep.nmdpdjp as "nm_dokter!",
  bridging_sep.kddpjp as "kd_dokter!",
  bridging_sep.no_kartu as "no_kartu!",
  if(bridging_sep.jkel='L','Laki Laki','Perempan') as "jk!",
  bridging_rujukan_bpjs.tglRujukan as "tgl_surat_rujuk!",
  bridging_rujukan_bpjs.no_rujukan as "no_surat_rujuk!",
  bridging_rujukan_bpjs.ppkDirujuk as "kd_ppk_rujuk!",
  bridging_rujukan_bpjs.nm_ppkDirujuk as "nm_ppk_rujuk!",
  if(bridging_rujukan_bpjs.jnsPelayanan='1',' Rawat Inap',' Rawat Jalan') as "jns_pelayanan!",
  bridging_rujukan_bpjs.tipeRujukan as "tgl_rujukan!",
  bridging_rujukan_bpjs.catatan as "catatan!",
  bridging_rujukan_bpjs.diagRujukan as "kd_diagnosa_rujuk!",
  bridging_rujukan_bpjs.nama_diagRujukan "nm_diagnosa_rujuk!",
  bridging_rujukan_bpjs.poliRujukan as "kd_poli_rujuk!",
  bridging_rujukan_bpjs.nama_poliRujukan as "nm_poli_rujuk!",
  bridging_rujukan_bpjs.tglRencanaKunjungan as "tgl_rencana_rujuk!" 
from bridging_sep 
inner join bridging_rujukan_bpjs on bridging_rujukan_bpjs.no_sep=bridging_sep.no_sep 
where bridging_rujukan_bpjs.tipeRujukan = '0. Penuh' and bridging_sep.no_sep = ?
