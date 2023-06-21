mod db;

use std::{sync::Arc, env};

use cita_trie::{PatriciaTrie, Trie};
use db::RocksTrieDB;
use hasher::HasherKeccak;

fn main() {
    let db_path = env::args().nth(1).unwrap();
    let db = PatriciaTrie::new(
        Arc::new(RocksTrieDB::new(db_path)),
        Arc::new(HasherKeccak::new()),
    );

    let res = db.get(&0u64.to_be_bytes());
    println!("{:?}", res);
}
