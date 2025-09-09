use emulation_6502::*;

#[cfg(test)]

fn setup() -> (CPU, Mem) {
    let mut cpu: CPU = CPU::new();
    let mut mem: Mem = Mem::new();

    cpu.reset(&mem);

    mem.write_byte(0xFFFC, 0x00);
    mem.write_byte(0xFFFD, 0x80);
    
    cpu.reset(&mem);
    // allows access to current cpu and memory
    (cpu, mem)
}

#[test]
fn asl_acc_multiplies_the_accumulator_by_2() {
    let (mut cpu, mut mem): (CPU, Mem) = setup();
    cpu.print_status();

    mem.write_byte(0x8000, CPU::INS_LDA_IM);
    mem.write_byte(0x8001, 0x20);
    mem.write_byte(0x8002, CPU::INS_ASL_ACC);
    cpu.execute(4, &mut mem);
    assert!(*cpu.a() == 0x40);
    cpu.print_status();

    mem.write_byte(0x8003, CPU::INS_LDA_IM);
    mem.write_byte(0x8004, 0xEF);
    mem.write_byte(0x8005, CPU::INS_ASL_ACC);
    cpu.execute(4, &mut mem);
    cpu.print_status();

    assert!(*cpu.flags() & CPU::C == 0);
    assert!(*cpu.flags() & CPU::Z == 0);
    assert!(*cpu.flags() & CPU::I == 0);
    assert!(*cpu.flags() & CPU::D == 0);
    assert!(*cpu.flags() & CPU::B == 0);
    assert!(*cpu.flags() & CPU::UNUSED == CPU::UNUSED);
    assert!(*cpu.flags() & CPU::V == 0);
    assert!(*cpu.flags() & CPU::N == CPU::N);
    }
