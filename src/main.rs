mod freeze_value_example;
mod pattern_scan_example;

enum Mode {
    FreezeValue,
    PatternScan,
}

fn main() {
    // Change this to change the example that is run
    let mode = Mode::FreezeValue;

    match mode {
        Mode::FreezeValue => freeze_value_example::entry_point().unwrap(),
        Mode::PatternScan => pattern_scan_example::entry_point().unwrap(),
    }
}