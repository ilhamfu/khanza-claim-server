SELECT operasi.no_rawat,
       operasi.tgl_operasi as tanggal_operasi,
       operasi.jenis_anasthesi,
       operasi.kategori,
        paket_operasi.nm_perawatan as nama_perawatan,
       laporan_operasi.diagnosa_preop,
       laporan_operasi.diagnosa_postop,
       laporan_operasi.jaringan_dieksekusi,
       laporan_operasi.selesaioperasi as selesai_operasi,
       laporan_operasi.permintaan_pa,
       laporan_operasi.laporan_operasi,

  (SELECT nm_dokter
   FROM dokter
   WHERE dokter.kd_dokter = operasi.operator1) AS nama_op_1,

  (SELECT nm_dokter
   FROM dokter
   WHERE dokter.kd_dokter = operasi.operator2) AS nama_op_2,

  (SELECT nm_dokter
   FROM dokter
   WHERE dokter.kd_dokter = operasi.operator3) AS nama_op_3,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.asisten_operator1) AS nama_asop_1,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.asisten_operator2) AS nama_asop_2,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.asisten_operator3) AS nama_asop_3,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.instrumen) AS nama_instrumen,

  (SELECT nm_dokter
   FROM dokter
   WHERE dokter.kd_dokter = operasi.dokter_anak) AS nama_dokter_anak,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.perawaat_resusitas)AS nama_perawat_resusitas,

  (SELECT nm_dokter
   FROM dokter
   WHERE dokter.kd_dokter = operasi.dokter_anestesi) AS nama_dokter_anasthesi,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.asisten_anestesi) AS nama_asisten_anesthesi_1,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.asisten_anestesi2) AS nama_asisten_anesthesi_2,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.bidan) AS nama_bidan_1,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.bidan2) AS nama_bidan_2,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.bidan3) AS nama_bidan_3,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.perawat_luar) AS nama_perawat_luar,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.omloop) AS nama_onloop_1,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.omloop2) AS nama_onloop_2,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.omloop3) AS nama_onloop_3,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.omloop4) AS nama_onloop_4,

  (SELECT nama
   FROM petugas
   WHERE petugas.nip = operasi.omloop5) AS nama_onloop_5,

  (SELECT nm_dokter
   FROM dokter
   WHERE dokter.kd_dokter = operasi.dokter_pjanak) AS nama_dokter_pj_anak,

  (SELECT nm_dokter
   FROM dokter
   WHERE dokter.kd_dokter = operasi.dokter_umum) AS nama_dokter_umum 
FROM operasi
INNER JOIN laporan_operasi ON operasi.no_rawat=laporan_operasi.no_rawat
INNER JOIN paket_operasi ON operasi.kode_paket=paket_operasi.kode_paket
AND operasi.tgl_operasi=laporan_operasi.tanggal
where operasi.no_rawat = ?
order by tanggal_operasi asc
