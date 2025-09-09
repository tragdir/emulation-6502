use emulation_6502::*;

fn main() {
    let mut cpu: CPU = CPU::new();
    let mut mem: Mem = Mem::new();

    mem.write_byte(0xFFFC, 0x00);
    mem.write_byte(0xFFFD, 0x80);
    mem.write_byte(0x8000, CPU::INS_LDA_IM);
    mem.write_byte(0x8001, 0x08);
    mem.write_byte(0x8002, CPU::INS_ASL_ACC);

    println!("Resetting CPU");
    cpu.reset(&mem);
    println!("Printing CPU Status");
    cpu.print_status();
    println!("Executing 4 cycles");
    cpu.execute(4, &mut mem);
    println!("Printing status after execution");
    cpu.print_status();
}
