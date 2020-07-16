pub mod object;
pub mod objstore;
pub mod keystore;
//pub mod keystore;

use objstore::filestore::FileStore;
use objstore::objstore::ObjectStore;
//use keystore::keystore::SQLiteKeyStore;

use std::path::PathBuf;
//use std::str;
use uuid::Uuid;
use std::io::prelude::*;
use std::io;
//use std::fs::File;

fn main() -> io::Result<()> {

    // PUT an object:
    //  get object size
    //  hash object data
    //  determine object placement
    //  generate uuid
    //  store key in keystore
    //  put data into object store
    //  return uuid
    //
    // GET an object by uuid:
    //  query keystore for uuid
    //  determine object location and size
    //  read object from object store
    //  return object
    //

    let mut fs = FileStore::new(PathBuf::from("."));

    /*
    let uuid = fs.put("asdfqwerty1234".as_bytes()).unwrap();
    println!("uuid: {}", uuid);

    let data = fs.get(uuid).unwrap().unwrap();
    println!("get returned {} bytes: {}", data.len(), String::from_utf8(data).unwrap());

    //fs.delete(uuid).unwrap();
    //println!("deleted {}", uuid);

    let data = fs.get(uuid).unwrap().unwrap();
    println!("{:?}", data);
    */

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let tokens: Vec<&str> = input.split(" ").collect();
        println!("{:?}", tokens);

        let cmd = String::from(tokens[0]);

        let mut arg = String::from(tokens[1]);
        arg.truncate(tokens[1].len() - 1);
        println!("{:?}: {:?}", cmd, arg);

        match cmd.as_str() {
            "put" => { 
                let uuid = fs.put(arg.as_bytes())?;
                println!("uuid: {:?}", uuid);
            },
            "get" => {
                let uuid = Uuid::from_slice(arg.as_bytes()).unwrap();
                let data = fs.get(uuid)?;
                println!("data: {:?}", data);
            },
            "delete" => {
                let uuid = Uuid::from_slice(arg.as_bytes()).unwrap();
                let result = fs.delete(uuid);
                println!("{:?}", result);
            },
            "objects" => {
                
            }
            _ => {
                println!("Unknown command: {:?}", cmd);
            }
        }
    }
}
