use serde_json;
use serde::{Serialize, Deserialize};

pub fn struct_to_hashmap<T>(t: T) 
where T: Serialize + Deserialize {
    let v = serde_json::to_value(&entry).unwrap();
    let document = serde_json::from_value(v).unwrap();
}
