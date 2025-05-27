select JSON_OBJECT(
  p.skala,
  JSON_EXTRACT(CONCAT('[',GROUP_CONCAT(JSON_OBJECT(
    "nama_pemeriksaan",p.nama_pemeriksaan,
    "pengkajian", JSON_EXTRACT(p.pengkajian_skala,'$')
  )),']'),'$')) as "pemeriksaan:Json<PemeriksaanTriaseSekunder>"  from (
  select 
    'skala3' as skala,
    s3.no_rawat,
    mtp.nama_pemeriksaan,
    JSON_EXTRACT(JSON_ARRAYAGG(ms3.pengkajian_skala3),'$') as pengkajian_skala
  from data_triase_igddetail_skala3 s3
  inner join master_triase_skala3 ms3 on s3.kode_skala3 = ms3.kode_skala3
  inner join master_triase_pemeriksaan mtp on mtp.kode_pemeriksaan = ms3.kode_pemeriksaan
  where s3.no_rawat = ?
  group by s3.no_rawat, ms3.kode_pemeriksaan
  union all
  select 
    'skala4' as skala,
    s4.no_rawat,
    mtp.nama_pemeriksaan,
    JSON_ARRAYAGG(ms4.pengkajian_skala4) as pengkajian_skala
  from data_triase_igddetail_skala4 s4
  inner join master_triase_skala4 ms4 on s4.kode_skala4 = ms4.kode_skala4
  inner join master_triase_pemeriksaan mtp on mtp.kode_pemeriksaan = ms4.kode_pemeriksaan
  where s4.no_rawat = ?
  group by s4.no_rawat, ms4.kode_pemeriksaan 
  union all
  select 
    'skala5' as skala,
    s5.no_rawat,
    mtp.nama_pemeriksaan,
    JSON_ARRAYAGG(ms5.pengkajian_skala5) as pengkajian_skala
  from data_triase_igddetail_skala5 s5
  inner join master_triase_skala5 ms5 on s5.kode_skala5 = ms5.kode_skala5
  inner join master_triase_pemeriksaan mtp on mtp.kode_pemeriksaan = ms5.kode_pemeriksaan
  where s5.no_rawat = ?
  group by s5.no_rawat, ms5.kode_pemeriksaan 
) p
group by p.skala
