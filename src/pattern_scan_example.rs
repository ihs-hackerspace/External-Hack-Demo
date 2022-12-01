use toy_arms::external::Process;
use toy_arms::external::error::TAExternalError;
use toy_arms::external::Module;
use toy_arms::external::{ read, write };

struct Instruction {
    instruction_bytes: Vec<u8>,
    address: usize,
    disabled: bool
}

impl Instruction {
    // Create a new Instruction
    fn new(instruction_bytes: Vec<u8>, address: usize) -> Result<Self, TAExternalError> {
        Ok(Self {
            instruction_bytes,
            address,
            disabled: false
        })
    }

    // Create a new Instruction from a pattern scan
    fn from_pattern(instruction_bytes: Vec<u8>, module: &Module, mask: &str) -> Result<Self, TAExternalError> {
        let address = module.find_pattern(&mask).ok_or(TAExternalError::ModuleNotFound)?;
        Ok(Self {
            instruction_bytes: instruction_bytes,
            address,
            disabled: false
        })
    }

    // Toggle off the instruction
    fn nop(&mut self, process: &Process) -> Result<(), TAExternalError> {
        for i in 0..self.instruction_bytes.len() {
            write::<u8>(process.process_handle, self.address + i as usize, &mut 0x90)?;
        }
        self.disabled = true;

        Ok(())
    }

    // Restore the orginal instruction
    fn restore(&mut self, process: &Process) -> Result<(), TAExternalError> {
        for (i, byte) in self.instruction_bytes.iter_mut().enumerate() {
            write::<u8>(process.process_handle, self.address + i, &mut *byte)?;
        }
        self.disabled = false;

        Ok(())
    }

    // Toggle the instruction on or off depending on its current state
    fn toggle(&mut self, process: &Process) -> Result<(), TAExternalError> {
        if self.disabled {
            self.restore(process)?;
        } else {
            self.nop(process)?;
        }

        Ok(())
    }
}

pub fn entry_point() -> Result<(), TAExternalError> {
    // Get the process handle for the game
    println!("Getting left4dead process handle...");
    let left_4_dead = Process::from_process_name("left4dead.exe")?;
    
    // Get the base address of the server.dll module to use as a base for the offset chains
    println!("Getting server.dll module...");
    let server_dll = left_4_dead.get_module_info("server.dll")?;

    // Creating the health instruction
    println!("Getting health instruction...");
    let health_instruction_bytes: Vec<u8> = vec![0x89, 0x37];
    let health_address = server_dll.find_pattern("?? ?? 5F B8 01 00 00 00 5E 83").ok_or(TAExternalError::ModuleNotFound)?;
    let mut health_instruction = Instruction::new(
        health_instruction_bytes,
        health_address, 
    )?;

    // Creating the ammo instruction
    println!("Getting ammo instruction...");
    let ammo_instruction_bytes: Vec<u8> = vec![0x89, 0x2F];
    let mut ammo_instruction = Instruction::from_pattern(
        ammo_instruction_bytes,
        &server_dll,
        "?? ?? 8B 07 DB 44 24 18", 
    )?;
    
    // Nop the instructions
    println!("Nopping the instructions");

    health_instruction.nop(&left_4_dead)?;
    ammo_instruction.nop(&left_4_dead)?;

    println!("Instructions nopped");

    loop {
        health_instruction.toggle(&left_4_dead)?;
        ammo_instruction.toggle(&left_4_dead)?;

        if health_instruction.disabled && ammo_instruction.disabled {
            println!("Instructions nopped");
        } else {
            println!("Instructions restored");
        }

        std::thread::sleep(std::time::Duration::from_millis(10000));
    }

    Ok(())
}