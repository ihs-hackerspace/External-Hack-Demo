use std::cell::RefCell;
use toy_arms::external::Process;
use toy_arms::external::error::TAExternalError;
use toy_arms::external::Module;
use toy_arms::external::{ read, write };

struct InstructionPattern {
    pattern: RefCell<Vec<u8>>,
    address: usize,
    toggled: bool
}

impl InstructionPattern {
    // Create a new InstructionPattern
    fn new(module: &Module, mask: String, pattern: Vec<u8>) -> Result<Self, TAExternalError> {
        let address = module.find_pattern(&mask).ok_or(TAExternalError::ProcessNotFound)?;
        Ok(Self {
            pattern: RefCell::new(pattern),
            address: address,
            toggled: false
        })
    }

    // Toggle off the instruction
    fn nop(&mut self, process: &Process) -> Result<(), TAExternalError> {
        for i in 0..self.pattern.borrow().len() {
            write::<u8>(process.process_handle, self.address + i as usize, &mut 0x90)?;
        }
        self.toggled = true;
        Ok(())
    }

    // Toggle on the instruction
    fn restore(&mut self, process: &Process) -> Result<(), TAExternalError> {
        for (i, byte) in self.pattern.borrow_mut().iter_mut().enumerate() {
            write::<u8>(process.process_handle, self.address + i, &mut *byte)?;
        }
        self.toggled = false;
        Ok(())
    }

    // Toggle the instruction on or off depending on its current state
    fn toggle_nop(&mut self, process: &Process) -> Result<(), TAExternalError> {
        if self.toggled {
            self.restore(process)?;
        } else {
            self.nop(process)?;
        }
        Ok(())
    }
}

pub fn entry_point() -> Result<(), TAExternalError> {
    // Get the process handle for the game
    let left_4_dead = Process::from_process_name("left4dead.exe")?;
    
    // Get the base address of the server.dll module to use as a base for the offset chains
    let server_dll =left_4_dead.get_module_info("server.dll")?;

    let health_pattern: Vec<u8> = vec![0x89, 0x37];
    let mut health_instructions = InstructionPattern::new(
        &server_dll,
        "?? ?? 5F B8 01 00 00 00 5E 83".to_string(), 
        health_pattern,
    )?;

    let ammo_pattern: Vec<u8> = vec![0x89, 0x2F];
    let mut ammo_instructions = InstructionPattern::new(
        &server_dll,
        "?? ?? 8B 07 DB 44 24 18".to_string(), 
        ammo_pattern,
    )?;
    
    // Nop the instructions
    health_instructions.nop(&left_4_dead)?;
    ammo_instructions.nop(&left_4_dead)?;

    // loop {
    //     health_instructions.toggle_nop(&left_4_dead)?;
    //     ammo_instructions.toggle_nop(&left_4_dead)?;

    //     if health_instructions.toggled && ammo_instructions.toggled {
    //         println!("Toggled both");
    //     } else {
    //         println!("Toggled off both");
    //     }

    //     std::thread::sleep(std::time::Duration::from_millis(10000));
    // }

    return Ok(());
}