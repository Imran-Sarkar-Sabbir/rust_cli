use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde_json::{Deserializer, Value};

use serde::{Serialize, Deserialize};

fn read_file(file_name: String) -> String {
    let path = Path::new("src").join(file_name);
    fs::read_to_string(path).expect("couldn't read file.")
}

fn open_file(file_name: String) -> File {
    let path = Path::new("src").join(file_name);
    File::open(path).expect("couldn't read file.")
}

pub fn read_json_file(file_name: String) -> serde_json::Map<String, Value> {
    let contents = read_file(file_name);
    let parsed: Value = serde_json::from_str(&contents).expect("error parsing json");
    parsed.as_object().unwrap().clone()
}




mod serde_json_any_key {
    use std::any::{Any, TypeId};
    use serde::ser::{Serialize, Serializer, SerializeMap, Error};
    use std::cell::RefCell;
    struct SerializeMapIterWrapper<'a, K, V>
    {
        pub iter: RefCell<&'a mut (dyn Iterator<Item=(&'a K, &'a V)> + 'a)>
    }

    impl<'a, K, V> Serialize for SerializeMapIterWrapper<'a, K, V> where
        K: Serialize + Any,
        V: Serialize
    {
        fn serialize<S>(& self, serializer: S) -> Result<S::Ok, S::Error> where
            S: Serializer
        {
            let mut ser_map = serializer.serialize_map(None)?;
            let mut iter = self.iter.borrow_mut();
            // handle strings specially so they don't get escaped and wrapped inside another string
            if TypeId::of::<K>() == TypeId::of::<String>() {
                while let Some((k, v)) = iter.next() {
                    let s = (k as &dyn Any).downcast_ref::<String>().ok_or(S::Error::custom("Failed to serialize String as string"))?;
                    ser_map.serialize_entry(s, &v)?;
                }
            } else {
                while let Some((k, v)) = iter.next() {
                    ser_map.serialize_entry(match &serde_json::to_string(&k)
                                            {
                                                Ok(key_string) => key_string,
                                                Err(e) => { return Err(e).map_err(S::Error::custom); }
                                            }, &v)?;
                }
            }
            ser_map.end()
        }
    }

    pub fn map_iter_to_json<'a, K, V>(iter: &'a mut dyn Iterator<Item=(&'a K, &'a V)>) -> Result<String, serde_json::Error> where
        K: Serialize + Any,
        V: Serialize
    {
        serde_json::to_string(&SerializeMapIterWrapper {
            iter: RefCell::new(iter)
        })
    }
}

pub fn read_json_as_stream(file_name: String) {
    let file = open_file(file_name);
    let mut reader = BufReader::new(file);
    let stream = Deserializer::from_reader(&mut reader).into_iter::<Value>();


    let mut btree: BTreeMap<String, Value> = BTreeMap::new();
    println!("btree: {}", serde_json_any_key::map_iter_to_json(&mut btree.iter())?);

}