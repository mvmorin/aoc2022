#[test]
fn day23() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    // part 1
    let (mut elves, mut map) = parse_input(&input);
    // map.print_matrix(|b| if b {"#"} else {"."}.to_string());
    for round in 0..10 {
        perform_round(&mut elves, &mut map, round);
    }

    let (min_x,max_x,min_y,max_y) = find_bounds(&elves);
    let area = (max_x - min_x + 1)*(max_y - min_y + 1);
    // println!("{:?}", (min_x,max_x,min_y,max_y) );
    println!("{}", area - elves.len() as i64);

    // part 2
    let (mut elves, mut map) = parse_input(&input);
    let mut round = 0;
    while perform_round(&mut elves, &mut map, round) { round += 1; }
    println!("{}", round+1);
}

const COORD_BOUND: u32 = 130;

fn parse_input(s: &str) -> (Vec<(i64,i64)>, ShiftedIndexMatrix<bool>) {
    let mut map = ShiftedIndexMatrix::centered(COORD_BOUND, false);
    let mut elves = Vec::new();

    for (x,line) in s.lines().enumerate() {
        for (y,c) in line.chars().enumerate() {
            if c == '#' {
                let (row, col) = (x as i64, y as i64);
                elves.push( (row,col) );
                map.set(row,col,true);
            }
        }
    }
    (elves, map)
}

// x down (from north, to south), y right (from west to east)
const N_NE_E_SE_S_SW_W_NW: [(i64,i64); 8] = [(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1)];

const N_NW_NE: [(i64,i64); 3] = [(-1,0),(-1,-1),(-1,1)];
const S_SW_SE: [(i64,i64); 3] = [(1,0),(1,-1),(1,1)];
const W_NW_SW: [(i64,i64); 3] = [(0,-1),(-1,-1),(1,-1)];
const E_NE_SE: [(i64,i64); 3] = [(0,1),(-1,1),(1,1)];

const DIRECTIONS: [&[(i64,i64)]; 4] = [&N_NW_NE, &S_SW_SE, &W_NW_SW, &E_NE_SE];

fn perform_round(elves: &mut Vec<(i64,i64)>, map: &mut ShiftedIndexMatrix<bool>, round_idx: usize) -> bool {
    let mut proposed_moves: ShiftedIndexMatrix<u32> = ShiftedIndexMatrix::centered(COORD_BOUND, 0);
    let mut elves_to_move: Vec<(usize,(i64,i64))> = Vec::with_capacity(elves.len());

    for (elf_idx,elf) in elves.iter().enumerate() {
        let (x,y) = elf;

        let mut has_neigbour = false;
        for (dx,dy) in N_NE_E_SE_S_SW_W_NW.iter() {
            has_neigbour = has_neigbour || map.get(x+dx,y+dy);
        }

        if !has_neigbour { continue; }

        for dir_idx in 0..4 {
            let to_check = DIRECTIONS[ (dir_idx + round_idx) % 4 ];

            let mut has_neigbour = false;
            for (dx,dy) in to_check.iter() {
                has_neigbour = has_neigbour || map.get(x+dx,y+dy);
            }

            if has_neigbour { continue; }

            let move_to = (x + to_check[0].0, y + to_check[0].1);
            let tmp = proposed_moves.get(move_to.0, move_to.1);
            proposed_moves.set(move_to.0, move_to.1, tmp + 1);
            elves_to_move.push((elf_idx,move_to));
            break;
        }
    }

    for &(elf_idx,move_to) in elves_to_move.iter() {
        if proposed_moves.get(move_to.0, move_to.1) == 1 {
            let old_pos = elves[elf_idx];
            map.set(old_pos.0, old_pos.1, false);
            map.set(move_to.0, move_to.1, true);
            elves[elf_idx] = move_to;
        }
    }

    return elves_to_move.len() > 0;
}

fn find_bounds(elves: &Vec<(i64,i64)>) -> (i64,i64,i64,i64) {
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    for &(x,y) in elves.iter() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    (min_x,max_x,min_y,max_y)
}

struct ShiftedIndexMatrix<T> {
    data: Vec<T>,
    row_low: i64,
    row_up: i64,
    col_low: i64,
    col_up: i64,
}

impl<T> ShiftedIndexMatrix<T>
where T: Copy
{
    fn centered(idx_bound: u32, fill: T) -> Self {
        let data = vec![fill; ((2*idx_bound)*(2*idx_bound)) as usize];
        let bound = idx_bound as i64;
        ShiftedIndexMatrix {
            data,
            row_low: -bound,
            row_up: bound,
            col_low: -bound,
            col_up: bound,
        }
    }

    fn data_index(&self, row: i64, col: i64) -> usize {
        if row < self.row_low || row >= self.row_up || col < self.col_low || col >= self.col_up {
            panic!("invalid index");
        }

        let row_offset = row - self.row_low;
        let col_offset = col - self.col_low;
        let n_col = self.col_up - self.col_low;

        (col_offset + row_offset*n_col) as usize
    }

    fn get(&self, row: i64, col: i64) -> T {
        self.data[self.data_index(row,col)]
    }

    fn set(&mut self, row: i64, col: i64, val: T) {
        let idx = self.data_index(row,col);
        self.data[idx] = val;
    }

    #[allow(dead_code)]
    fn print_matrix<F>(&self, f: F) where F: Fn(T) -> String {
        for row in self.row_low..self.row_up {
            for col in self.col_low..self.col_up {
                let idx = self.data_index(row,col);
                print!("{}", f(self.data[idx]) );
            }
            println!();
        }
    }
}

