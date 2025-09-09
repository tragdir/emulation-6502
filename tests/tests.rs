use emulation_6502::*;

#[cfg(test)]

fn setup() -> (CPU, Mem) {
    let mut cpu: CPU = CPU::new();
    let mut mem: Mem = Mem::new();

    cpu.reset(&mem);

    mem.write_byte(0xFFFC, 0x00);
    mem.write_byte(0xFFFD, 0x80);
    
    cpu.reset(&mem);

    cpu.print_status();
    // allows access to current cpu and memory
    (cpu, mem)
}

#[test]
fn asl_acc_multiplies_the_accumulator_by_2() {
    let (mut cpu, mut mem): (CPU, Mem) = setup();

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
    }

#[test]
fn jsr_jumps_to_correct_address_given() {
    let (mut cpu, mut mem): (CPU, Mem) = setup();

    // JSR takes 6 cycles to execute  
    mem.write_byte(0x8000, CPU::INS_JSR_AB);
    // jump to 0x5049
    mem.write_byte(0x8001, 0x49);
    mem.write_byte(0x8002, 0x50);

    cpu.execute(6, &mut mem);
    
    // print the status of the cpu after jsr
    cpu.print_status();

    assert!(*cpu.pc() == 0x5049)
}

#[test]
fn rts_jumps_back_from_jsr_call() {
    let (mut cpu, mut mem): (CPU, Mem) = setup();

    mem.write_byte(0x8000, CPU::INS_JSR_AB);
    mem.write_byte(0x8001, 0x49);
    mem.write_byte(0x8002, 0x50);
    mem.write_byte(0x5049, CPU::INS_LDX_IM);
    mem.write_byte(0x504A, 0x02);
    mem.write_byte(0x504B, CPU::INS_RTS);

    cpu.execute(14, &mut mem);
    
    cpu.print_status();
    // should jump back to 0x8003
    assert!(*cpu.pc() == 0x8003);
}
