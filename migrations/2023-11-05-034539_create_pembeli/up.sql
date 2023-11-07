-- Your SQL goes here
CREATE TABLE pembeli (
  id_pembeli SERIAL NOT NULL,
  nama_pembeli VARCHAR NOT NULL,
  jk VARCHAR NOT NULL,
  no_tlp VARCHAR NOT NULL,
  alamat VARCHAR NOT NULL,
  PRIMARY KEY(id_pembeli)
);
