use std::collections::VecDeque;

#[test]
fn day12() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    let (start,end,map) = parse_input(input);
    // println!("{:?}, {:?}, {:?}", start, end, map);

    let shortest = find_shortest_distance(start,end,&map);
    println!("{:?}", shortest);

    let possible_starts = all_lowest_points(&map);
    let shortest_possible = possible_starts.iter().map(|&s| find_shortest_distance(s,end,&map)).min().unwrap();
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

fn find_shortest_distance(start: (usize,usize), end: (usize,usize), map: &Vec<Vec<u32>>) -> usize {
    let n_rows = map.len();
    let n_cols = map[0].len();

    let mut visited: Vec<Vec<bool>> = Vec::new();
    for map_row in map.iter() {
        visited.push(vec![false; map_row.len()]);
    }

    let mut front: VecDeque<((usize,usize),usize)> = VecDeque::new();
    front.push_back(((start),0));


    while let Some(((row,col),dist)) = front.pop_front() {
        if (row,col) == end {
            return dist;
        }

        let mut add_to_front = |r_off:isize,c_off:isize| {
            if (row == 0 && r_off < 0)
                || (col == 0 && c_off < 0)
                || (row == n_rows-1 && r_off > 0)
                || (col == n_cols-1 && c_off > 0) {
                return;
            }

            let r_n = (row as isize + r_off) as usize;
            let c_n = (col as isize + c_off) as usize;

            if (map[r_n][c_n] as i64) - (map[row][col] as i64) <= 1 && !visited[r_n][c_n] {
                visited[r_n][c_n] = true;
                front.push_back(((r_n,c_n),dist+1));
            }
        };

        add_to_front(1,0);
        add_to_front(-1,0);
        add_to_front(0,1);
        add_to_front(0,-1);
    }

    return usize::MAX;
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
