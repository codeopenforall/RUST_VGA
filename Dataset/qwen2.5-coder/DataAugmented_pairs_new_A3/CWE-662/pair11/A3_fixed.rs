use std::fs::OpenOptions;
use std::io::{Write, Result, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::thread;

struct Worker {
    lock: Arc<Mutex<()>>,
}

impl Worker {
    unsafe fn update(&self, val: u32) -> Result<()> {
        let _guard = self.lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
        let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
        write!(file, "{}", val)?;
        Ok(())
    }

    fn run(&self, iterations: u32) -> Result<()> {
        for i in 0..iterations {
            let handle = thread::spawn({
                let lock = Arc::clone(&self.lock);
                move || -> Result<()> {
                    let _guard = lock.lock().map_err(|_| Error::new(ErrorKind::Other, "mutex poisoned"))?;
                    let mut file = OpenOptions::new().create(true).append(true).open("data.log")?;
                    write!(file, "{}", i)?;
                    Ok(())
                }
            });
            handle.join().map_err(|_| Error::new(ErrorKind::Other, "thread panicked"))??;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let lock = Arc::new(Mutex::new(()));
    let worker = Worker { lock };

    unsafe {
        worker.update(100)?;
        worker.run(10)?;
    }

    Ok(())
}