#[test]
fn day17() {
    let input = include_str!("input.txt").trim();
    // let input = include_str!("input_nils.txt").trim();
    // let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let moves = input.chars().map(|c| {
        if c == '<' {
            -1
        } else if c == '>' {
            1
        } else {
            panic!("invalid input");
        }
    }).collect::<Vec<i64>>();

    // part 1
    let height = simulate(2022, &moves);
    println!("{}", height);

    // part 2
    let height = simulate_periodic(1_000_000_000_000, &moves);
    println!("{}", height);
}

fn simulate(n_rocks: usize, moves: &Vec<i64>) -> i64 {
    let mut chamber = Chamber::new(&moves,&SHAPES);

    for _ in 0..n_rocks {
        chamber.simulate_next_rock();
    }
    chamber.height
}

fn simulate_periodic(n_rocks: i64, moves: &Vec<i64>) -> i64 {
    let mut chamber = Chamber::new(moves,&SHAPES);

    let mut shape_move_seen_after = vec![vec![-1i64; moves.len()];5];

    for simulated_rocks in 0..n_rocks {
        let sh_idx = chamber.shapes.idx;
        let mv_idx = chamber.moves.idx;

        if shape_move_seen_after[sh_idx][mv_idx] < 0 {
            shape_move_seen_after[sh_idx][mv_idx] = simulated_rocks;
        } else {
            let mut ref_chamber = Chamber::new(moves,&SHAPES);
            let n_rocks_transient = shape_move_seen_after[sh_idx][mv_idx];
            for _ in 0..n_rocks_transient {
                ref_chamber.simulate_next_rock();
            }
            let height_period = chamber.height - ref_chamber.height;
            let n_rocks_period = simulated_rocks - n_rocks_transient;

            let mut equal = true;
            for _ in 0..n_rocks_period {
                let (row,col) = chamber.simulate_next_rock();
                let (row_ref,col_ref) = ref_chamber.simulate_next_rock();

                if !(row == row_ref + height_period && col == col_ref) {
                    equal = false;
                }
            }

            chamber = ref_chamber;

            if equal {
                let remainder = (n_rocks - n_rocks_transient) % n_rocks_period;
                let n_periods = (n_rocks - n_rocks_transient) / n_rocks_period;
                for _ in 0..remainder {
                    chamber.simulate_next_rock();
                }
                return chamber.height + height_period*(n_periods - 1)
            }
        }

        chamber.simulate_next_rock();
    }
    return chamber.height;
}



#[derive(Debug,Clone,Copy)]
struct Shape {
    r3: u8,
    r2: u8,
    r1: u8,
    r0: u8,
    width: u8,
    height: u8,
}

const SHAPES: [Shape;5] = [
    Shape{
        r3: 0b0000,
        r2: 0b0000,
        r1: 0b0000,
        r0: 0b1111,
        width: 4,
        height: 1,
    },
    Shape{
        r3: 0b0000,
        r2: 0b0010,
        r1: 0b0111,
        r0: 0b0010,
        width: 3,
        height: 3,
    },
    Shape{
        r3: 0b0000,
        r2: 0b0100,
        r1: 0b0100,
        r0: 0b0111,
        width: 3,
        height: 3,
    },
    Shape{
        r3: 0b0001,
        r2: 0b0001,
        r1: 0b0001,
        r0: 0b0001,
        width: 1,
        height: 4,
    },
    Shape{
        r3: 0b0000,
        r2: 0b0000,
        r1: 0b0011,
        r0: 0b0011,
        width: 2,
        height: 2,
    },
];

const ROW_BUF_SIZE: usize = 1024;

#[derive(Debug)]
struct Chamber<'a> {
    chamber: [u8; ROW_BUF_SIZE],
    height: i64,
    moves: CyclicIndexable<'a,i64>,
    shapes: CyclicIndexable<'a,Shape>,
}

impl<'a> Chamber<'a> {
    fn new(moves: &'a[i64], shapes: &'a[Shape]) -> Self {
        Chamber{
            chamber: [0; ROW_BUF_SIZE],
            height: 0,
            moves: CyclicIndexable::new(moves),
            shapes: CyclicIndexable::new(shapes),
        }
    }

    fn shape_free(&self, row: i64, col: i64, shape: Shape) -> bool {
        if row < 0 { return false; }
        if col < 0 || col as u8 + shape.width > 7 { return false; }

        let row = row as usize;
        let r3 = (row + 3) % ROW_BUF_SIZE;
        let r2 = (row + 2) % ROW_BUF_SIZE;
        let r1 = (row + 1) % ROW_BUF_SIZE;
        let r0 = (row + 0) % ROW_BUF_SIZE;

        return (self.chamber[r3] & (shape.r3 << col) == 0)
            && (self.chamber[r2] & (shape.r2 << col) == 0)
            && (self.chamber[r1] & (shape.r1 << col) == 0)
            && (self.chamber[r0] & (shape.r0 << col) == 0)
    }

    fn add_shape(&mut self, row: i64, col: i64, shape: Shape) -> i64{
        let row = row as usize;
        let r3 = (row + 3) % ROW_BUF_SIZE;
        let r2 = (row + 2) % ROW_BUF_SIZE;
        let r1 = (row + 1) % ROW_BUF_SIZE;
        let r0 = (row + 0) % ROW_BUF_SIZE;

        self.chamber[r3] |= shape.r3 << col;
        self.chamber[r2] |= shape.r2 << col;
        self.chamber[r1] |= shape.r1 << col;
        self.chamber[r0] |= shape.r0 << col;
        return row as i64 + shape.height as i64 - 1; // last row something was added to
    }

    fn clear_4_rows(&mut self, row: i64) {
        let row = row as usize;
        let r3 = (row + 3) % ROW_BUF_SIZE;
        let r2 = (row + 2) % ROW_BUF_SIZE;
        let r1 = (row + 1) % ROW_BUF_SIZE;
        let r0 = (row + 0) % ROW_BUF_SIZE;

        self.chamber[r3] = 0;
        self.chamber[r2] = 0;
        self.chamber[r1] = 0;
        self.chamber[r0] = 0;
    }

    fn simulate_next_rock(&mut self) -> (i64,i64) {
        let shape = self.shapes.next().unwrap();
        let mut row = self.height + 3;
        let mut col = 2;

        self.clear_4_rows(row);

        loop {
            let col_shift = self.moves.next().unwrap();

            if self.shape_free(row, col + col_shift, shape) {
                col += col_shift;
            }

            if self.shape_free(row-1, col, shape) {
                row -= 1;
            } else {
                let top_row_added = self.add_shape(row, col, shape);
                self.height = self.height.max(top_row_added + 1);
                break;
            }
        }

        return (row,col);
    }

    #[allow(dead_code)]
    fn print_chamber(&self, start_row: i64, end_row: i64) {
        for row in (start_row..=end_row).rev() {
            for col in 0..7 {
                let r = row as usize % ROW_BUF_SIZE;
                let row_mask = 1 << col;
                let pixel = if self.chamber[r] & row_mask == 0 {
                    "."
                } else {
                    "#"
                };
                print!("{}", pixel);
            }
            println!();
        }
    }

}
















#[derive(Debug)]
struct CyclicIndexable<'a,T> {
    vec: &'a [T],
    idx: usize,
    len: usize,
}

impl<'a,T> CyclicIndexable<'a,T> {
    fn new(vec: &'a [T]) -> Self {
        CyclicIndexable{vec, idx: 0, len: vec.len()}
    }
}

impl<'a,T> Iterator for CyclicIndexable<'a,T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let el = self.vec[self.idx];
        self.idx = (self.idx + 1) % self.len;
        return Some(el);
    }
}

