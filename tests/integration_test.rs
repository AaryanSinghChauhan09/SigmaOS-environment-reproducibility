// Integration test for SigmaOS-environment-reproducibility
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use environment_reproducibility as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
