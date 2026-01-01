use std::fs;
use std::io;
use std::path::PathBuf;

struct Resolver {
    path: String,
}

impl Resolver {
    fn resolve(&self) -> io::Result<PathBuf> {
        let raw_path = PathBuf::from(&self.path);
        let abs_path = fs::canonicalize(raw_path)?;
        let allowed = fs::canonicalize("./allowed")?;
        if !abs_path.starts_with(&allowed) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Access not permitted"));
        }
        Ok(abs_path)
    }
}

fn process_file(file_path: &str) -> io::Result<()> {
    let resolver = Resolver {
        path: file_path.to_string(),
    };
    let resolved_path = resolver.resolve()?;
    if let Ok(mut f) = fs::OpenOptions::new().write(true).create(false).open(&resolved_path) {
        let _ = f.write_all(b"safe update");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = if args.len() > 1 { &args[1] } else { "./allowed/test.txt" };
    if let Err(e) = process_file(file_path) {
        eprintln!("Error: {}", e);
    }
}