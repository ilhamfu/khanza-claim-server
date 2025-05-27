select 
  kamar_inap.kd_kamar as "kode_kamar!",
  bangsal.nm_bangsal as "nama_bangsal!",
  kamar_inap.trf_kamar as "tarif_kamar!",
  kamar_inap.lama as "lama!",
  kamar_inap.ttl_biaya as "total!",
  IFNULL(CONCAT("[",
    ( 
      select GROUP_CONCAT(JSON_OBJECT( 
        'nama',biaya_sekali.nama_biaya,
        'biaya',biaya_sekali.besar_biaya
      )) from biaya_sekali 
      where kd_kamar = kamar_inap.kd_kamar
    ),
  "]"),"[]") as "bs!:Json<Vec<TempBiayaSekali>>",
  IFNULL(CONCAT("[",
    (
      select GROUP_CONCAT(JSON_OBJECT (
        'nama',biaya_harian.nama_biaya,
        'biaya',biaya_harian.besar_biaya,
        'jumlah',biaya_harian.jml
      )) from biaya_harian 
      where kd_kamar = kamar_inap.kd_kamar
    ),
  "]"),"[]") as "bh!:Json<Vec<TempBiayaHarian>>"
from kamar_inap 
inner join kamar on kamar_inap.kd_kamar=kamar.kd_kamar 
inner join bangsal on kamar.kd_bangsal=bangsal.kd_bangsal
where kamar_inap.no_rawat=? 
order by kamar_inap.tgl_masuk,kamar_inap.kd_kamar;
