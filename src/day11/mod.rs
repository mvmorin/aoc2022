use std::collections::VecDeque;

#[test]
fn day11() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    // part 1
    let mut monkeys: VecDeque<Monkey> = input.split("\n\n").map(|s| Monkey::from(s)).collect();
    let worry_mod = monkeys.iter().map(|m| m.test_div).product();
    for _ in 0..20 {
        round(&mut monkeys, 3, worry_mod);
    }
    // println!("{:#?}", monkeys);

    let (insp1, insp2) = two_largest(monkeys.iter().map(|m| m.n_inspected));
    println!("{}*{} = {:?}", insp1, insp2, insp1*insp2);

    // part 2
    let mut monkeys: VecDeque<Monkey> = input.split("\n\n").map(|s| Monkey::from(s)).collect();
    let worry_mod = monkeys.iter().map(|m| m.test_div).product();
    for _ in 0..10_000 {
        round(&mut monkeys, 1, worry_mod);
    }
    // println!("{:#?}", monkeys);

    let (insp1, insp2) = two_largest(monkeys.iter().map(|m| m.n_inspected));
    println!("{}*{} = {:?}", insp1, insp2, insp1*insp2);
}

fn round(monkeys: &mut VecDeque<Monkey>, calm_div: u64, worry_mod: u64) {
    for monkey_idx in 0..monkeys.len() {
        while let Some((to,worry)) = monkeys[monkey_idx].inspect(calm_div, worry_mod) {
            monkeys[to].items.push_back(worry);
        }
    }
}

fn two_largest(it: impl Iterator<Item = u64>) -> (u64, u64){
    let mut largest = 0;
    let mut second_largest = 0;
    for val in it {
        if val > largest {
            second_largest = largest;
            largest = val;
        } else if val > second_largest {
            second_largest = val;
        }
    }

    (largest, second_largest)
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    op_mul: u64,
    op_add: u64,
    op_pow: u32,
    test_div: u64,
    true_pass_to: usize,
    false_pass_to: usize,
    n_inspected: u64,
}

impl Monkey {
    fn from(s: &str) -> Self {
        let mut l = s.lines();

        let _m_str = l.next().unwrap().strip_prefix("Monkey ").unwrap();
        let items_str = l.next().unwrap().strip_prefix("Starting items: ").unwrap();
        let op_str = l.next().unwrap().strip_prefix("Operation: new = ").unwrap();
        let test_str = l.next().unwrap().strip_prefix("Test: divisible by ").unwrap();
        let true_str = l.next().unwrap().strip_prefix("  If true: throw to monkey ").unwrap();
        let false_str = l.next().unwrap().strip_prefix("  If false: throw to monkey ").unwrap();

        let items: VecDeque<u64> = items_str.split(", ").map(|s| s.parse::<u64>().unwrap()).collect();
        let (op_mul, op_add, op_pow) =
            if let Some(_) = op_str.strip_prefix("old * old") {
                (1, 0, 2)
            } else if let Some(val_str) = op_str.strip_prefix("old + ") {
                (1, val_str.parse::<u64>().unwrap(), 1)
            } else if let Some(val_str) = op_str.strip_prefix("old * ") {
                (val_str.parse::<u64>().unwrap(), 0, 1)
            } else {
                panic!("input error");
            };

        let test_div = test_str.parse::<u64>().unwrap();
        let true_pass_to = true_str.parse::<usize>().unwrap();
        let false_pass_to = false_str.parse::<usize>().unwrap();

        Monkey{items, op_mul, op_add, op_pow, test_div, true_pass_to, false_pass_to, n_inspected: 0}
    }

    fn inspect(&mut self, calm_div: u64, worry_mod: u64) -> Option<(usize, u64)> {
        let worry = self.items.pop_front()?;

        let worry = (worry.pow(self.op_pow) * self.op_mul) + self.op_add;
        let worry = worry / calm_div;
        let worry = worry % worry_mod;

        let pass_to = if worry % self.test_div == 0 {
            self.true_pass_to
        } else {
            self.false_pass_to
        };

        self.n_inspected += 1;

        Some((pass_to, worry))
    }
}
