select periksa_lab.tgl_periksa as tanggal_periksa,
       periksa_lab.jam,
       periksa_lab.no_rawat,
       jns_perawatan_lab.nm_perawatan as nama_perawatan,
       petugas.nama as nama_petugas,
       dokter.nm_dokter as nama_dokter,
       CONCAT('[',
          GROUP_CONCAT(JSON_OBJECT(
            'pemeriksaan',template_laboratorium.Pemeriksaan,
            'nilai',detail_periksa_lab.nilai,
            'satuan',template_laboratorium.satuan,
            'nilai_rujukan',detail_periksa_lab.nilai_rujukan,
            'keterangan',detail_periksa_lab.keterangan
          ))
        ,']') as "detail:Json<Vec<DetailLab>>"

FROM periksa_lab
INNER JOIN jns_perawatan_lab ON periksa_lab.kd_jenis_prw=jns_perawatan_lab.kd_jenis_prw
INNER JOIN petugas ON periksa_lab.nip=petugas.nip
INNER JOIN dokter ON periksa_lab.kd_dokter=dokter.kd_dokter
INNER JOIN detail_periksa_lab on (
  detail_periksa_lab.no_rawat=periksa_lab.no_rawat
  AND detail_periksa_lab.kd_jenis_prw=periksa_lab.kd_jenis_prw
  AND detail_periksa_lab.tgl_periksa=periksa_lab.tgl_periksa
  AND detail_periksa_lab.jam=periksa_lab.jam
)
INNER JOIN template_laboratorium ON detail_periksa_lab.id_template=template_laboratorium.id_template
WHERE periksa_lab.kategori<>'PA'
  AND periksa_lab.no_rawat=?
GROUP BY periksa_lab.no_rawat,periksa_lab.tgl_periksa,periksa_lab.jam,periksa_lab.kd_jenis_prw
ORDER BY periksa_lab.tgl_periksa,
         periksa_lab.jam;

