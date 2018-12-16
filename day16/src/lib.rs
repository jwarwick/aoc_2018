extern crate util;
#[macro_use] extern crate scan_fmt;

type Instruction = [i32; 4];
type Registers = [i32; 4];

#[derive(Debug)]
struct Input {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

impl Input {
    fn parse(l1: &str, l2: &str, l3: &str) -> Input {
        
        let (b1, b2, b3, b4) = scan_fmt!(l1,
                                         "Before: [{d}, {d}, {d}, {d}]",
                                         i32, i32, i32, i32);
        let (i1, i2, i3, i4) = scan_fmt!(l2,
                                         "{d} {d} {d} {d}",
                                         i32, i32, i32, i32);
        let (a1, a2, a3, a4) = scan_fmt!(l3,
                                         "After: [{d}, {d}, {d}, {d}]",
                                         i32, i32, i32, i32);
        Input {
            before: [b1.unwrap(), b2.unwrap(), b3.unwrap(), b4.unwrap()],
            instruction: [i1.unwrap(), i2.unwrap(), i3.unwrap(), i4.unwrap()],
            after: [a1.unwrap(), a2.unwrap(), a3.unwrap(), a4.unwrap()],
        }
    }
}

fn addr(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] +
        registers[instruction[2] as usize];
}

fn addi(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] +
        instruction[2];
}

fn mulr(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] *
        registers[instruction[2] as usize];
}

fn muli(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] *
        instruction[2];
}

fn banr(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] &
        registers[instruction[2] as usize];
}

fn bani(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] &
        instruction[2];
}

fn borr(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] |
        registers[instruction[2] as usize];
}

fn bori(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        registers[instruction[1] as usize] |
        instruction[2];
}

fn setr(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] = registers[instruction[1] as usize]
}

fn seti(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] = instruction[1];
}

fn gtir(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        if instruction[1] > registers[instruction[2] as usize] {
            1
        } else {
            0
        };
}

fn gtri(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        if registers[instruction[1] as usize] > instruction[2] {
            1
        } else {
            0
        };
}

fn gtrr(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        if registers[instruction[1] as usize] > registers[instruction[2] as usize] {
            1
        } else {
            0
        };
}

fn eqir(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        if instruction[1] == registers[instruction[2] as usize] {
            1
        } else {
            0
        };
}

fn eqri(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        if registers[instruction[1] as usize] == instruction[2] {
            1
        } else {
            0
        };
}

fn eqrr(instruction: &Instruction, registers: &mut Registers) {
    registers[instruction[3] as usize] =
        if registers[instruction[1] as usize] == registers[instruction[2] as usize] {
            1
        } else {
            0
        };
}
pub fn count_multiples(content: &str) -> usize {
    let mut inputs: Vec<Input> = Vec::new();
    let lines: Vec<_> = content.lines().collect();
    for i in lines.chunks(4) {
        let ls = i;
        let result = Input::parse(ls[0], ls[1], ls[2]);
        inputs.push(result);
    }

    let funcs: Vec<fn(&Instruction, &mut Registers)> =
        vec![addr, addi, mulr, muli, banr, bani, borr, bori,
        setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];

    let mut result: usize = 0;
    for i in inputs {
        let mut local_match = 0;
        for f in funcs.iter() {
            let mut registers = i.before.clone();
            f(&i.instruction, &mut registers);
            if registers == i.after {
                local_match += 1;
            }
        }
        if local_match >= 3 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let contents = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";
        assert_eq!(count_multiples(&contents), 1);
    }

    #[test]
    fn test_addi() {
        let mut r: Registers = [3, 0, 7, 1];
        let i: Instruction = [9, 2, 1, 1];
        addi(&i, &mut r);
        assert_eq!(r, [3, 8, 7, 1]);
    }

    #[test]
    fn test_addr() {
        let mut r: Registers = [3, 2, 1, 1];
        let i: Instruction = [9, 2, 1, 2];
        addr(&i, &mut r);
        assert_eq!(r, [3, 2, 3, 1]);
    }

    #[test]
    fn test_muli() {
        let mut r: Registers = [3, 0, 7, 1];
        let i: Instruction = [9, 2, 2, 1];
        muli(&i, &mut r);
        assert_eq!(r, [3, 14, 7, 1]);
    }

    #[test]
    fn test_mulr() {
        let mut r: Registers = [3, 4, 3, 1];
        let i: Instruction = [9, 2, 1, 2];
        mulr(&i, &mut r);
        assert_eq!(r, [3, 4, 12, 1]);
    }

    #[test]
    fn test_bani() {
        let mut r: Registers = [3, 0, 255, 16];
        let i: Instruction = [9, 2, 1, 3];
        bani(&i, &mut r);
        assert_eq!(r, [3, 0, 255, 1]);
    }

    #[test]
    fn test_banr() {
        let mut r: Registers = [3, 1, 255, 16];
        let i: Instruction = [9, 2, 1, 3];
        banr(&i, &mut r);
        assert_eq!(r, [3, 1, 255, 1]);
    }

    #[test]
    fn test_bori() {
        let mut r: Registers = [3, 0, 254, 16];
        let i: Instruction = [9, 2, 1, 1];
        bori(&i, &mut r);
        assert_eq!(r, [3, 255, 254, 16]);
    }

    #[test]
    fn test_borr() {
        let mut r: Registers = [3, 254, 1, 16];
        let i: Instruction = [9, 2, 1, 0];
        borr(&i, &mut r);
        assert_eq!(r, [255, 254, 1, 16]);
    }

    #[test]
    fn test_seti() {
        let mut r: Registers = [3, 0, 254, 16];
        let i: Instruction = [9, 2, 1, 1];
        seti(&i, &mut r);
        assert_eq!(r, [3, 2, 254, 16]);
    }

    #[test]
    fn test_setr() {
        let mut r: Registers = [3, 254, 1, 16];
        let i: Instruction = [9, 1, 1, 0];
        setr(&i, &mut r);
        assert_eq!(r, [254, 254, 1, 16]);
    }

    #[test]
    fn test_gtir() {
        let mut r: Registers = [3, 6, 1, 16];
        let i: Instruction = [9, 7, 1, 0];
        gtir(&i, &mut r);
        assert_eq!(r, [1, 6, 1, 16]);
    }

    #[test]
    fn test_gtri() {
        let mut r: Registers = [3, 6, 1, 16];
        let i: Instruction = [9, 1, 7, 0];
        gtri(&i, &mut r);
        assert_eq!(r, [0, 6, 1, 16]);
    }

    #[test]
    fn test_gtrr() {
        let mut r: Registers = [3, 6, 6, 16];
        let i: Instruction = [9, 2, 1, 2];
        gtrr(&i, &mut r);
        assert_eq!(r, [3, 6, 0, 16]);
    }

    #[test]
    fn test_eqir() {
        let mut r: Registers = [3, 6, 1, 16];
        let i: Instruction = [9, 7, 1, 0];
        eqir(&i, &mut r);
        assert_eq!(r, [0, 6, 1, 16]);
    }

    #[test]
    fn test_eqri() {
        let mut r: Registers = [3, 6, 1, 16];
        let i: Instruction = [9, 1, 7, 0];
        eqri(&i, &mut r);
        assert_eq!(r, [0, 6, 1, 16]);
    }

    #[test]
    fn test_eqrr() {
        let mut r: Registers = [3, 6, 6, 16];
        let i: Instruction = [9, 2, 1, 2];
        eqrr(&i, &mut r);
        assert_eq!(r, [3, 6, 1, 16]);
    }
}
