select
  bspb.kd_dokter_bpjs as kode_dokter,
  bspb.nm_dokter_bpjs as nama_dokter,
  bspb.nm_poli_bpjs as nama_poli,
  bspb.no_kartu as no_kartu,
  p.nm_pasien as nama_pasien,
  p.tgl_lahir as tanggal_lahir,
  bspb.diagnosa as diagnosa,
  bspb.tgl_surat as tanggal_surat,
  bspb.no_surat as no_surat,
  bspb.tgl_rencana as tanggal_rencana,
  p.jk as jenis_kel
from bridging_surat_pri_bpjs bspb 
inner join reg_periksa rp on bspb.no_rawat = rp.no_rawat
inner join pasien p on p.no_rkm_medis = rp.no_rkm_medis
where bspb.no_rawat = ?
