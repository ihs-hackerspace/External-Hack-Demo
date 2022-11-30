use toy_arms::external::Process;
use toy_arms::external::error::TAExternalError;
use toy_arms::external::{ read, write };
use winapi::ctypes::c_void;
use std::thread::sleep;
use std::time::Duration;

fn follow_offset_chain(
    process: *mut c_void, 
    module_base: usize, 
    offsets: &Vec<usize>
) -> Result<usize, TAExternalError> {
    // Create a mutable starting address
    let mut address = module_base;

    // Loop through the offsets, find the address they point to, and 
    // update the starting address for the next iteration
    for offset in offsets.iter().take(offsets.len() - 1) {
        // The pointer is stored as a 4-byte value so we need to read as a u32 and cast to a usize
        address = read::<u32>(process, address + *offset)? as usize;
    }

    // Return the final address if no errors occurred
    return Ok(address + offsets[offsets.len() - 1]);
}

fn entry_point() -> Result<(), TAExternalError> {
    // Create offset chains based on the game's memory found with Cheat Engine
    let health_offset_chain: Vec<usize> = vec![0x5D5444, 0xD8];
    let pistol_ammo_offset_chain: Vec<usize> = vec![0x5D6104, 0x4A8];
    
    // Get the process handle for the game
    let left_4_dead = Process::from_process_name("left4dead.exe")?;
    
    // Get the base address of the server.dll module to use as a base for the offset chains
    let server_dll = left_4_dead.get_module_base("server.dll")?;
    
    // Get the address of the health value
    let health_address = follow_offset_chain(left_4_dead.process_handle, server_dll, &health_offset_chain)?;
    println!("health_adr -> 0x{:x}", health_address);
    
    // Get the address of the pistol ammo value
    let ammo_address = follow_offset_chain(left_4_dead.process_handle, server_dll, &pistol_ammo_offset_chain)?;
    println!("ammo_address -> 0x{:x}", ammo_address);
    
    loop {
        // Read the health value
        let health = read::<u32>(left_4_dead.process_handle, health_address)?;
        println!("health: {}", health);
    
        // Read the pistol ammo value
        let ammo = read::<u32>(left_4_dead.process_handle, ammo_address)?;
        println!("ammo: {}", ammo);
    
        // Write to the health value to set it to 100
        write::<u32>(left_4_dead.process_handle, health_address, &mut 100)?;
    
        sleep(Duration::from_millis(1000));
    }
}

fn main() -> Result<(), TAExternalError> {
    // Call the entry point function and handle any errors
    match entry_point() {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        }
    }
}
