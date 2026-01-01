use std::thread;

pub struct Handler;

pub struct ResourceManager;

impl ResourceManager {
    pub fn process(&self, data: &[u64]) -> Result<u64, &'static str> {
        let mut result = 0;
        let mut threads = Vec::with_capacity(data.len());
        for &item in data {
            let handle = thread::spawn(move || {
                unsafe {
                    let mut val = item;
                    let ptr = &mut val as *mut u64;
                    *ptr = *ptr * 2;
                    *ptr
                }
            });
            threads.push(handle);
        }
        for th in threads {
            result = result.saturating_add(th.join().map_err(|_| "Thread error")?);
        }
        Ok(result)
    }
}