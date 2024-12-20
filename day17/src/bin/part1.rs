use std::{
    io::{stdin, BufReader},
    ops::BitXorAssign,
    process,
};

use day17::{parse_input, ExecutionError, Registers};

fn main() {
    // Read input
    let (regs, program) = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Execute program in the Virtual Machine
    let mut vm = VirtualMachine::new(regs, &program);
    let out = vm.execute().unwrap_or_else(|e| {
        eprintln!("VM error: {}", e);
        process::exit(1);
    });

    // Print result
    println!("Result: {}", out_to_string(&out));
}

#[derive(Debug)]
struct VirtualMachine<'a> {
    regs: Registers,
    ip: usize,
    mem: &'a [u8],
    out: Vec<u8>,
}

impl<'a> VirtualMachine<'a> {
    /// Instantiate a new Virtual Machine
    pub fn new(regs: Registers, mem: &'a [u8]) -> Self {
        Self {
            regs,
            ip: 0,
            mem,
            out: vec![],
        }
    }

    /// Execute the program loaded in the VM
    pub fn execute(&mut self) -> Result<Vec<u8>, ExecutionError> {
        loop {
            // Check if instruction is in range
            // With space for an operand as well
            if self.ip >= self.mem.len() - 1 {
                // Terminate execution
                break;
            }

            // Get current instruction and operand
            let inst = self.mem[self.ip];
            let operand = self.mem[self.ip + 1];

            // dbg!(&self.regs);
            // dbg!((self.ip, inst));

            // Execute instruction
            self.execute_instruction(inst, operand)?;
        }

        let res = self.out.clone();
        Ok(res)
    }

    // Resolve combo operand
    fn resovle_combo_operand(&self, operand: u8) -> Result<i64, ExecutionError> {
        match operand {
            0..=3 => Ok(operand as i64),
            4 => Ok(self.regs.a),
            5 => Ok(self.regs.b),
            6 => Ok(self.regs.c),
            _ => Err(ExecutionError::new()),
        }
    }

    // Execute instruction
    fn execute_instruction(&mut self, inst: u8, operand: u8) -> Result<(), ExecutionError> {
        // Increment instruction pointer
        self.ip += 2;

        // Select instruciton to execute
        match inst {
            0 => self.inst_adv(operand),
            1 => self.inst_bxl(operand),
            2 => self.inst_bst(operand),
            3 => self.inst_jnz(operand),
            4 => self.inst_bxc(),
            5 => self.inst_out(operand),
            6 => self.inst_bdv(operand),
            7 => self.inst_cdv(operand),
            _ => Err(ExecutionError::new()),
        }
    }

    // ADV instruction
    fn inst_adv(&mut self, operand: u8) -> Result<(), ExecutionError> {
        let operand_val = self.resovle_combo_operand(operand)?;
        let mut cur = self.regs.a;
        for _ in 0..operand_val {
            cur /= 2;
        }
        self.regs.a = cur;
        Ok(())
    }

    // BXL instruction
    fn inst_bxl(&mut self, operand: u8) -> Result<(), ExecutionError> {
        self.regs.b.bitxor_assign(operand as i64);
        Ok(())
    }

    // BST instruction
    fn inst_bst(&mut self, operand: u8) -> Result<(), ExecutionError> {
        // self.regs.b = self.resovle_combo_operand(operand)? % 8;
        self.regs.b = self.resovle_combo_operand(operand)? % 8;

        Ok(())
    }

    // JNZ instruction
    fn inst_jnz(&mut self, operand: u8) -> Result<(), ExecutionError> {
        if self.regs.a != 0 {
            self.ip = operand as usize;
        }

        Ok(())
    }

    // BXC instruction
    fn inst_bxc(&mut self) -> Result<(), ExecutionError> {
        self.regs.b.bitxor_assign(self.regs.c);
        Ok(())
    }

    // OUT instruction
    fn inst_out(&mut self, operand: u8) -> Result<(), ExecutionError> {
        let val = self.resovle_combo_operand(operand)? % 8;
        self.out.push(val as u8);
        Ok(())
    }

    // BDV instruction
    fn inst_bdv(&mut self, operand: u8) -> Result<(), ExecutionError> {
        let operand_val = self.resovle_combo_operand(operand)?;
        let mut cur = self.regs.a;
        for _ in 0..operand_val {
            cur /= 2;
        }
        self.regs.b = cur;
        Ok(())
    }

    // CDV instruction
    fn inst_cdv(&mut self, operand: u8) -> Result<(), ExecutionError> {
        let operand_val = self.resovle_combo_operand(operand)?;
        let mut cur = self.regs.a;
        for _ in 0..operand_val {
            cur /= 2;
        }
        self.regs.c = cur;
        Ok(())
    }
}

fn out_to_string(out: &[u8]) -> String {
    let strings: Vec<String> = out.iter().map(|val| val.to_string()).collect();
    strings.join(",")
}
