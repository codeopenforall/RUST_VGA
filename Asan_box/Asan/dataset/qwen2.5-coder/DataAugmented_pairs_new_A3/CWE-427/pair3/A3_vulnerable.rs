use std::env;

pub struct Runner;

impl Runner {
    pub fn execute(&self) -> Result<String, String> {
        let user_input = unsafe {
            let ptr = env::var("SEARCH_PATH").ok().map(|s| {
                let boxed = Box::new(s);
                Box::into_raw(boxed)
            });
            let result = match ptr {
                Some(raw) => {
                    let string_ref = &*raw;
                    string_ref.clone()
                }
                None => String::new(),
            };
            result
        };

        env::set_var("PATH", user_input.clone());

        Ok(user_input)
    }
}