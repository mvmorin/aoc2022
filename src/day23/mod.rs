#[test]
fn day23() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");

    // part 1
    let mut elves = parse_input(&input);
    for round in 0..10 {
        perform_round(&mut elves, round);
    }

    let (min_x,max_x,min_y,max_y) = find_bounds(&elves);
    let area = (max_x - min_x + 1)*(max_y - min_y + 1);
    // println!("{:?}", (min_x,max_x,min_y,max_y) );
    println!("{}", area - elves.len() as i32);

    // part 2
    let mut elves = parse_input(&input);
    let mut round = 0;
    while perform_round(&mut elves, round) { round += 1; }
    println!("{}", round+1);
}

use std::collections::HashMap;
use std::collections::HashSet;

fn coord_to_u64(x:i32,y:i32) -> u64 {
    let x = ((x as i64) & 0xFFFF_FFFF) as u64;
    let y = ((y as i64) & 0xFFFF_FFFF) as u64;
    (x << 32) | y
}

fn u64_to_coord(h: u64) -> (i32,i32) {
    let x = (h >> 32) as i32;
    let y = ((h << 32) >> 32) as i32;
    (x,y)
}

fn parse_input(s: &str) -> HashSet<u64> {
    let mut elves = HashSet::new();
    for (x,line) in s.lines().enumerate() {
        for (y,c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(coord_to_u64(x as i32, y as i32));
            }
        }
    }
    elves
}

// x down (from north, to south), y right (from west to east)
const N_NE_E_SE_S_SW_W_NW: [(i32,i32); 8] = [(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1)];

const N_NW_NE: [(i32,i32); 3] = [(-1,0),(-1,-1),(-1,1)];
const S_SW_SE: [(i32,i32); 3] = [(1,0),(1,-1),(1,1)];
const W_NW_SW: [(i32,i32); 3] = [(0,-1),(-1,-1),(1,-1)];
const E_NE_SE: [(i32,i32); 3] = [(0,1),(-1,1),(1,1)];

const DIRECTIONS: [&[(i32,i32)]; 4] = [&N_NW_NE, &S_SW_SE, &W_NW_SW, &E_NE_SE];

fn perform_round(elves: &mut HashSet<u64>, round_idx: usize) -> bool {
    let mut elves_to_move: HashMap<u64,u64> = HashMap::new();
    let mut proposed_moves: HashMap<u64,u32> = HashMap::new();

    for elf in elves.iter() {
        let (x,y) = u64_to_coord(*elf);

        let mut has_neigbour = false;
        for (dx,dy) in N_NE_E_SE_S_SW_W_NW.iter() {
            has_neigbour = has_neigbour || elves.contains( &coord_to_u64(x+dx,y+dy) );
        }

        if !has_neigbour { continue; }

        for dir_idx in 0..4 {
            let to_check = DIRECTIONS[ (dir_idx + round_idx) % 4 ];

            let mut has_neigbour = false;
            for (dx,dy) in to_check.iter() {
                has_neigbour = has_neigbour || elves.contains( &coord_to_u64(x+dx,y+dy) );
            }

            if has_neigbour { continue; }

            let move_to = coord_to_u64(x + to_check[0].0, y + to_check[0].1);
            elves_to_move.insert(*elf,move_to);
            proposed_moves
                .entry(move_to)
                .and_modify(|count| { *count += 1 })
                .or_insert(1);
            break;
        }
    }

    for (elf,move_to) in elves_to_move.iter() {
        if *proposed_moves.get(move_to).unwrap() == 1 {
            elves.remove(elf);
            elves.insert(*move_to);
        }
    }

    return elves_to_move.len() > 0;
}

fn find_bounds(elves: &HashSet<u64>) -> (i32,i32,i32,i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    for elf in elves.iter() {
        let (x,y) = u64_to_coord(*elf);
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    (min_x,max_x,min_y,max_y)
}

#[allow(dead_code)]
fn print_elves(elves: &HashSet<u64>) {
    let (min_x,max_x,min_y,max_y) = find_bounds(elves);
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if elves.contains( &coord_to_u64(x,y) ) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
