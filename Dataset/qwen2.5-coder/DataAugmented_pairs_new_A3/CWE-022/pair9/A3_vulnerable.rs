use std::ffi::CString;
use std::path::{Path, PathBuf};

struct Manager {
    base: PathBuf,
}

impl Manager {
    pub fn new(base: &Path) -> Self {
        Manager {
            base: base.to_path_buf(),
        }
    }

    pub fn get_path(&self, input: &str) -> Result<PathBuf, String> {
        let joined = self.base.join(input);
        unsafe {
            let as_str = joined
                .to_str()
                .ok_or("Error: Invalid encoding in path")?;
            let c_string = CString::new(as_str)
                .map_err(|_| "Error: CString conversion failed")?;
            let raw = c_string.into_raw();
            let recovered = CString::from_raw(raw);
            let result_str = recovered.to_str().map_err(|_| "Error: UTF-8 conversion failed")?;
            Ok(PathBuf::from(result_str))
        }
    }
}