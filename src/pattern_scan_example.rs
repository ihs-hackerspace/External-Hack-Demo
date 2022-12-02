use std::time::Duration;
use std::thread::sleep;
use toy_arms::external::Process;
use toy_arms::external::Module;
use toy_arms::external::error::TAExternalError;
use toy_arms::external::write;

// Alias for an unsigned 8-bit integer
#[allow(non_camel_case_types)]
type byte = u8;

// Alias for a vector of bytes
type Bytes = Vec<byte>;

#[derive(Debug)]
pub enum Error {
    TAExternalError(TAExternalError),
    PatternScanError,
}
  
impl From<TAExternalError> for Error {
    fn from(error: TAExternalError) -> Self {
        Error::TAExternalError(error)
    }
}

// Define a structure that will hold our hex instructions, 
// and the starting address of those instructions in memory, 
// and whether or not the instructions have been patched
struct Instruction {
    instruction_bytes: Bytes,
    address: usize,
    disabled: bool
}

impl Instruction {
    // Create a new Instruction
    fn new(instruction_bytes: Bytes, address: usize) -> Self {
        Self {
            instruction_bytes,
            address,
            disabled: false
        }
    }

    // Create a new Instruction from a pattern scan
    fn from_pattern(instruction_bytes: Bytes, module: &Module, mask: &str) -> Result<Self, Error> {
        let address = module.find_pattern(&mask).ok_or(Error::PatternScanError)?;
        Ok(Self {
            instruction_bytes,
            address,
            disabled: false
        })
    }

    // Toggle off the instruction 
    // 
    // For context, the term "nop" comes from the x86 instruction "no operation" 
    // which is a 1 byte instruction that does nothing. This is useful because
    // we can replace the original instruction with a nop, and then when we want
    // to toggle the instruction back on, we can replace the nop with the original
    fn nop(&mut self, process: &Process) -> Result<(), Error> {
        for i in 0..self.instruction_bytes.len() {
            write::<byte>(process.process_handle, self.address + i as usize, &mut 0x90)?;
        }
        self.disabled = true;

        Ok(())
    }

    // Restore the orginal instruction
    fn restore(&mut self, process: &Process) -> Result<(), Error> {
        for (i, byte) in self.instruction_bytes.iter_mut().enumerate() {
            write::<byte>(process.process_handle, self.address + i, &mut *byte)?;
        }
        self.disabled = false;

        Ok(())
    }

    // Toggle the instruction on or off depending on its current state
    fn toggle(&mut self, process: &Process) -> Result<(), Error> {
        if self.disabled {
            self.restore(process)?;
        } else {
            self.nop(process)?;
        }

        Ok(())
    }
}

pub fn run() -> Result<(), Error> {
    // Get the process handle for the game
    println!("Getting left4dead process handle...");
    let left_4_dead = Process::from_process_name("left4dead.exe")?;
    
    // Get the base address of the server.dll module to use as a base for the offset chains
    println!("Getting server.dll module...");
    let server_dll = left_4_dead.get_module_info("server.dll")?;

    // Creating the health instruction
    println!("Getting health instruction...");
    let health_instruction_bytes: Bytes = vec![0x89, 0x37]; // Found using Cheat Engine "Find what writes to this address" feature
    let health_address = server_dll.find_pattern("?? ?? 5F B8 01 00 00 00 5E 83").ok_or(Error::PatternScanError)?;
    let mut health_instruction = Instruction::new(
        health_instruction_bytes,
        health_address, 
    );

    // Creating the ammo instruction
    println!("Getting ammo instruction...");
    let mut ammo_instruction = Instruction::from_pattern(
        vec![0x89, 0x2F], // Found using Cheat Engine "Find what writes to this address" feature
        &server_dll,
        "?? ?? 8B 07 DB 44 24 18", 
    )?;
    
    // Nop the instructions
    println!("Nopping the instructions");

    // Nop the health instruction. This will prevent the game from updating the health value
    // in memory, because the instruction the game uses to update the health value is now a nop
    // (no operation) instruction, which does nothing.
    health_instruction.nop(&left_4_dead)?;


    // Nop the ammo instruction. This will prevent the game from updating the ammo value 
    // for the same reason as above.
    ammo_instruction.nop(&left_4_dead)?;

    println!("Instructions nopped");

    // This will be used to toggle the instructions on and off every 10 seconds
    // to demonstrate that the instructions are being toggled on and off
    loop {
        health_instruction.toggle(&left_4_dead)?;
        ammo_instruction.toggle(&left_4_dead)?;

        if health_instruction.disabled && ammo_instruction.disabled {
            println!("Instructions nopped");
        } else {
            println!("Instructions restored");
        }

        sleep(Duration::from_millis(10000)); // Sleep for 10 seconds
    }

    // Allow unreachable code because I will probably comment out the loop above
    // in a demonstration, and I don't want to deal with the compiler complaining
    // about unreachable code. In the real world, you don't need to do this.
    #[allow(unreachable_code)]
    Ok(())
}