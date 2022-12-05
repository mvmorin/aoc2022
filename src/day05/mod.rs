use std::fs;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn from(s: &str, n_stacks: usize) -> Self {
        let mut input = s.trim_end().split('\n').rev();
        input.next().unwrap();

        let mut stacks: Vec<Vec<char>> = Vec::new();
        for _column in 0..n_stacks {
            stacks.push(Vec::new());
        }

        for row in input.map(|s| s.chars().collect::<Vec<_>>()) {
            for i in 0..n_stacks {
                let c = row[1 + i * 4];
                if c.is_ascii_alphabetic() {
                    stacks[i].push(c);
                }
            }
        }

        Stacks { stacks }
    }

    fn move_one_by_one(&mut self, m: &Move) {
        for _ in 0..m.nbr {
            match self.stacks[m.from - 1].pop() {
                Some(c) => self.stacks[m.to - 1].push(c),
                None => panic!("input error"),
            }
        }
    }

    fn move_many(&mut self, m: &Move) {
        let from = &self.stacks[m.from - 1];
        let last = from.len();
        let first = last - m.nbr;

        for i in first..last {
            let c = self.stacks[m.from - 1][i];
            self.stacks[m.to - 1].push(c);
        }

        self.stacks[m.from - 1].drain(first..last);
    }
}

#[derive(Debug)]
struct Move {
    nbr: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from(s: &str) -> Self {
        let mut s = s.trim_end().split(' ');
        s.next();
        let nbr = s.next().unwrap().parse::<usize>().unwrap();
        s.next();
        let from = s.next().unwrap().parse::<usize>().unwrap();
        s.next();
        let to = s.next().unwrap().parse::<usize>().unwrap();

        Move { nbr, from, to }
    }
}

#[test]
fn day05() {
    let input = fs::read_to_string("src/day05/input.txt").unwrap();
    let mut input = input.trim_end().split("\n\n");

    let start_input = input.next().unwrap();
    let moves_input = input.next().unwrap();

    // part 1
    let mut stacks = Stacks::from(start_input, 9);
    let moves = moves_input.split('\n').map(|s| Move::from(s));

    for m in moves {
        stacks.move_one_by_one(&m);
    }

    for stack in stacks.stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");

    // part 2
    let mut stacks = Stacks::from(start_input, 9);
    let moves = moves_input.split('\n').map(|s| Move::from(s));

    for m in moves {
        stacks.move_many(&m);
    }

    for stack in stacks.stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}
