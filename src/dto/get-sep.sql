SELECT
  bridging_sep.no_sep as no_sep,
  bridging_sep.tglsep as tanggal_sep,
  bridging_sep.nomr as no_rm,
  bridging_sep.no_kartu as no_kartu,
  bridging_sep.nama_pasien as nama_pasien,
  bridging_sep.tanggal_lahir as tanggal_lahir,
  bridging_sep.jkel as jk,
  bridging_sep.notelep as no_telp,
  bridging_sep.nmpolitujuan as nama_poli_tujuan,
  bridging_sep.nmdpdjp as nama_dpjp,
  bridging_sep.nmppkrujukan as nama_ppk_rujukan,
  bridging_sep.nmdiagnosaawal as nama_diagnosa_awal,
  bridging_sep.catatan as catatan,
  bridging_sep.peserta as peserta,
  (
    CASE bridging_sep.jnspelayanan
      WHEN '1' THEN 'Rawat Inap'
      WHEN '2' THEN 'Rawat Jalan'
    END
  ) AS jenis_pelayanan,
  (
    CASE bridging_sep.tujuankunjungan
    WHEN '0' THEN 'Konsultasi Dokter (Pertama)'
    ELSE 'Kunjungan Kontrol (Ulangan)'
  END
  ) AS tujuan_kunjungan,
  (
    CASE bridging_sep.flagprosedur
      WHEN '0' THEN 'Prosedur Tidak Berkelanjutan'
      WHEN '1' THEN 'Prosedur dan Terapi Berkelanjutan'
      ELSE ''
    END
  ) AS flag_prosedur,
  (
    CASE bridging_sep.klsrawat
      WHEN '1' THEN 'Kelas 1'
      WHEN '2' THEN 'Kelas 2'
      ELSE 'Kelas 3'
    END
  ) AS kelas_rawat,
  (
    CASE bridging_sep.klsnaik
      WHEN '1' THEN 'VVIP'
      WHEN '2' THEN 'VIP'
      WHEN '3' THEN 'Kelas I'
      WHEN '4' THEN 'Kelas II'
      WHEN '5' THEN 'Kelas III'
      WHEN '6' THEN 'ICCU'
      WHEN '7' THEN 'ICU'
      WHEN '8' THEN 'Diatas Kelas I'
      ELSE ''
    END
  ) AS kelas_naik,
  (
    CASE bridging_sep.lakalantas
      WHEN '0' THEN 'BPJS Kesehatan'
      WHEN '1' THEN 'Jasa Raharja'
      WHEN '2' THEN 'Jasa Raharja & BPJS Ketenagakerjaan/Taspen'
      WHEN '3' THEN 'BPJS Ketenagakerjaan, Taspen, dll'
      ELSE ''
    END
  ) AS lakalantas
FROM
  bridging_sep
WHERE
  bridging_sep.no_rawat = ?
  AND bridging_sep.jnspelayanan = ?
