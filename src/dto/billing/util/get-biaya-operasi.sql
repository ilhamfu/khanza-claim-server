select
  paket_operasi.nm_perawatan as "nama_perawatan!",
  (
    operasi.biayaoperator1
    + operasi.biayaoperator2
    + operasi.biayaoperator3
    + operasi.biayaasisten_operator1
    + operasi.biayaasisten_operator2
    + operasi.biayaasisten_operator3
    + operasi.biayainstrumen
    + operasi.biayadokter_anak
    + operasi.biayaperawaat_resusitas
    + operasi.biayadokter_anestesi
    + operasi.biayaasisten_anestesi
    + operasi.biayaasisten_anestesi2
    + operasi.biayabidan
    + operasi.biayabidan2
    + operasi.biayabidan3
    + operasi.biayaperawat_luar
    + operasi.biayaalat
    + operasi.biayasewaok
    + operasi.akomodasi
    + operasi.bagian_rs
    + operasi.biaya_omloop 
    + operasi.biaya_omloop2
    + operasi.biaya_omloop3
    + operasi.biaya_omloop4
    + operasi.biaya_omloop5
    + operasi.biayasarpras
    + operasi.biaya_dokter_pjanak
    + operasi.biaya_dokter_umum
  ) as "biaya!"
from operasi 
inner join paket_operasi on operasi.kode_paket=paket_operasi.kode_paket 
where operasi.no_rawat=? 
