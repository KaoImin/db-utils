use std::{path::Path, sync::Arc};

use derive_more::Display;
use rocksdb::{
    ops::{Get, Open},
    DB,
};

pub struct RocksTrieDB {
    db: Arc<DB>,
}

impl RocksTrieDB {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let db = Arc::new(DB::open(&Default::default(), path).unwrap());

        RocksTrieDB { db }
    }
}

impl cita_trie::DB for RocksTrieDB {
    type Error = RocksTrieDBError;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        let ret = self.db.get(key).map_err(to_store_err)?.map(|r| r.to_vec());
        Ok(ret)
    }

    fn contains(&self, key: &[u8]) -> Result<bool, Self::Error> {
        self.get(key).map(|r| r.is_some())
    }

    fn insert(&self, _key: Vec<u8>, _value: Vec<u8>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn insert_batch(&self, _keys: Vec<Vec<u8>>, _values: Vec<Vec<u8>>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn remove(&self, _key: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn remove_batch(&self, _keys: &[Vec<u8>]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn flush(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, Display)]
pub enum RocksTrieDBError {
    RocksDB(rocksdb::Error),
}

impl std::error::Error for RocksTrieDBError {}

fn to_store_err(e: rocksdb::Error) -> RocksTrieDBError {
    RocksTrieDBError::RocksDB(e)
}
