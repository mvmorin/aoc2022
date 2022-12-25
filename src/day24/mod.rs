#[test]
fn day24() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    let map = parse_input(input);
    let (start, goal) = find_start_goal(&map);

    // part 1
    let min_time = min_time_through(&map, start, goal, 0);
    println!("{}", min_time);

    // part 2
    let min_time_back = min_time_through(&map, goal, start, min_time);
    let min_time_back_again = min_time_through(&map, start, goal, min_time_back);
    println!("{}", min_time_back_again);
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Tile {
    Free,
    Wall,
    Up,
    Right,
    Down,
    Left,
}

fn parse_input(s: &str) -> Matrix<Tile> {
    let n_rows = s.lines().count();
    let n_cols = s.lines().next().unwrap().chars().count();
    let mut map = Matrix::new(Tile::Free, n_rows, n_cols);

    for (row,line) in s.lines().enumerate() {
        for (col,c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Free,
                '#' => Tile::Wall,
                '^' => Tile::Up,
                '>' => Tile::Right,
                'v' => Tile::Down,
                '<' => Tile::Left,
                c => panic!("error in input, found {}", c),
            };
            map.set(row,col,tile);
        }
    }

    map
}

fn find_start_goal(map: &Matrix<Tile>) -> ((usize,usize), (usize,usize)) {
    let mut start_col = 0;
    let mut end_col = 0;

    for col in 0..map.n_cols {
        if map.get(0,col) == Tile::Free { start_col = col; }
        if map.get(map.n_rows-1,col) == Tile::Free { end_col = col; }
    }

    ( (0,start_col), (map.n_rows-1,end_col) )
}

fn index_add_with_wrap(idx: usize, to_add: isize, low: usize, up: usize) -> usize {
    if idx < low || idx >= up { panic!("invalid bound for wrapping add"); }

    let period = (up - low) as isize;
    let shifted = idx as isize - low as isize;
    ((( (shifted + to_add) % period) + period) % period) as usize + low
}

fn position_free_at_time(map: &Matrix<Tile>, row: usize, col: usize, min: usize) -> bool {
    if row == 0 || col == 0 || row == map.n_rows-1 || col == map.n_cols-1 {
        return map.get(row,col) == Tile::Free
    }

    let min = min as isize;

    let left_shifted_col = index_add_with_wrap(col, -min, 1,map.n_cols-1);
    if map.get(row,left_shifted_col) == Tile::Right { return false; }

    let right_shifted_col = index_add_with_wrap(col, min, 1,map.n_cols-1);
    if map.get(row,right_shifted_col) == Tile::Left { return false; }

    let up_shifted_row = index_add_with_wrap(row, -min, 1,map.n_rows-1);
    if map.get(up_shifted_row,col) == Tile::Down { return false; }

    let down_shifted_row = index_add_with_wrap(row, min, 1,map.n_rows-1);
    if map.get(down_shifted_row,col) == Tile::Up { return false; }

    true
}

fn min_time_through(map: &Matrix<Tile>, start: (usize,usize), goal: (usize,usize), start_time: usize) -> usize {
    let before_goal_row = if goal.0 == 0 {
        goal.0 + 1
    } else {
        goal.0 - 1
    };
    let n_rows = map.n_rows;
    let n_cols = map.n_cols;

    let mut time = start_time;
    let mut reachable = Matrix::new(false, n_rows, n_cols);
    reachable.set(start.0, start.1, true);

    while !reachable.get(goal.0, goal.1) {
        let mut next_reachable = Matrix::new(false, n_rows, n_cols);
        time += 1;

        next_reachable.set(start.0, start.1, true); // start always reachable

        if reachable.get(before_goal_row, goal.1) {
            next_reachable.set(goal.0,goal.1,true); // goal reachable if the one before is
        }

        for row in 1..n_rows-1 {
            for col in 1..n_cols-1 {
                let previous_reachable =
                    reachable.get(row, col)
                    || reachable.get(row+1, col)
                    || reachable.get(row-1, col)
                    || reachable.get(row, col+1)
                    || reachable.get(row, col-1);

                if previous_reachable && position_free_at_time(&map, row, col, time) {
                    next_reachable.set(row,col,true);
                }
            }
        }

        reachable = next_reachable;
    }

    return time;
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
    fn print_matrix(&self) {
        for row in 0..self.n_rows {
            for col in 0..self.n_cols {
                let idx = self.data_index(row,col);
                print!("{:?} ", self.data[idx]);
            }
            println!();
        }
    }
}
