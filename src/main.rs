mod prelude;
mod examples;
use examples::{pattern_scan, freeze_value};

#[allow(dead_code)]
enum Mode {
    FreezeValue,
    PatternScan,
}

fn main() {
    // Change this to change the example that is run
    let mode = Mode::PatternScan;

    match mode {
        Mode::FreezeValue => freeze_value::run().unwrap(),
        Mode::PatternScan => pattern_scan::run().unwrap(),
    }
}