SELECT data_triase_igdsekunder.anamnesa_singkat,
       data_triase_igdsekunder.catatan,
       data_triase_igdsekunder.plan,
       data_triase_igdsekunder.tanggaltriase as tanggal_triase,
       data_triase_igdsekunder.nik,
       data_triase_igd.tekanan_darah,
       data_triase_igd.nadi,
       data_triase_igd.pernapasan,
       data_triase_igd.suhu,
       data_triase_igd.saturasi_o2,
       data_triase_igd.nyeri,
       data_triase_igd.cara_masuk,
       data_triase_igd.alat_transportasi,
       data_triase_igd.alasan_kedatangan,
       data_triase_igd.keterangan_kedatangan,
       data_triase_igd.kode_kasus,
       master_triase_macam_kasus.macam_kasus,
       pegawai.nama as nama_pegawai
FROM data_triase_igdsekunder
INNER JOIN data_triase_igd ON data_triase_igd.no_rawat=data_triase_igdsekunder.no_rawat
INNER JOIN master_triase_macam_kasus ON data_triase_igd.kode_kasus=master_triase_macam_kasus.kode_kasus
INNER JOIN pegawai ON data_triase_igdsekunder.nik=pegawai.nik
WHERE data_triase_igd.no_rawat=?
