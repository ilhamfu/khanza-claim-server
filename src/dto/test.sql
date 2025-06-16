SELECT
  JSON_OBJECT(
    'waktu_masuk',min(timestamp(tgl_masuk,jam_masuk)),
    'waktu_keluar',max(timestamp(tgl_keluar,jam_keluar))
  ) as data
FROM kamar_inap
WHERE no_rawat = '2025/05/31/000047'
GROUP BY no_rawat
