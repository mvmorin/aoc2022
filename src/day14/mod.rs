use std::collections::HashSet;

type Map = HashSet<(i32,i32)>;

#[test]
fn day14() {
    let input = include_str!("input.txt");
    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    // println!("{}", input);

    // part 1
    let mut map = parse_input(input);
    let lowest = find_lowest(&map);
    let mut count = 0;
    while add_sand_no_floor(&mut map, (500,0), lowest) {
        count += 1;
    }
    println!("{:?}", count);

    // part 2
    let mut map = parse_input(input);
    let lowest = find_lowest(&map);
    let mut count = 0;
    while add_sand_floor(&mut map, (500,0), lowest) {
        count += 1;
    }
    println!("{:?}", count);
}

fn parse_input(input: &str) -> Map {
    let mut map = HashSet::new();

    for line in input.lines() {
        let mut points = line.split(" -> ");
        let (mut col_p, mut row_p) = parse_tuple(points.next().unwrap());
        map.insert((col_p,row_p));

        for s in points {
            let (col,row) = parse_tuple(s);

            let c_diff = col - col_p;
            let r_diff = row - row_p;
            let n_steps = (c_diff.abs()).max(r_diff.abs());

            for step in 1..=n_steps {
                let col_mid = col_p + step * c_diff.signum();
                let row_mid = row_p + step * r_diff.signum();
                map.insert((col_mid,row_mid));
            }

            col_p = col;
            row_p = row;
        }
    }

    return map
}

fn parse_tuple(s: &str) -> (i32,i32) {
    let mut coords = s.split(',');
    let col = coords.next().unwrap().parse::<i32>().unwrap();
    let row = coords.next().unwrap().parse::<i32>().unwrap();
    (col,row)
}

fn find_lowest(map: &Map) -> i32 {
    map.iter().map(|k| k.1).max().unwrap()
}

fn add_sand_no_floor(map: &mut Map, start_point: (i32,i32), lowest: i32) -> bool {
    if map.contains(&start_point) {
        return false;
    }

    let (mut col, mut row) = start_point;
    loop {
        if row >= lowest {return false;}

        if !map.contains(&(col,row+1)) {
            row += 1;
        } else if !map.contains(&(col-1,row+1)) {
            row += 1;
            col -= 1;
        } else if !map.contains(&(col+1,row+1)) {
            row += 1;
            col += 1;
        } else {
            map.insert((col,row));
            return true;
        }
    }
}

fn add_sand_floor(map: &mut Map, start_point: (i32,i32), lowest: i32) -> bool {
    if map.contains(&start_point) {
        return false;
    }

    let (mut col, mut row) = start_point;
    loop {
        if row + 1 == lowest + 2 {
            map.insert((col,row));
            return true;
        } else if !map.contains(&(col,row+1)) {
            row += 1;
        } else if !map.contains(&(col-1,row+1)) {
            row += 1;
            col -= 1;
        } else if !map.contains(&(col+1,row+1)) {
            row += 1;
            col += 1;
        } else {
            map.insert((col,row));
            return true;
        }
    }
}
