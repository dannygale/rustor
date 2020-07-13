pub mod object;
pub mod objstore;
pub mod keystore;
//pub mod keystore;

use keystore::json_keystore::JsonKeystore;
use objstore::filestore::FileStore;
use objstore::objstore::ObjectStore;
//use keystore::keystore::SQLiteKeyStore;

use std::path::PathBuf;
//use std::str;
//use uuid::Uuid;
use std::io;

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

    let uuid = fs.put("asdfqwerty1234".as_bytes()).unwrap();
    println!("uuid: {}", uuid);

    let data = fs.get(uuid).unwrap().unwrap();
    println!("get returned {} bytes: {}", data.len(), String::from_utf8(data).unwrap());

    //fs.delete(uuid).unwrap();
    //println!("deleted {}", uuid);

    let data = fs.get(uuid).unwrap().unwrap();

    /*
    loop {
        println!("> ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
                
            }
            Err(error) => println!("error: {}", error)
        }
    }
    */

    Ok(())
}
