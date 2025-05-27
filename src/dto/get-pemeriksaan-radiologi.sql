SELECT
  periksa_radiologi.no_rawat as no_rawat,
  periksa_radiologi.nip as kode_petugas,
  petugas.nama as nama_petugas,
  periksa_radiologi.kd_jenis_prw as kode_perawatan,
  jns_perawatan_radiologi.nm_perawatan as nama_perawatan,
  periksa_radiologi.tgl_periksa as tanggal_periksa,
  periksa_radiologi.jam as jam_periksa,
  periksa_radiologi.kd_dokter as kode_dokter,
  d1.nm_dokter as nama_dokter,
  periksa_radiologi.status as status,
  periksa_radiologi.proyeksi as proyeksi,
  periksa_radiologi.kV as kv,
  periksa_radiologi.mAS as mas,
  periksa_radiologi.FFD as ffd,
  periksa_radiologi.BSF as bfs,
  periksa_radiologi.inak as inak,
  periksa_radiologi.jml_penyinaran as jumlah_penyinaran,
  periksa_radiologi.dosis as dosis
FROM
periksa_radiologi
INNER JOIN jns_perawatan_radiologi ON periksa_radiologi.kd_jenis_prw = jns_perawatan_radiologi.kd_jenis_prw
INNER JOIN petugas ON periksa_radiologi.nip = petugas.nip
INNER JOIN dokter d1 ON d1.kd_dokter = periksa_radiologi.kd_dokter
WHERE no_rawat = ?
order by periksa_radiologi.tgl_periksa,periksa_radiologi.jam
