
use std::collections::HashMap;

pub struct QueryString<'buf> {
     map: HashMap<&'buf str, Value<'buf>>,
}
pub enum Value<'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
impl<'buf> QueryString<'buf>{
    fn get(&self, key: &str) -> Option<&Value>{
        self.map.get(key) 
    }
}

