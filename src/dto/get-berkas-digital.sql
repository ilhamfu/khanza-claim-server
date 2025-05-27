SELECT 
  bdp.kode,
  mbd.nama as kategori,
  bdp.lokasi_file
FROM berkas_digital_perawatan bdp
INNER JOIN master_berkas_digital mbd ON mbd.kode = bdp.kode
WHERE bdp.no_rawat = ?;
