
use rustor::*;

use std::path::PathBuf;
use std::str;
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
    // DELTE an object by uuid:
    //  look up the object by uuid
    //  remove the object from the keystore
    //  remove the data from storage
    //  update the freelist to include the freed data

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
        input.truncate(input.len() - 1);
        
        let tokens: Vec<&str> = input.split(" ").collect();
        println!("{:?}", tokens);

        let cmd = String::from(tokens[0]);

        let mut arg = String::new();
        if tokens.len() == 2 {
            arg.push_str(tokens[1]);
        } 
        println!("{:?}: {:?}", cmd, arg);

        match cmd.as_str() {
            "put" => { 
                let uuid = fs.put(arg.as_bytes())?;
                println!("uuid: {:?}", uuid);
            },
            "get" => {
                let uuid = Uuid::parse_str(arg.as_str()).unwrap();
                let data = match fs.get(uuid)? {
                    Some(d) => d,
                    None => Vec::new()
                };
                println!("data: {:?}", data);
            },
            "delete" => {
                let uuid = Uuid::parse_str(arg.as_str()).unwrap();
                let result = fs.delete(uuid);
                println!("{:?}", result);
            },
            "objects" => {
                let objects = fs.get_objects();
                for (uuid, key) in objects {
                    println!("{:?}", key)
                }
            }
            _ => {
                println!("Unknown command: {:?}", cmd);
            }
        }
    }
}
