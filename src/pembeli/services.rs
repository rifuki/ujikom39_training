use crate::{Pool, schema::pembeli::dsl::pembeli, pembeli::models::Pembeli};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use std::convert::From;
use std::fmt;
use std::error::Error;

pub async fn get_buyers (pool: &Pool) -> Result<Vec<Pembeli>, BuyerError> {
    let mut conn = pool.get().map_err(BuyerError::ConnectionError)?;
    let all_buyers= pembeli 
        .load::<Pembeli>(&mut conn)
        .map_err(BuyerError::DatabaseError)?;

    Ok(all_buyers)
}

pub async fn create_buyer (pool: &Pool, payload: Pembeli) -> Result<Pembeli, BuyerError> {
    let mut conn = pool.get()?;
    let new_buyer = Pembeli {
        id_pembeli: payload.id_pembeli,
        nama_pembeli: payload.nama_pembeli,
        jk: payload.jk,
        no_tlp: payload.no_tlp,
        alamat: payload.alamat
    };

    let result = diesel::insert_into(pembeli)
        .values(new_buyer)
        .returning(Pembeli::as_returning())
        .get_result(&mut conn)?;

    Ok(result)
}

pub async fn delete_buyer(pool: &Pool, id_pembeli: i32) -> Result<Pembeli, BuyerError> {
    let mut conn = pool.get()?;

    let result = diesel::delete(pembeli
        .find(id_pembeli))
        .get_result(&mut conn)?;

    Ok(result)
}

#[derive(Debug)]
pub enum BuyerError {
    DatabaseError(DieselError),
    ConnectionError(R2d2Error),
}

impl fmt::Display for BuyerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuyerError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            BuyerError::ConnectionError(err) => write!(f, "Connection Error: {}", err),
        }
    }
}

impl Error for BuyerError {}

impl From<DieselError> for BuyerError {
    fn from(error: DieselError) -> Self {
        BuyerError::DatabaseError(error)
    }
}

impl From<R2d2Error> for BuyerError {
    fn from(error: R2d2Error) -> Self {
        BuyerError::ConnectionError(error)
    }
}
