
type Map = Vec<Vec<bool>>;

#[test]
fn day14() {
    let input = include_str!("input.txt");
    // let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    // println!("{}", input);

    // part 1
    let (mut map, lowest) = parse_input(input);
    let count = count_sand_no_floor(&mut map, lowest);
    println!("{:?}", count);

    // part 2
    let (mut map, lowest) = parse_input(input);
    let count = count_sand_floor(&mut map, lowest);
    println!("{:?}", count);
}

fn parse_input(input: &str) -> (Map, usize) {
    fn parse_tuple(s: &str) -> (usize,usize) {
        let mut coords = s.split(',');
        let col = coords.next().unwrap().parse::<usize>().unwrap();
        let row = coords.next().unwrap().parse::<usize>().unwrap();
        (col,row)
    }

    let mut map = Vec::new();
    for _ in 0..=1000 { // size is based on triangle from starting point and assuming non-negative coordinates
        map.push(vec![false; 501]);
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

#[derive(Clone,Copy)]
enum Direction {
    Down,
    Left,
    Right,
    Stop,
}
use Direction::*;

impl Direction {
    fn next(&self) -> Self {
        match self {
            Down => Left,
            Left => Right,
            Right => Stop,
            Stop => panic!("trying to continue from stop node"),
        }
    }

    fn next_coord(&self,col:usize,row:usize) -> (usize,usize) {
        match self {
            Down => (col,row+1),
            Left => (col-1,row+1),
            Right => (col+1,row+1),
            Stop => panic!("trying to continue from stop node"),
        }
    }
}

fn count_sand_no_floor(map: &mut Map, lowest: usize) -> usize {
    let mut count = 0;
    let mut path = Vec::new();
    path.push(((500,0),Down));

    while let Some(&((col, row), direction)) = path.last() {

        if row >= lowest {
            return count;
        }

        match direction {
            Stop => {
                path.pop();
                map[col][row] = true;
                count += 1;
            },
            dir => {
                let (c_new, r_new) = dir.next_coord(col,row);
                if !map[c_new][r_new] {
                    path.push( ((c_new,r_new),Down) );
                } else {
                    let last = path.last_mut().unwrap();
                    *last = ((col,row), dir.next());
                }
            },
        }
    }

    return count
}

fn count_sand_floor(map: &mut Map, lowest: usize) -> usize {
    let mut count = 0;
    let mut path = Vec::new();
    path.push(((500,0),Down));

    while let Some(&((col, row), direction)) = path.last() {

        match direction {
            Stop => {
                path.pop();
                map[col][row] = true;
                count += 1;
            },
            dir => {
                let (c_new, r_new) = dir.next_coord(col,row);
                if !map[c_new][r_new] && r_new < lowest+2 {
                    path.push( ((c_new,r_new),Down) );
                } else {
                    let last = path.last_mut().unwrap();
                    *last = ((col,row), dir.next());
                }
            },
        }
    }

    return count
}
