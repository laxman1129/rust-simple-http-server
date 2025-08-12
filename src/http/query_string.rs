use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc
pub struct QueryString<'buf> {
    data : HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str), // single value, e.g. a=1
    Multiple(Vec<&'buf str>) // multiple values, e.g. d=7&d=abc
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// we use From trait to convert from &str to QueryString and not FromStr trait, because FromStr does not support lifetime parameters
// we are not using TryFrom trait here, because all strings are valid query strings, so we don't expect any errors during conversion.
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find("="){
                key = &sub_str[i..];
                val = &sub_str[i+1..];
            }

            // insert or update the value in the HashMap
            data.entry(key)
                .and_modify(|existing:&mut Value|{
                    match existing {
                        Value::Single(prev_val)=>{
                            // we added * to dereference the previous value, so we can use it
                            // dereferencing means changing the reference to the value it points to
                            // `existing` is a variable that points to some memory location, and we want to change the value at that memory location
                            // to be able to swap the memory the `existing` points to, we need to dereference it by adding `*` before it
                            // this means follow the pointer and write this new value over whatever it was pointing to earlier
                            *existing = Value::Multiple(vec![prev_val, val]);
                        },
                        Value::Multiple(vec) => {
                            vec.push(val); // if already a multiple value, just push the new value
                        }
                    }
                })
                .or_insert(Value::Single(val));


        }
        QueryString{data}
    }
}