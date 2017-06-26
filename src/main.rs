
extern crate leveldb;
extern crate gethrpc;

use std::path::Path;
use std::net::SocketAddr;
use leveldb::database::Database;
use leveldb::kv::KV;
use leveldb::iterator::Iterable;
use leveldb::options::{Options,WriteOptions,ReadOptions};

use gethrpc::GethRPCClient;

const DEFAULT_DIR: &'static str = "./blockdb";

fn main() {
  let path = Path::new(DEFAULT_DIR);
  
  let database = BlockDB::new(path);
  
  database.write_block_to_db(b"hello");


  let addr = "127.0.0.1:8000"
    .parse::<SocketAddr>()
    .expect("Expect to parse address");

  let client_addr = "http://127.0.0.1:8545";
    

  let mut client = GethRPCClient::new(client_addr);

  let x: String = client.client_version();

  println!("x: {:?}", x);
  
/*
  
  let read_opts = ReadOptions::new();
  let mut iter = database.iter(read_opts);
  let entry = iter.next();
  assert_eq!(
    entry,
    Some((1, vec![1]))
  );*/
}

pub struct BlockDB {
  db: Database<i32>
}

impl BlockDB {
  pub fn new(path: &Path) -> BlockDB {
      let mut options = Options::new();
      options.create_if_missing = true;
      let db = match Database::open(path, options) {
        Ok(db) => { db },
        Err(e) => { panic!("failed to open database: {:?}", e) }
      };
      BlockDB {
        db: db,
      }
  }
  
  pub fn write_block_to_db(&self, block: &[u8]) -> () {
      let write_opts = WriteOptions::new();
      match self.db.put(write_opts, 1, block) {
          Ok(_) => { () },
          Err(e) => { panic!("failed to write to database: {:?}", e) }
      };    
  }

  pub fn read_block_from_db(&self, key: i32) -> Option<Vec<u8>> {
      let read_opts = ReadOptions::new();
      let res = self.db.get(read_opts, key);
      let data = match res {
        Ok(data) => { data },
        Err(e) => { panic!("failed reading data: {:?}", e) }
      };
      data
  } 

}