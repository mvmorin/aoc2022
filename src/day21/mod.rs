use std::collections::HashMap;

#[test]
fn day21() {
    let input = include_str!("input.txt");
    let monkeys = parse_monkeys(input);

    // part 1
    let root = get_value("root", &monkeys);
    println!("{}", root);

    // part 2
    let mut cache = HashMap::new();
    let root = monkeys.get("root").unwrap();
    let val = if on_human_branch(&root.a, &monkeys, &mut cache) {
        let val = get_value(&root.b, &monkeys);
        propagate_to_human(val, &root.a, &monkeys, &mut cache)
    } else if on_human_branch(&root.b, &monkeys, &mut cache) {
        let val = get_value(&root.a, &monkeys);
        propagate_to_human(val, &root.b, &monkeys, &mut cache)
    } else {
        panic!("trying to propagate to human on tree without human");
    };
    println!("{}", val);
}

#[derive(Debug)]
struct Monkey {
    op: Operator,
    val: u64,
    a: String,
    b: String,
}

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Div,
    Mul,
    Const,
}

fn parse_monkeys(s: &str) -> HashMap<String,Monkey> {
    let mut monkeys = HashMap::new();

    for line in s.lines() {
        let mut parts = line.split([':', ' ']);
        let name = parts.nth(0).unwrap().to_string();

        let val_or_a = parts.nth(1).unwrap();

        let m = if let Ok(val) = val_or_a.parse::<u64>() {
            Monkey {
                op: Operator::Const,
                val,
                a: "".to_string(),
                b: "".to_string(),
            }
        } else {
            let a = val_or_a.to_string();
            let op = match parts.nth(0).unwrap() {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "/" => Operator::Div,
                "*" => Operator::Mul,
                _ => panic!("invalid input"),
            };
            let b = parts.nth(0).unwrap().to_string();

            Monkey { op, val: 0, a, b, }
        };

        monkeys.insert(name, m);
    }

    monkeys
}

fn get_value(name: &str, monkeys: &HashMap<String,Monkey>) -> u64 {
    let monkey = monkeys.get(name).unwrap();

    if let Operator::Const = monkey.op { return monkey.val; }

    let a_val = get_value(&monkey.a, monkeys);
    let b_val = get_value(&monkey.b, monkeys);
    match monkey.op {
        Operator::Add => { a_val + b_val },
        Operator::Sub => { a_val - b_val },
        Operator::Div => { a_val / b_val },
        Operator::Mul => { a_val * b_val },
        Operator::Const => { panic!("we should have guarded against this") },
    }
}

fn on_human_branch<'a>(name: &'a str, monkeys: &'a HashMap<String,Monkey>, cache: &mut HashMap<&'a str,bool>) -> bool {
    if cache.contains_key(name) { return *cache.get(name).unwrap(); }

    let monkey = monkeys.get(name).unwrap();
    match monkey.op {
        Operator::Const => { name == "humn" },
        _ => {
            on_human_branch(&monkey.a, monkeys, cache)
            || on_human_branch(&monkey.b, monkeys, cache)
        },
    }
}

fn propagate_to_human<'a>(val: u64, name: &str, monkeys: &'a HashMap<String,Monkey>, cache: &mut HashMap<&'a str, bool>) -> u64 {
    let monkey = monkeys.get(name).unwrap();

    if let Operator::Const = monkey.op {
        if name != "humn" { panic!("trying to propagate human value to wrong leaf"); };
        return val;
    }

    let (human_on_a, non_human_val) =
        if on_human_branch(&monkey.a, monkeys, cache) {
            (true, get_value(&monkey.b, monkeys))
        } else if on_human_branch(&monkey.b, monkeys, cache) {
            (false,get_value(&monkey.a, monkeys))
        } else {
            panic!("trying to propagate to human on tree without human");
        };

    match monkey.op {
        Operator::Add => {
            if human_on_a {
                return propagate_to_human(val - non_human_val, &monkey.a, monkeys, cache);
            } else {
                return propagate_to_human(val - non_human_val, &monkey.b, monkeys, cache);
            }
        },
        Operator::Sub => {
            if human_on_a {
                return propagate_to_human(val + non_human_val, &monkey.a, monkeys, cache);
            } else {
                return propagate_to_human(non_human_val - val, &monkey.b, monkeys, cache);
            };
        },
        Operator::Div => {
            if human_on_a {
                return propagate_to_human(val * non_human_val, &monkey.a, monkeys, cache);
            } else {
                return propagate_to_human(non_human_val / val, &monkey.b, monkeys, cache);
            };
        },
        Operator::Mul => {
            if human_on_a {
                return propagate_to_human(val / non_human_val, &monkey.a, monkeys, cache);
            } else {
                return propagate_to_human(val / non_human_val, &monkey.b, monkeys, cache);
            };
        },
        Operator::Const => { panic!("we should have guarded against this") },
    };
}

