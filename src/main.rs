mod freeze_value_example;
mod pattern_scan_example;

#[allow(dead_code)]
enum Mode {
    FreezeValue,
    PatternScan,
}

fn main() {
    // Change this to change the example that is run
    let mode = Mode::PatternScan;

    match mode {
        Mode::FreezeValue => freeze_value_example::run().unwrap(),
        Mode::PatternScan => pattern_scan_example::run().unwrap(),
    }
}