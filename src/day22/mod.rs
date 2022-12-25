use std::collections::HashMap;

#[test]
fn day22() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    let mut input_parts = input.split("\n\n");

    let (map_segments, start_seg, start_pos) = parse_map_segments(input_parts.next().unwrap());
    let moves = parse_moves(input_parts.next().unwrap());
    let segment_connections = parse_segment_connections(input_parts.next().unwrap());

    // part 1
    let mut pos = Position {
        segment: start_seg.clone(),
        position: start_pos.clone(),
        direction: Direction::Right,
    };

    for m in moves.iter() {
        pos.apply_move_flat(m, &map_segments);
    }

    println!("{}", pos.to_password(&map_segments));

    // part 2
    let mut pos = Position {
        segment: start_seg.clone(),
        position: start_pos.clone(),
        direction: Direction::Right,
    };

    for m in moves.iter() {
        pos.apply_move_cube(m, &map_segments, &segment_connections);
    }

    println!("{}", pos.to_password(&map_segments));
}

struct Segment {
    free: Matrix<bool>,
    local_to_global_offset: (usize,usize),
    segment_side: usize,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Walk(usize),
}

#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Position {
    segment: (usize,usize),
    position: (usize,usize),
    direction: Direction,
}

impl Position {
    fn apply_move_flat(&mut self, m: &Move, segments: &HashMap<(usize,usize),Segment>) {
        match m {
            Move::Left => match self.direction {
                Direction::Up =>    { self.direction = Direction::Left; },
                Direction::Right => { self.direction = Direction::Up; },
                Direction::Down =>  { self.direction = Direction::Right; },
                Direction::Left =>  { self.direction = Direction::Down; },
            },
            Move::Right => match self.direction {
                Direction::Up =>    { self.direction = Direction::Right; },
                Direction::Right => { self.direction = Direction::Down; },
                Direction::Down =>  { self.direction = Direction::Left; },
                Direction::Left =>  { self.direction = Direction::Up; },
            },
            Move::Walk(steps) => {
                for _ in 0..*steps {
                    self.step_flat(segments);
                }
            }
        }
    }

    fn apply_move_cube(&mut self, m: &Move, segments: &HashMap<(usize,usize),Segment>, segment_connections: &HashMap<(usize,usize,Direction),(usize,usize,Direction)>) {
        match m {
            Move::Left => match self.direction {
                Direction::Up =>    { self.direction = Direction::Left; },
                Direction::Right => { self.direction = Direction::Up; },
                Direction::Down =>  { self.direction = Direction::Right; },
                Direction::Left =>  { self.direction = Direction::Down; },
            },
            Move::Right => match self.direction {
                Direction::Up =>    { self.direction = Direction::Right; },
                Direction::Right => { self.direction = Direction::Down; },
                Direction::Down =>  { self.direction = Direction::Left; },
                Direction::Left =>  { self.direction = Direction::Up; },
            },
            Move::Walk(steps) => {
                for _ in 0..*steps {
                    self.step_cube(segments, segment_connections);
                }
            }
        }
    }


    fn step_flat(&mut self, segments: &HashMap<(usize,usize),Segment>) {
        let (dr, dc) = match self.direction {
            Direction::Up =>    (-1, 0),
            Direction::Down =>  (1, 0),
            Direction::Right => (0, 1),
            Direction::Left =>  (0, -1),
        };

        let mut new_segment = segments.get(&self.segment).unwrap();
        let mut new_seg_row = self.segment.0;
        let mut new_seg_col = self.segment.1;
        let (new_row, wrapped_row) = wrapping_add(self.position.0, dr, new_segment.segment_side);
        let (new_col, wrapped_col) = wrapping_add(self.position.1, dc, new_segment.segment_side);

        // if wrapped position, find next segment
        if wrapped_row || wrapped_col {
            loop {
                (new_seg_row, _) = wrapping_add(new_seg_row, dr, 4);
                (new_seg_col, _) = wrapping_add(new_seg_col, dc, 4);
                if let Some(seg) = segments.get(&(new_seg_row,new_seg_col)) {
                    new_segment = seg;
                    break;
                }
            }
        }

        if new_segment.free.get(new_row, new_col) {
            self.position = (new_row, new_col);
            self.segment = (new_seg_row, new_seg_col);
        }
    }

    fn step_cube(&mut self, segments: &HashMap<(usize,usize), Segment>, segment_connections: &HashMap<(usize,usize,Direction),(usize,usize,Direction)>) {
        let (dr, dc) = match self.direction {
            Direction::Up =>    (-1, 0),
            Direction::Down =>  (1, 0),
            Direction::Right => (0, 1),
            Direction::Left =>  (0, -1),
        };

        let mut new_segment = segments.get(&self.segment).unwrap();
        let segment_side = new_segment.segment_side;
        let mut new_seg_row = self.segment.0;
        let mut new_seg_col = self.segment.1;
        let (mut new_row, wrapped_row) = wrapping_add(self.position.0, dr, segment_side);
        let (mut new_col, wrapped_col) = wrapping_add(self.position.1, dc, segment_side);
        let mut new_direction = self.direction;

        if wrapped_row || wrapped_col { // we moved out from the segment, find the neighboring
            let distance_from_left_corner = match self.direction {
                Direction::Up => self.position.1,
                Direction::Right => self.position.0,
                Direction::Down => segment_side - 1 - self.position.1,
                Direction::Left => segment_side - 1 - self.position.0,
            };

            (new_seg_row, new_seg_col, new_direction) =
                *segment_connections.get(&(self.segment.0, self.segment.1, self.direction)).unwrap();

            new_segment = segments.get(&(new_seg_row,new_seg_col)).unwrap();

            (new_row, new_col) = match new_direction {
                Direction::Up => (segment_side - 1, distance_from_left_corner),
                Direction::Right => (distance_from_left_corner, 0),
                Direction::Down => (0, segment_side - 1 - distance_from_left_corner),
                Direction::Left => (segment_side - 1 - distance_from_left_corner, segment_side - 1),
            };
        }

        if new_segment.free.get(new_row, new_col) {
            self.position = (new_row, new_col);
            self.segment = (new_seg_row, new_seg_col);
            self.direction = new_direction;
        }
    }

    fn to_global_coord(&self, segments: &HashMap<(usize,usize),Segment>) -> (usize,usize) {
        let seg = segments.get(&self.segment).unwrap();
        let global_row = self.position.0 + seg.local_to_global_offset.0 + 1;
        let global_col = self.position.1 + seg.local_to_global_offset.1 + 1;
        (global_row, global_col)
    }

    fn to_password(&self, segments: &HashMap<(usize,usize), Segment>) -> usize {
        let (row, col) = self.to_global_coord(segments);
        let dir_val = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };
        // println!("{},{},{:?}", row, col, self.direction);
        1000*row + col*4 + dir_val
    }
}


fn wrapping_add(a: usize, da: isize, wrap: usize) -> (usize,bool) {
    let new_a = a as isize + da;
    let wrap = wrap as isize;
    let res = (((new_a % wrap) + wrap) % wrap) as usize;
    let wrapped = new_a < 0 || new_a >= wrap;
    (res,wrapped)
}

fn parse_map_segments(s: &str) -> (HashMap<(usize,usize),Segment>, (usize,usize), (usize,usize)) {
    let n_rows = s.lines().count();
    let n_cols = s.lines().map(|l| l.len()).max().unwrap();

    let map_ratio = if n_rows > n_cols {
        (4,3)
    } else {
        (3,4)
    };
    assert!( (n_rows / map_ratio.0) == (n_cols / map_ratio.1) );
    let segment_side = n_rows / map_ratio.0;

    // Transform map to easier indexed format (matrix)
    let mut raw_map = Matrix::new(false, n_rows, n_cols);
    for (row,line) in s.lines().enumerate() {
        for (col,c) in line.chars().enumerate() {
            raw_map.set(row,col, c == '.');
        }
    }
    // raw_map.print_matrix(|e| if e {"."} else {"#"}.to_string() );

    // for each segment...
    let mut segments = HashMap::new();
    for seg_row in 0..map_ratio.0 {
        for seg_col in 0..map_ratio.1 {
            let row_offset = seg_row * segment_side;
            let col_offset = seg_col * segment_side;
            let mut segment_map = Matrix::new(false, segment_side, segment_side);

            // ...copy over to segment map
            let mut contains_free_space = false;
            for row in 0..segment_side {
                for col in 0..segment_side {
                    let free_tile = raw_map.get(row + row_offset, col + col_offset);
                    contains_free_space = contains_free_space || free_tile;
                    segment_map.set(row, col, free_tile);
                }
            }

            // if non-trivial segment, store it
            if contains_free_space {
                segments.insert((seg_row,seg_col), Segment {
                    free: segment_map,
                    local_to_global_offset: (row_offset, col_offset),
                    segment_side,
                });
            }

        }
    }

    // find start position and return
    for seg_col in 0..map_ratio.1 {
        if let Some(seg) = segments.get(&(0,seg_col)) {
            for col in 0..segment_side {
                if seg.free.get(0,col) {
                    return (segments, (0,seg_col), (0,col));
                }
            }
        }
    }
    panic!("didn't find a starting position");
}

fn parse_moves(s: &str) -> Vec<Move> {
    let mut chs = s.chars().peekable();
    let mut moves = Vec::new();
    while let Some(c) = chs.next() {
        match c {
            'L' => { moves.push(Move::Left); },
            'R' => { moves.push(Move::Right); },
            '\n' => {},
            _ => {
                let mut num_str = String::from(c);

                while let Some(&c) = chs.peek() {
                    if c == 'L' || c == 'R' || c == '\n' {break;}
                    num_str.push(c);
                    chs.next();
                }

                let num = num_str.parse::<usize>().unwrap();
                moves.push(Move::Walk(num))
            }
        }
    }

    return moves;
}

fn parse_segment_connections(s: &str) -> HashMap<(usize,usize,Direction), (usize,usize,Direction)>{
    let mut connections = HashMap::new();
    for line in s.lines() {
        let mut parts = line.split([' ',',']);

        let row = parts.next().unwrap().parse::<usize>().unwrap();
        let col = parts.next().unwrap().parse::<usize>().unwrap();

        let dist_row = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_col = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_dir = str_to_direction(parts.next().unwrap());
        connections.insert((row,col,Direction::Up), (dist_row,dist_col,dist_dir));

        let dist_row = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_col = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_dir = str_to_direction(parts.next().unwrap());
        connections.insert((row,col,Direction::Right), (dist_row,dist_col,dist_dir));

        let dist_row = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_col = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_dir = str_to_direction(parts.next().unwrap());
        connections.insert((row,col,Direction::Down), (dist_row,dist_col,dist_dir));

        let dist_row = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_col = parts.next().unwrap().parse::<usize>().unwrap();
        let dist_dir = str_to_direction(parts.next().unwrap());
        connections.insert((row,col,Direction::Left), (dist_row,dist_col,dist_dir));
    }
    connections
}

fn str_to_direction(s: &str) -> Direction {
    match s {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        s => panic!("invalid input: {:?}", s),
    }
}




struct Matrix<T> {
    data: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}

impl<T> Matrix<T>
where T: Copy + std::fmt::Debug
{
    fn new(init: T, n_rows: usize, n_cols: usize) -> Self {
        Matrix {
            data: vec![init; n_rows*n_cols],
            n_rows,
            n_cols,
        }
    }

    fn data_index(&self, row: usize, col: usize) -> usize {
        col + row*self.n_cols
    }

    fn get(&self, row: usize, col: usize) -> T {
        let idx = self.data_index(row,col);
        self.data[idx]
    }

    fn set(&mut self, row: usize, col: usize, el: T) {
        let idx = self.data_index(row,col);
        self.data[idx] = el;
    }

    #[allow(dead_code)]
    fn print_matrix<F>(&self, f: F) where F: Fn(T) -> String {
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                let idx = self.data_index(row,col);
                print!("{}", f(self.data[idx]) );
            }
            println!();
        }
    }
}

