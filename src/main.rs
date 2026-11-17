//! SigmaOS-environment-reproducibility — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_environment_reproducibility_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-environment-reproducibility: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-environment-reproducibility: ok");
}
