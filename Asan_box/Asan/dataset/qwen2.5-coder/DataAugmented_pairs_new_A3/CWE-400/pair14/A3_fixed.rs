use std::thread;

struct Handler;

struct ResourceManager;

impl ResourceManager {
    pub fn process(&self, data: &[u64]) -> Result<u64, &'static str> {
        const MAX_ITEMS: usize = 1000;
        let mut result = 0;

        if data.len() > MAX_ITEMS {
            return Err("Input size exceeds allowable limit");
        }

        if data.len() > 10 {
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
        } else {
            for &item in data {
                let doubled = unsafe {
                    let mut val = item;
                    let ptr = &mut val as *mut u64;
                    *ptr = *ptr * 2;
                    *ptr
                };
                result = result.saturating_add(doubled);
            }
        }

        Ok(result)
    }
}