use crate::KeyStore;
use crate::Object;

pub struct SQLiteKeyStore {
    filename: &'static str,
    dbconn: Connection,
}

impl SQLiteKeyStore {
    pub fn new(filename: &'static str) -> SQLiteKeyStore {
        let mut ks = SQLiteKeyStore {
            filename,
            dbconn: Connection::open(filename).unwrap()
        };

        ks.create_table(&"object", vec![
            ("uuid", "TEXT PRIMARY KEY"),
            ("hash", "UNSIGNED BIG INT"),
            ("size", "UNSIGNED BIG INT"),
        ]);
        
        ks
    }

    pub fn create_table(&mut self, name: &str, fields: Vec<(&str, &str)>) -> Result<()> {
        let mut s: String = format!("CREATE TABLE {} ( \n )", name);
        for (field, field_type) in fields {
            s.push_str(format!("{} {},\n", field, field_type).as_str());
        }
        self.dbconn.execute(s.as_str(), params![]);
        Ok(())
    }
}

impl KeyStore for SQLiteKeyStore {
    fn store(mut self, obj: &Object) -> Result<Object> {
        // an object has been built and needs to be stored
        self.dbconn.execute("INSERT INTO object (uuid, hash, size) VALUES (?1, ?2, ?3)", &[obj.uuid, obj.hash, obj.size]);

        Ok(obj)
    }

    fn retrieve(mut self, uuid: Uuid) -> Result<Object> {
        self.dbconn.query_row("");

        let mut obj = Default::default();

        Ok(obj)
    }

}
