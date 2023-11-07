-- Your SQL goes here
CREATE TABLE barang (
  id_barang SERIAL,
  nama_barang VARCHAR(20),
  harga INT,
  stok INT,
  id_supplier INT,
  PRIMARY KEY(id_barang),
  CONSTRAINT fk_supplier
    FOREIGN KEY(id_supplier)
      REFERENCES supplier(id_supplier)
);
