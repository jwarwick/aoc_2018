type Instruction = (OpCode, i64, i64, i64);
type Registers = [i64; 6];

use std::collections::HashSet;

#[derive(Debug, Clone)]
enum OpCode {
    Nop,
    AddR, AddI,
    MulR, MulI,
    BanR, BanI,
    BorR, BorI,
    SetR, SetI,
    GtRI, GtIR, GtRR,
    EqRI, EqIR, EqRR,
}

#[derive(Debug)]
pub struct Device {
    registers: Registers,
    ip: i64,
    ip_register: usize,
    instructions: Vec<Instruction>,
}

impl Device {
    pub fn load(content: &str, registers: &Registers) -> Device {
        let ip =  0;
        let mut ip_register = 0;
        let mut instructions = Vec::new();

        for l in content.lines() {
            let s: Vec<_> = l.split_whitespace().collect();
            let opcode: String = s.get(0).expect("Opcode string").to_string();
            if opcode == "#ip" {
                ip_register = Device::num_at_index(&s, 1) as usize;
            } else {
                instructions.push((Device::op_string_to_code(&opcode),
                                   Device::num_at_index(&s, 1) as i64,
                                   Device::num_at_index(&s, 2) as i64,
                                   Device::num_at_index(&s, 3) as i64));

            }
        }

        Device {registers: *registers, ip, ip_register, instructions}
    }

    pub fn execute_max(&mut self, max_steps: &usize) -> usize{
        let mut s: HashSet<i64> = HashSet::new();
        let mut cnt: usize = 0;
        while self.ip_is_valid() && cnt < *max_steps {
            cnt = cnt + 1;
            self.registers[self.ip_register] = self.ip;
            let inst = self.instructions[self.ip as usize].clone();
            //if self.ip == 28 {
            //    let c = self.registers[2];
            //    if !s.contains(&c) {
            //        s.insert(c);
            //        println!("{}", c);
            //        
            //    }
            //    println!("{}\t{:?}\t{:?}", self.ip, inst, self.registers);
            //}
            self.op(&inst);
            self.ip = self.registers[self.ip_register];
            self.ip += 1;
            //println!("\t\t=>\t{}\t{:?}", self.ip, self.registers);
        }
        cnt
    }

    pub fn execute(&mut self) -> usize {
        self.execute_max(&std::usize::MAX)
    }

    pub fn registers(&self) -> Registers {
        self.registers.clone()
    }

    fn num_at_index(v: &Vec<&str>, i: usize) -> isize {
        let val_str: String = v.get(i).expect("Num at index").to_string();
        val_str.parse().expect("Number in string")
    }

    fn op_string_to_code(s: &String) -> OpCode {
        match s.as_str() {
            "addr" => OpCode::AddR,
            "addi" => OpCode::AddI,
            "mulr" => OpCode::MulR,
            "muli" => OpCode::MulI,
            "banr" => OpCode::BanR,
            "bani" => OpCode::BanI,
            "borr" => OpCode::BorR,
            "bori" => OpCode::BorI,
            "setr" => OpCode::SetR,
            "seti" => OpCode::SetI,
            "gtri" => OpCode::GtRI,
            "gtir" => OpCode::GtIR,
            "gtrr" => OpCode::GtRR,
            "eqri" => OpCode::EqRI,
            "eqir" => OpCode::EqIR,
            "eqrr" => OpCode::EqRR,
            c => {println!("Unknown opcode: {}", c); OpCode::Nop},
        }
    }

    fn ip_is_valid(&self) -> bool {
        self.ip >= 0 && self.ip < self.instructions.len() as i64
    }

    fn op(&mut self, instruction: &Instruction) {
        let op = &instruction.0;
        match op {
            OpCode::AddR => self.addr(&instruction),
            OpCode::AddI => self.addi(&instruction),
            OpCode::MulR => self.mulr(&instruction),
            OpCode::MulI => self.muli(&instruction),
            OpCode::BanR => self.banr(&instruction),
            OpCode::BanI => self.bani(&instruction),
            OpCode::BorR => self.borr(&instruction),
            OpCode::BorI => self.bori(&instruction),
            OpCode::SetR => self.setr(&instruction),
            OpCode::SetI => self.seti(&instruction),
            OpCode::GtRR => self.gtrr(&instruction),
            OpCode::GtRI => self.gtri(&instruction),
            OpCode::GtIR => self.gtir(&instruction),
            OpCode::EqRR => self.eqrr(&instruction),
            OpCode::EqRI => self.eqri(&instruction),
            OpCode::EqIR => self.eqir(&instruction),
            c => println!("Unknown instruction {:?}", c),
        }
    }

    fn addr(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] +
            self.registers[instruction.2 as usize];
    }

    fn addi(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] +
            instruction.2;
    }

    fn mulr(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] *
            self.registers[instruction.2 as usize];
    }

    fn muli(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] *
            instruction.2;
    }

    fn banr(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] &
            self.registers[instruction.2 as usize];
    }

    fn bani(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] &
            instruction.2;
    }

    fn borr(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] |
            self.registers[instruction.2 as usize];
    }

    fn bori(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            self.registers[instruction.1 as usize] |
            instruction.2;
    }

    fn setr(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] = self.registers[instruction.1 as usize]
    }

    fn seti(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] = instruction.1;
    }

    fn gtir(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            if instruction.1 > self.registers[instruction.2 as usize] {
                1
            } else {
                0
            };
    }

    fn gtri(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            if self.registers[instruction.1 as usize] > instruction.2 {
                1
            } else {
                0
            };
    }

    fn gtrr(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            if self.registers[instruction.1 as usize] > self.registers[instruction.2 as usize] {
                1
            } else {
                0
            };
    }

    fn eqir(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            if instruction.1 == self.registers[instruction.2 as usize] {
                1
            } else {
                0
            };
    }

    fn eqri(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            if self.registers[instruction.1 as usize] == instruction.2 {
                1
            } else {
                0
            };
    }

    fn eqrr(&mut self, instruction: &Instruction) {
        self.registers[instruction.3 as usize] =
            if self.registers[instruction.1 as usize] == self.registers[instruction.2 as usize] {
                1
            } else {
                0
            };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let contents = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

        let mut d = Device::load(contents, &[0; 6]);
        d.execute();
        let r = d.registers();
        assert_eq!(r[0], 6);
    }

    #[test]
    fn test_addi() {
        let r: Registers = [3, 0, 7, 1, 0, 0];
        let i: Instruction = (OpCode::AddI, 0, 7, 1);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 10, 7, 1, 0, 0]);
    }

    #[test]
    fn test_addr() {
        let r: Registers = [3, 0, 7, 1, 0, 0];
        let i: Instruction = (OpCode::AddR, 0, 2, 1);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 10, 7, 1, 0, 0]);
    }

    #[test]
    fn test_muli() {
        let r: Registers = [3, 0, 7, 1, 0, 0];
        let i: Instruction = (OpCode::MulI, 0, 2, 1);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 6, 7, 1, 0, 0]);
    }

    #[test]
    fn test_mulr() {
        let r: Registers = [3, 0, 7, 1, 0, 0];
        let i: Instruction = (OpCode::MulR, 0, 2, 3);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 0, 7, 21, 0, 0]);
    }

    #[test]
    fn test_bani() {
        let r: Registers = [3, 0, 255, 16, 0, 0];
        let i: Instruction = (OpCode::BanI, 2, 1, 3);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 0, 255, 1, 0, 0]);
    }

    #[test]
    fn test_banr() {
        let r: Registers = [3, 1, 255, 16, 0, 0];
        let i: Instruction = (OpCode::BanR, 2, 1, 3);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 1, 255, 1, 0, 0]);
    }

    #[test]
    fn test_bori() {
        let r: Registers = [3, 0, 254, 16, 0, 0];
        let i: Instruction = (OpCode::BorI, 2, 1, 1);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 255, 254, 16, 0, 0]);
    }

    #[test]
    fn test_borr() {
        let r: Registers = [3, 254, 1, 16, 0, 0];
        let i: Instruction = (OpCode::BorR, 2, 1, 0);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [255, 254, 1, 16, 0, 0]);
    }

    #[test]
    fn test_seti() {
        let r: Registers = [3, 0, 254, 16, 0, 0];
        let i: Instruction = (OpCode::SetI, 2, 1, 1);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 2, 254, 16, 0, 0]);
    }

    #[test]
    fn test_setr() {
        let r: Registers = [3, 254, 1, 16, 0, 0];
        let i: Instruction = (OpCode::SetR, 1, 1, 0);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [254, 254, 1, 16, 0, 0]);
    }

    #[test]
    fn test_gtir() {
        let r: Registers = [3, 6, 1, 16, 0, 0];
        let i: Instruction = (OpCode::GtIR, 7, 1, 0);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [1, 6, 1, 16, 0, 0]);
    }

    #[test]
    fn test_gtri() {
        let r: Registers = [3, 6, 1, 16, 0, 0];
        let i: Instruction = (OpCode::GtRI, 1, 7, 0);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [0, 6, 1, 16, 0, 0]);
    }

    #[test]
    fn test_gtrr() {
        let r: Registers = [3, 6, 1, 16, 0, 0];
        let i: Instruction = (OpCode::GtRR, 2, 1, 2);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 6, 0, 16, 0, 0]);
    }

    #[test]
    fn test_eqir() {
        let r: Registers = [3, 6, 1, 16, 0, 0];
        let i: Instruction = (OpCode::EqIR, 7, 1, 0);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [0, 6, 1, 16, 0, 0]);
    }

    #[test]
    fn test_eqri() {
        let r: Registers = [3, 6, 1, 16, 0, 0];
        let i: Instruction = (OpCode::EqRI, 1, 7, 0);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [0, 6, 1, 16, 0, 0]);
    }

    #[test]
    fn test_eqrr() {
        let r: Registers = [3, 6, 6, 16, 0, 0];
        let i: Instruction = (OpCode::EqRR, 2, 1, 2);
        let mut device = Device {ip: 0, ip_register: 5, registers: r, instructions: vec![i]};
        device.execute();
        assert_eq!(device.registers, [3, 6, 1, 16, 0, 0]);
    }
}
