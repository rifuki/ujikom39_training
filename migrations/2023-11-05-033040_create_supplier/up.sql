-- Your SQL goes here
CREATE TABLE supplier (
  id_supplier SERIAL,
  nama_supplier VARCHAR(30),
  no_tlp CHAR,
  alamat VARCHAR(100),
  PRIMARY KEY(id_supplier)
);
