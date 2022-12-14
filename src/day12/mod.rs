use std::collections::VecDeque;

#[test]
fn day12() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    let (start,end,map) = parse_input(input);
    // println!("{:?}, {:?}, {:?}", start, end, map);

    // part 1
    let distances = distances_to_end(&map, end);
    println!("{}", distances[start.0][start.1]);

    // part 2
    let possible_starts = all_lowest_points(&map);
    let shortest_possible = possible_starts.iter().map(|&s| distances[s.0][s.1]).min().unwrap();
    println!("{:?}", shortest_possible);
}

fn char_to_int(c: char) -> u32 {
    c as u32 - 'a' as u32
}

fn parse_input(input: &str) -> ((usize,usize), (usize,usize), Vec<Vec<u32>>) {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut start = (0,0);
    let mut end = (0,0);

    for (row, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (col, c) in line.chars().enumerate() {
            let height;
            if c == 'S' {
                start = (row,col);
                height = char_to_int('a');
            } else if c == 'E' {
                end = (row,col);
                height = char_to_int('z');
            } else {
                height = char_to_int(c);
            }

            map[row].push(height);
        }
    }

    (start, end, map)
}

fn all_lowest_points(map: &Vec<Vec<u32>>) -> Vec<(usize,usize)> {
    let mut points: Vec<(usize,usize)> = Vec::new();

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, height) in row.iter().enumerate() {
            if *height == 0 {
                points.push((row_idx,col_idx));
            }
        }
    }
    points
}

fn distances_to_end(map: &Vec<Vec<u32>>, end: (usize,usize)) -> Vec<Vec<u32>> {
    let n_rows = map.len();
    let n_cols = map[0].len();

    let mut front = VecDeque::new();
    let mut distances = Vec::new();
    for _ in 0..n_rows {
        distances.push(vec![u32::MAX; n_cols]);
    }

    distances[end.0][end.1] = 0;
    front.push_back(end);

    while let Some((row,col)) = front.pop_front() {
        let current_distance = distances[row][col];
        let current_height = map[row][col];

        let mut add_to_front = |r_off:isize,c_off:isize| {
            if (row == 0 && r_off < 0)
                || (col == 0 && c_off < 0)
                || (row == n_rows-1 && r_off > 0)
                || (col == n_cols-1 && c_off > 0) {
                return;
            }

            let r_n = (row as isize + r_off) as usize;
            let c_n = (col as isize + c_off) as usize;

            if (map[r_n][c_n] as i64) - (current_height as i64) >= -1 && distances[r_n][c_n] > current_distance + 1 {
                distances[r_n][c_n] = current_distance + 1;
                front.push_back((r_n,c_n));
            }
        };

        add_to_front(1,0);
        add_to_front(-1,0);
        add_to_front(0,1);
        add_to_front(0,-1);
    }

    return distances;
}
