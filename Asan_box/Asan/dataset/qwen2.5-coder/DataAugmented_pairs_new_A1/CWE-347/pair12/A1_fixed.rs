use std::sync::Arc;
use std::thread;

#[derive(Clone)]
struct Verifier;

impl Verifier {
    pub fn validate(&self, message: &str, signature: &[u8], domain: &str) -> bool {
        // Remove the unsafe block and check if the domain is valid
        domain == "valid_domain"
    }
}

fn process(arc_verifier: Arc<Verifier>, msg: &'static str, sig: &'static [u8], dom: &'static str) -> bool {
    arc_verifier.validate(msg, sig, dom)
}

fn main() {
    let verifier = Arc::new(Verifier);
    let message = "Important message";
    let signature = b"fake_signature"; 
    let domain = "valid_domain"; 
    let handles: Vec<_> = (0..4).map(|_| {
        let verifier_c = Arc::clone(&verifier);
        thread::spawn(move || {
            process(verifier_c, message, signature, domain)
        })
    }).collect();
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    for valid in results {
        if !valid {
            println!("Verification error");
            std::process::exit(1);
        }
    }
    println!("Verification passed");
}