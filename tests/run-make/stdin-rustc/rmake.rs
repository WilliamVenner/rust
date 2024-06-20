//! This test checks rustc `-` (stdin) support

use run_make_support::{is_windows, rustc};
use std::path::PathBuf;

const HELLO_WORLD: &str = r#"
fn main() {
    println!("Hello world!");
}
"#;

const NOT_UTF8: &[u8] = &[0xff, 0xff, 0xff];

fn main() {
    // echo $HELLO_WORLD | rustc -
    rustc().arg("-").stdin(HELLO_WORLD).run();
    assert!(
        PathBuf::from(if !is_windows() { "rust_out" } else { "rust_out.exe" })
            .try_exists()
            .unwrap()
    );

    // echo $NOT_UTF8 | rustc -
    rustc().arg("-").stdin(NOT_UTF8).run_fail().assert_stderr_contains(
        "error: couldn't read from stdin, as it did not contain valid UTF-8",
    );
}
