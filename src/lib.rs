use derive_getters::Getters;

type Byte = u8;
type Word = u16;
const MEM_MAX: usize = 1024 * 64; // 64kB memory

pub struct Mem {
    data: Box<[Byte; MEM_MAX]>,
}

#[derive(PartialEq, Eq, Getters)]
pub struct CPU {
    pc: Word, // program counter
    sp: Byte, // Stack pointer
    a : Byte, // Accumulator
    x : Byte, // X register
    y : Byte, // Y register

    flags: Byte,
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            data: Box::new([0; MEM_MAX]),
        }
    }

    pub fn init(&mut self) {
        for i in 0..MEM_MAX {
            self.data[i] = 0;
        }
    }

    pub fn read_byte(&self, address: Word) -> Byte {
        println!("Reading address: 0x{:04X} ({})", address, address);
        assert!((address as usize) < MEM_MAX);
        self.data[address as usize]
    }

    pub fn write_byte(&mut self, address: Word, byte: Byte) {
        println!("Writing byte 0x{:02X} to address 0x{:04X} ({})", byte, address, address);
        assert!((address as usize) < MEM_MAX);
        self.data[address as usize] = byte;
    }
}

#[allow(dead_code)]
impl CPU {
    pub const C: Byte = 1 << 0; // Carry flag
    pub const Z: Byte = 1 << 1; // Zero flag
    pub const I: Byte = 1 << 2; // Interrupt Disable
    pub const D: Byte = 1 << 3; // Decimal Mode
    pub const B: Byte = 1 << 4; // Break
    pub const UNUSED: Byte = 1 << 5; // always set
    pub const V: Byte = 1 << 6; // Overflow flag
    pub const N: Byte = 1 << 7; // Negative flag
    
    
    pub fn new() -> CPU {
        CPU {
            pc : 0,
            sp : 0xFF,
            a  : 0,
            x  : 0,
            y  : 0,
            flags : Self::UNUSED,
        }
    }

    pub fn reset(&mut self, memory: &Mem) {
        self.pc = self.read_word(memory, 0xFFFC);
        self.sp = 0xFF;
        self.a  = 0;
        self.x  = 0;
        self.y  = 0;
        self.flags |= Self::UNUSED;
    }

    fn read_word(&self, memory: &Mem, address: Word) -> Word {
        let low_byte: Word = memory.read_byte(address) as Word;
        let high_byte: Word = memory.read_byte(address.wrapping_add(1)) as Word;
        return (high_byte << 8) | low_byte;
    }

    pub fn print_status(&self) {
        println!("Status:");
        println!(
            "PC:\t0X{:02X}{:02X} \nSP:\t0X{:02X} \na:\t0X{:02X} \nX:\t0X{:02X} \ny:\t0X{:02X} \nFlags:\t0b{:08b}",
            self.pc >> 8,
            self.pc & 0xFF,
            self.sp,
            self.a,
            self.x,
            self.y,
            self.flags
        );
    }

    fn fetch_byte(&mut self, cycles: &mut u32, memory: &Mem) -> Byte {
        let data: Byte = memory.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        *cycles = cycles.wrapping_sub(1);
        return data;
    }

    fn process_address_abs(&mut self, cycles: &mut u32, memory: &Mem) -> (Byte, Byte, Word) {
        let low_byte: Byte = self.fetch_byte(cycles, memory);
        let high_byte: Byte = self.fetch_byte(cycles, memory);
        return (low_byte, high_byte, (high_byte as Word) << 8 | low_byte as Word);
    }

    fn process_address_zp(&mut self, cycles: &mut u32, memory: &Mem) -> (Byte, Word) {
        let low_byte: Byte = self.fetch_byte(cycles, memory);
        let high_byte: Byte = 0x00; // zero page address
        return (low_byte, (high_byte as Word) << 8 | low_byte as Word);
    }
    
    /** Loads byte into Accumulator */
    pub const INS_LDA_IM: Byte = 0xA9;  
    /** Loads byte into X Register */                             
    pub const INS_LDX_IM: Byte = 0xA2;  
    /** Jumps to a Subroutine at given absolute address */
    pub const INS_JSR_AB: Byte = 0x20;  
    /** Does a Logical AND operation on the Accumulator */
    pub const INS_AND_IM: Byte = 0x29;  
    /** Does an Arithmetic Shift Left on the Accumulator */
    pub const INS_ASL_ACC: Byte = 0x0A; 
    /** Does an Arithmetic Shift Left on the Zero Page address */
    pub const INS_ASL_ZP: Byte = 0x06;  

    pub fn execute(&mut self, mut cycles: u32, memory: &mut Mem) {
        while cycles > 0 {
            let instruction: Byte = self.fetch_byte(&mut cycles, memory);
            println!(
                "Fetched instruction: 0x{:04X} at PC: 0x{:02X}{:02X}",
                instruction,
                self.pc >> 8,
                self.pc & 0xFF
            );
            match instruction {
                Self::INS_LDA_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.a = value;
                    self.flags &= !(Self::Z | Self::N); // clears relavent flags
                    if value == 0 {
                        self.flags |= Self::Z;
                    }
                    if value & Self::N != 0 {
                        self.flags |= Self::N;
                    }
                }

                Self::INS_LDX_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.x = value;
                    self.flags &= !(Self::Z | Self::N);
                    if value == 0 {
                        self.flags |= Self::Z;
                    }
                    if value & Self::N != 0 {
                        self.flags |= Self::N;
                    }
                }

                Self::INS_JSR_AB => {
                    let (low_byte, high_byte, dest_address) = self.process_address_abs(&mut cycles, memory);
                    memory.write_byte(0x01 << 8 | self.sp as Word, low_byte);
                    self.sp -= 1;
                    memory.write_byte(0x01 << 8 | self.sp as Word, high_byte);
                    self.sp -= 1;
                    self.pc = dest_address;
                    cycles -= 3;
                }
                
                Self::INS_AND_IM => {
                    let value: Byte = self.fetch_byte(&mut cycles, memory);
                    self.flags &= !(Self::Z | Self::N);
                    self.a &= value;
                    if self.a == 0 {
                        self.flags |= Self::Z;
                    }
                    if value & Self::N != 0 {
                        self.flags |= Self::N;
                    }
                }

                Self::INS_ASL_ACC => {
                    let temp_carry: Byte = self.a & 1 << 7;
                    self.flags &= !(Self::Z | Self::N | Self::C);
                    self.a <<= 1;
                    self.flags |= temp_carry;
                    if self.a == 0 {
                        self.flags |= Self::Z;
                    }
                    if self.a & Self::N != 0 {
                        self.flags |= Self::N;
                    }
                    cycles -= 1;
                }

                Self::INS_ASL_ZP => {
                    let (_low_byte, _address) = self.process_address_zp(&mut cycles, memory);
                    let mut value: Byte = self.fetch_byte(&mut cycles, memory);
                    let temp_carry: Byte = value & 1 << 7;
                    self.flags &= !(Self::Z | Self::N | Self::C);
                    value <<= 1;
                    self.flags |= temp_carry;
                    if value == 0 {
                        self.flags |= Self::Z;
                    }
                    if value & Self::N != 0 {
                        self.flags |= Self::N;
                    }
                    
                }

                _ => println!("Unknown instruction: 0x{:02X}", instruction),
            }
        }
    }
}
