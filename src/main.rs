use std::fs;

fn main() {
    let pokemon_blue_rom = fs::read("Pokemon - Blue Version (USA, Europe) (SGB Enhanced).gb").unwrap();
    let bootloader_rom = fs::read("[BIOS] Nintendo Game Boy Boot ROM (World).gb").unwrap();

    let mut ps = ProgramState::new(pokemon_blue_rom, bootloader_rom);

    while let Ok(x) = ps.process_opcode() {
        println!("{}", x);
    }
}

#[derive(Default)]
struct ProgramState {
    register_af: u16,
    register_bc: u16,
    register_de: u16,
    register_hl: u16,
    stack_pointer: u16,
    program_counter: u16,

    game_rom: Vec<u8>,
    boot_rom: Vec<u8>
}

impl ProgramState {
    fn new(game_rom: Vec<u8>, boot_rom: Vec<u8>) -> Self {
        Self{
            game_rom: game_rom,
            boot_rom: boot_rom,

            ..Default::default()
        }
    }

    fn read(&self, addr: usize) -> u8 {
        if addr < 0x100 {
            self.boot_rom[addr]
        } else {
            self.game_rom[addr]
        }
    }

    fn process_opcode(&mut self) -> Result<u8, String> {
        let opcode = self.read(self.program_counter.into());
        match opcode {
            0x00 => Ok(0x00),
            0x21 => {
                println!("{:#04X} ld HL,nn", opcode);
                self.program_counter += 1;
                let lsb = self.read(self.program_counter.into());
                self.program_counter += 1;
                let msb = self.read(self.program_counter.into());
                self.register_hl = ((msb as u16) << 8) | (lsb as u16);
                println!("HL: {:#06X}", self.register_hl);
                self.program_counter += 1;
                Ok(0x21)
            },
            0x31 => {
                println!("{:#04X} ld SP,nn", opcode);
                self.program_counter += 1;
                let lsb = self.read(self.program_counter.into());
                self.program_counter += 1;
                let msb = self.read(self.program_counter.into());
                self.stack_pointer = ((msb as u16) << 8) | (lsb as u16);
                println!("SP: {:#06X}", self.stack_pointer);
                self.program_counter += 1;
                Ok(0x31)
            },
            0xAF => {
                println!("{:#04X} xor A", opcode);
                self.register_af ^= self.register_af << 8;
                if (self.register_af & 0xFF00) == 0 {
                    self.register_af |= 0x0080;
                }
                println!("AF: {:#06X}", self.register_af);
                self.program_counter += 1;
                Ok(0xAF)
            },
            _ => Err(format!("Undefined opcode {:#04X}", opcode))
        }
    }
}