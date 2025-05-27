select JSON_OBJECT(
  p.skala,
  JSON_EXTRACT(CONCAT('[',GROUP_CONCAT(JSON_OBJECT(
    "nama_pemeriksaan",p.nama_pemeriksaan,
    "pengkajian", JSON_EXTRACT(p.pengkajian_skala,'$')
  )),']'),'$')) as "pemeriksaan:Json<PemeriksaanTriaseUtama>"  from (
  select 
    'skala1' as skala,
    s1.no_rawat,
    mtp.nama_pemeriksaan,
    JSON_EXTRACT(JSON_ARRAYAGG(ms1.pengkajian_skala1),'$') as pengkajian_skala
  from data_triase_igddetail_skala1 s1
  inner join master_triase_skala1 ms1 on s1.kode_skala1 = ms1.kode_skala1
  inner join master_triase_pemeriksaan mtp on mtp.kode_pemeriksaan = ms1.kode_pemeriksaan
  where s1.no_rawat = ?
  group by s1.no_rawat, ms1.kode_pemeriksaan
  union all
  select 
    'skala2' as skala,
    s2.no_rawat,
    mtp.nama_pemeriksaan,
    JSON_ARRAYAGG(ms2.pengkajian_skala2) as pengkajian_skala
  from data_triase_igddetail_skala2 s2
  inner join master_triase_skala2 ms2 on s2.kode_skala2 = ms2.kode_skala2
  inner join master_triase_pemeriksaan mtp on mtp.kode_pemeriksaan = ms2.kode_pemeriksaan
  where s2.no_rawat = ?
  group by s2.no_rawat, ms2.kode_pemeriksaan ) p
group by p.skala
