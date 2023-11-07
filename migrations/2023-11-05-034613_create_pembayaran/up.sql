-- Your SQL goes here
CREATE TABLE pembayaran (
  id_pembayaran SERIAL,
  tgl_bayar DATE,
  total_bayar INT,
  id_transaksi INT,
  PRIMARY KEY(id_pembayaran),
  CONSTRAINT fk_transaksi
    FOREIGN KEY(id_transaksi)
      REFERENCES transaksi(id_transaksi)
);
