use std::collections::HashSet;

#[test]
fn day09() {
    let input = include_str!("input.txt");
    let moves = input.lines().map(parse_move).collect::<Vec<_>>();

    let mut string = vec![Knot{x:0,y:0}; 2];
    let res = count_covered_by_tail(&mut string, &moves);
    println!("{}", res);

    let mut string = vec![Knot{x:0,y:0}; 10];
    let res = count_covered_by_tail(&mut string, &moves);
    println!("{}", res);
}

fn count_covered_by_tail(string: &mut Vec<Knot>, moves: &Vec<(Direction,usize)>) -> usize {
    let mut visited_by_tail = HashSet::new();
    visited_by_tail.insert(string.last().unwrap().clone());

    for (dir,steps) in moves.iter() {
        for _ in 0..*steps {
            step_string(string, dir, &mut visited_by_tail);
        }
    }
    visited_by_tail.len()
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
struct Knot {
    x: i32,
    y: i32,
}

fn step_knot(knot: &Knot, dir: &Direction) -> Knot {
    let &Knot{x,y} = knot;
    match dir {
        Direction::Up => Knot{x,y:y+1},
        Direction::Left => Knot{x:x-1,y},
        Direction::Down => Knot{x,y:y-1},
        Direction::Right => Knot{x:x+1,y},
    }
}

fn catch_up(head: &Knot, tail: &Knot) -> Knot {
    let &Knot{x: hx,y: hy} = head;
    let &Knot{x: tx,y: ty} = tail;

    let dx = hx-tx;
    let dy = hy-ty;

    let (tx,ty) =
        if dx.abs() <= 1 && dy.abs() <= 1 {
            (tx,ty)
        } else if dx.abs() > 1 && dy.abs() == 0 {
            (tx + dx.signum(), ty)
        } else if dx.abs() == 0 && dy.abs() > 1 {
            (tx, ty + dy.signum())
        } else {
            (tx + dx.signum(), ty + dy.signum())
        };

    Knot{x:tx,y:ty}
}

fn step_string(string: &mut Vec<Knot>, dir: &Direction, visited_by_tail: &mut HashSet<Knot>) {
    let n_knots = string.len();

    let new_head = step_knot(&string[0], dir);
    string[0] = new_head;

    for t_idx in 1..n_knots {
        let new_tail = catch_up(&string[t_idx-1], &string[t_idx]);
        string[t_idx] = new_tail;
    }

    visited_by_tail.insert(string[n_knots-1].clone());
}

fn parse_move(line: &str) -> (Direction, usize) {
    let mut parts = line.split(' ');
    let dir = match parts.next().unwrap() {
        "U" => Direction::Up,
        "L" => Direction::Left,
        "D" => Direction::Down,
        "R" => Direction::Right,
        _ => panic!("invalid input"),
    };
    let steps = parts.next().unwrap().parse::<usize>().unwrap();

    (dir,steps)
}
