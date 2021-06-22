use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::io::prelude::*;
use std::hash::Hash;
use std::path::PathBuf;
use std::fs;

#[derive(Clone)]
pub struct PersistConfig {
    path: PathBuf
}

impl PersistConfig {
    pub fn new(path: String) -> PersistConfig {
        PersistConfig {
            path: PathBuf::from(path)
        }
    }
}

pub struct KVCache<Key: Hash + Eq + Serialize + DeserializeOwned, 
    Value: Serialize + DeserializeOwned> {
    // If true, persists cache on disk
    persist_config: Option<PersistConfig>,
    map: HashMap<Key, Value>
}

impl<Key: Hash + Eq + Serialize + DeserializeOwned, 
    Value: Serialize + DeserializeOwned> 
    KVCache<Key, Value> {
    pub fn new(persist_config: Option<PersistConfig>) -> KVCache<Key, Value> {
        match persist_config.clone() {
            None => KVCache {
                persist_config,
                map: HashMap::new()
            },
            Some(conf) => {
                match fs::File::open(conf.path.clone()) {
                    Err(_) => KVCache {
                        persist_config,
                        map: HashMap::new()
                    },
                    Ok(file) => {
                        let map: HashMap<Key, Value> 
                            = serde_json::from_reader(file).unwrap();

                        KVCache {
                            persist_config,
                            map
                        }
                    }
                }

            }
        }
    }

    pub fn add(&mut self, k: Key, v: Value) -> Option<Value> {
        let conf = self.persist_config.clone();
        match conf {
            None => self.map.insert(k, v),
            Some(conf) => {
                let r = self.map.insert(k, v);

                let mut file = fs::File::create(conf.path.clone()).unwrap();

                let serialized_map = serde_json::to_string(&self.map).unwrap();
                    

                file.write_all(serialized_map.as_bytes())
                    .expect("Failed to persist cache");

                r
            }

        }
    }

    pub fn get(&self, k: &Key) -> Option<&Value> {
        self.map.get(k)
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, Key, Value> {
        self.map.keys()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_kvcache() {
        let mut cache: KVCache<String, String> = KVCache::new(None);

        cache.add("hi".to_owned(), "ho".to_owned());
        assert!(cache.get(&"hi".to_owned()).unwrap() == &"ho".to_owned());
    }

    #[test]
    fn simple_persisted_kvcache() {
        let mut cache: KVCache<String, String> = KVCache::new(
            Some(PersistConfig::new("kvcache_test.json".to_owned())));

        cache.add("hi".to_owned(), "ho".to_owned());
        cache.add("hii".to_owned(), "ho".to_owned());
        assert!(cache.get(&"hi".to_owned()).unwrap() == &"ho".to_owned());
    }
}
