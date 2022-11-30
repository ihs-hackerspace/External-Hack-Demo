mod freeze_value_example;
mod pattern_scan_example;

fn main() {
    // Run the freeze value example
    // match freeze_value_example::entry_point() {
    //     Ok(_) => {}
    //     Err(e) => println!("Error: {}", e)
    // }

    // Run the pattern scan example
    match pattern_scan_example::entry_point() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e)
    }
}