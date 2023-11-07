-- Your SQL goes here
CREATE TABLE transaksi (
  id_transaksi SERIAL,
  id_barang INT,
  id_pembeli INT,
  tanggal DATE,
  keterangan VARCHAR(100),
  PRIMARY KEY (id_transaksi),
  CONSTRAINT fk_pembeli
    FOREIGN KEY(id_pembeli)
      REFERENCES pembeli(id_pembeli)
);
