
use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    pub query_map: HashMap<&'buf str, Value<'buf>>,
}
#[derive(Debug, PartialEq)]
pub enum Value<'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
impl<'buf> QueryString<'buf>{
    fn get(&self, key: &str) -> Option<&Value>{
        self.query_map.get(key) 
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self{
        let mut map = HashMap::new();

        for slice in s.split('&'){
            let mut key = slice;
            let mut val = "";
            if let Some(idx) = slice.find('='){
                key = &slice[..idx];
                val = &slice[idx +1..];
            }
            map.entry(key).and_modify(|operator| match operator  {
                                Value::Single(old_val) =>{
                                    *operator =  Value::Multiple(vec![val, old_val]);
                                },
                                Value::Multiple(vec) => {vec.push(val)},
                
                            })
                        .or_insert(Value::Single(val));
        }
        QueryString { query_map: map }
    }
}

