// @generated automatically by Diesel CLI.

diesel::table! {
    barang (id_barang) {
        id_barang -> Int4,
        #[max_length = 20]
        nama_barang -> Nullable<Varchar>,
        harga -> Nullable<Int4>,
        stok -> Nullable<Int4>,
        id_supplier -> Nullable<Int4>,
    }
}

diesel::table! {
    login (username) {
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    pembayaran (id_pembayaran) {
        id_pembayaran -> Int4,
        tgl_bayar -> Nullable<Date>,
        total_bayar -> Nullable<Int4>,
        id_transaksi -> Nullable<Int4>,
    }
}

diesel::table! {
    pembeli (id_pembeli) {
        id_pembeli -> Int4,
        nama_pembeli -> Varchar,
        jk -> Varchar,
        no_tlp -> Varchar,
        alamat -> Varchar,
    }
}

diesel::table! {
    supplier (id_supplier) {
        id_supplier -> Int4,
        #[max_length = 30]
        nama_supplier -> Nullable<Varchar>,
        #[max_length = 1]
        no_tlp -> Nullable<Bpchar>,
        #[max_length = 100]
        alamat -> Nullable<Varchar>,
    }
}

diesel::table! {
    transaksi (id_transaksi) {
        id_transaksi -> Int4,
        id_barang -> Nullable<Int4>,
        id_pembeli -> Nullable<Int4>,
        tanggal -> Nullable<Date>,
        #[max_length = 100]
        keterangan -> Nullable<Varchar>,
    }
}

diesel::joinable!(barang -> supplier (id_supplier));
diesel::joinable!(pembayaran -> transaksi (id_transaksi));
diesel::joinable!(transaksi -> pembeli (id_pembeli));

diesel::allow_tables_to_appear_in_same_query!(
    barang,
    login,
    pembayaran,
    pembeli,
    supplier,
    transaksi,
);
