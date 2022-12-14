// use std::collections::HashSet;

type Map = Vec<Vec<bool>>;

#[test]
fn day14() {
    let input = include_str!("input.txt");
    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    // println!("{}", input);

    // part 1
    let (mut map, lowest) = parse_input(input);
    let mut count = 0;
    while add_sand_no_floor(&mut map, lowest) {
        count += 1;
    }
    println!("{:?}", count);

    // part 2
    let (mut map, lowest) = parse_input(input);
    let mut count = 0;
    while add_sand_floor(&mut map, lowest) {
        count += 1;
    }
    println!("{:?}", count);
}

fn parse_input(input: &str) -> (Map, usize) {
    let mut map = Vec::new();
    for _ in 0..=1000 { // size is based on starting point
        map.push(vec![false; 500]);
    }

    let mut lowest = 0;

    for line in input.lines() {
        let mut points = line.split(" -> ");
        let (mut col_p, mut row_p) = parse_tuple(points.next().unwrap());
        map[col_p][row_p] = true;
        lowest = lowest.max(row_p);

        for s in points {
            let (col,row) = parse_tuple(s);

            let c_diff = col as isize - col_p as isize;
            let r_diff = row as isize - row_p as isize;
            let n_steps = (c_diff.abs()).max(r_diff.abs());

            for step in 1..=n_steps {
                let col_mid = (col_p as isize + step * c_diff.signum()) as usize;
                let row_mid = (row_p as isize + step * r_diff.signum()) as usize;
                map[col_mid][row_mid] = true;
                lowest = lowest.max(row_mid);
            }

            col_p = col;
            row_p = row;
        }
    }

    return (map, lowest)
}

fn parse_tuple(s: &str) -> (usize,usize) {
    let mut coords = s.split(',');
    let col = coords.next().unwrap().parse::<usize>().unwrap();
    let row = coords.next().unwrap().parse::<usize>().unwrap();
    (col,row)
}

fn add_sand_no_floor(map: &mut Map, lowest: usize) -> bool {
    if map[500][0] {
        return false;
    }

    let (mut col, mut row) = (500,0);
    loop {
        if row >= lowest {return false;}

        if !map[col][row+1] {
            row += 1;
        } else if !map[col-1][row+1] {
            row += 1;
            col -= 1;
        } else if !map[col+1][row+1] {
            row += 1;
            col += 1;
        } else {
            map[col][row] = true;
            return true;
        }
    }
}

fn add_sand_floor(map: &mut Map, lowest: usize) -> bool {
    if map[500][0] {
        return false;
    }

    let (mut col, mut row) = (500,0);
    loop {
        if row + 1 == lowest + 2 {
            map[col][row] = true;
            return true;
        } else if !map[col][row+1] {
            row += 1;
        } else if !map[col-1][row+1] {
            row += 1;
            col -= 1;
        } else if !map[col+1][row+1] {
            row += 1;
            col += 1;
        } else {
            map[col][row] = true;
            return true;
        }
    }
}
