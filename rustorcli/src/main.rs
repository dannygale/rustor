use librustor::*;
use librustor::object::ObjKey;
use librustor::RResult;
use librustor::objstore::BasicObjectStore;
use librustor::blockstore::SingleDeviceBlockStore;
use librustor::keystore::JsonKeystore;
use librustor::freelist::RCVecFreeList;

use std::path::PathBuf;
use std::str;
use uuid::Uuid;
use std::io::prelude::*;
use std::io;
//use std::fs::File;

#[macro_use]
extern crate clap;
use clap::App;


use log;
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};
use env_logger;



fn main() -> RResult<()> {

    env_logger::init();

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

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let keystore_file = matches.value_of("KEYSTORE").unwrap_or("keys.json");
    let objstore_file = matches.value_of("OBJSTORE").unwrap_or("data.bin");
    
    let interactive: bool = matches.is_present("interactive");
    debug!("{:#?}", matches);

    let size = 1024*1024;
    let mut bs = SingleDeviceBlockStore::new(PathBuf::from(objstore_file), size);
    let mut fl = RCVecFreeList::new(size);
    let kg = keygen::KeyGen {};
    let mut ks: JsonKeystore<ObjKey> = keystore::JsonKeystore::new(PathBuf::from(keystore_file));

    // reconstruct free list from keystore
    for obj in ks.get_objects().values() {
        debug!("taking blocks for {:?}", &obj.uuid);
        for block in obj.manifest.shards.iter() {
            trace!("{:#?}", &fl.by_addr);
            fl.take(block.span, block.lba)?;
        }
    }

    let mut fs = BasicObjectStore::new(&mut bs, &mut fl, kg, &mut ks);

    if interactive { return interactive_loop(&mut fs); }

    if let Some(subcommand) = matches.subcommand_name() {
        debug!("subcommand: {:#?}", &subcommand);
        if let Some(ref matches) = matches.subcommand_matches(subcommand) { 
            debug!("subcommand matches: {:#?}", &matches);
            match subcommand {
                "put" => {
                    let uuid = fs.put(matches.value_of("data").unwrap().as_bytes())?;
                    info!("uuid: {:?}", &uuid);
                }
                "get" => {
                    let data = fs.get(Uuid::parse_str(matches.value_of("uuid").unwrap()).unwrap())?;
                    //info!("data: {:?}", String::from_utf8(data.unwrap()));
                }
                "delete" => {

                }
                "keys" => { 

                }
                "objs" => {

                }
                _ => Err(format!("Unknown subcommand: {}", subcommand))?
            }
        }
    }

    Ok(())
}


fn interactive_loop(fs: &mut impl ObjectStore) -> RResult<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.truncate(input.len() - 1);

        let tokens: Vec<&str> = input.split(" ").collect();
        debug!("{:?}", tokens);

        let cmd = String::from(tokens[0]);

        let mut arg = String::new();
        if tokens.len() == 2 {
            arg.push_str(tokens[1]);
        } 
        debug!("{:?}: {:?}", cmd, arg);

        match cmd.as_str() {
            "put" => { 
                let uuid = fs.put(arg.as_bytes())?;
                debug!("uuid: {:?}", uuid);
            },
            "get" => {
                let uuid = Uuid::parse_str(arg.as_str()).unwrap();
                let data = match fs.get(uuid)? {
                    Some(d) => d,
                    None => Vec::new()
                };
                debug!("data: {:?}", data);
            },
            "delete" => {
                let uuid = Uuid::parse_str(arg.as_str()).unwrap();
                let result = fs.delete(uuid);
                debug!("{:?}", result);
            },
            "keys" => {

            },
            "objs" => {

            }
            /*
               "objects" => {
               let objects = fs.get_objects();
               for (uuid, key) in objects {
               debug!("{:?}", key)
               }
               }
               */
            _ => {
                debug!("Unknown command: {:?}", cmd);
            }
        }
    }
}
