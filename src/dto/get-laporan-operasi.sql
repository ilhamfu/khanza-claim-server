select
  tanggal,
  diagnosa_preop,
  diagnosa_postop,
  jaringan_dieksekusi,
  selesaioperasi as selesai_operasi,
  permintaan_pa,
  laporan_operasi
from laporan_operasi
where no_rawat=?
order by tanggal
