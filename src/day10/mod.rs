#[test]
fn day10() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");

    // part 1
    let mut instructions = input.lines().map(parse_op);
    let mut cpu = CPU::new();
    let mut tot_signal_strength = 0;
    for c in 1..=220 {
        if (c as i32 - 20) % 40 == 0 {
            tot_signal_strength += c * cpu.register;
        }
        cycle(&mut cpu, &mut instructions);
    }
    println!("{}", tot_signal_strength);

    // part 2
    let mut instructions = input.lines().map(parse_op);
    let mut cpu = CPU::new();
    for _row in 0..6 {
        for pixel in 0..40 {
            if (pixel - cpu.register).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            cycle(&mut cpu, &mut instructions);
        }
        println!();
    }
}

#[derive(Debug)]
enum Ops {
    Addx(i32),
    Noop,
}

use Ops::*;

fn parse_op(line: &str) -> Ops {
    if let Some(val) = line.strip_prefix("addx ") {
        let val = val.parse::<i32>().unwrap();
        Ops::Addx(val)
    } else {
        Ops::Noop
    }
}

fn cycles_to_complete(op: &Ops) -> usize {
    match op {
        Addx(_) => 2,
        Noop => 1,
    }
}

struct CPU {
    register: i32,
    op: Ops,
    cycles_remaining: usize,
}

impl CPU {
    fn new() -> Self {
        CPU{register: 1, op: Noop, cycles_remaining: 0}
    }
}

fn cycle(cpu: &mut CPU, instructions: &mut impl Iterator<Item = Ops>) {
    if cpu.cycles_remaining == 0 {
        cpu.op = instructions.next().unwrap();
        cpu.cycles_remaining = cycles_to_complete(&cpu.op);
    };

    cpu.cycles_remaining -= 1;

    if cpu.cycles_remaining == 0 {
        match cpu.op {
            Noop => {},
            Addx(val) => {
                cpu.register += val;
            }
        };
    };
}
