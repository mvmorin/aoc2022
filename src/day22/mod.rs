#[test]
fn day22() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    let mut parts = input.split("\n\n");
    let map = Map::from(parts.next().unwrap());
    let actions = Action::from(parts.next().unwrap());
    let start = (1, map.map[1].iter().position(|&t| t == Tile::Open).unwrap());
    // println!("{:?}", map.map);
    // println!("{:?}", actions);
    // println!("{:?}", start);

    // part 1
    let (mut row, mut col, mut dir) = (start.0, start.1, Direction::Right);
    for action in actions.iter() {
        // println!("{},{},{:?} - {:?}", row,col,dir,action);
        (row,col,dir) = map.take_action(action, row,col,dir);
        // println!("{},{},{:?}\n", row,col,dir);
    }

    println!("{},{},{:?}", row,col,dir);
    let dir_val = match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };
    println!("{}", 1000*row + col*4 + dir_val);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Solid,
    Empty,
}

struct Map {
    map: Vec<Vec<Tile>>
}

#[derive(Debug,Clone,Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Action {
    Walk(usize),
    Turn(Turn),
}

#[derive(Debug,Clone,Copy)]
enum Turn {
    Left,
    Right,
}

impl Map {
    fn from(s: &str) -> Self {
        let mut map = Vec::new();
        map.push(Vec::new());

        let mut longest_row = 0;
        for line in s.lines() {
            let mut row = vec![Tile::Empty];
            for c in line.chars() {
                match c {
                    ' ' => { row.push(Tile::Empty); },
                    '.' => { row.push(Tile::Open); },
                    '#' => { row.push(Tile::Solid); },
                    _ => panic!("this should never happen"),
                }
            }
            row.push(Tile::Empty);
            longest_row = longest_row.max(row.len());
            map.push(row);
        }
        map.push(Vec::new());

        // pad every row to
        for row in map.iter_mut() {
            for _ in 0..(longest_row - row.len()) {
                row.push(Tile::Empty);
            }
        }

        Map{map}
    }

    fn get_next(&self, row: usize, col: usize, dir: Direction) -> (usize,usize) {
        let (r_new, c_new) = match dir {
            Direction::Up => (row-1,col),
            Direction::Right => (row,col+1),
            Direction::Down => (row+1,col),
            Direction::Left => (row,col-1),
        };

        match &self.map[r_new][c_new] {
            Tile::Open => { return (r_new, c_new) },
            Tile::Solid => { return (row, col) },
            Tile::Empty => {
                let (mut point, search_dir) = match dir {
                    Direction::Up => ( (self.map.len()-1,col) , (-1,0) ),
                    Direction::Right => ( (row,0) , (0,1) ),
                    Direction::Down => ( (0,col) , (1,0) ),
                    Direction::Left => ( (row,self.map[row].len()-1) , (0,-1) ),
                };

                while let Tile::Empty = &self.map[point.0][point.1] {
                    let p0 = ((point.0 as isize) + search_dir.0) as usize;
                    let p1 = ((point.1 as isize) + search_dir.1) as usize;
                    point = (p0,p1);
                }

                match &self.map[point.0][point.1] {
                    Tile::Open => { return point; }
                    Tile::Solid => { return (row,col); },
                    Tile::Empty => panic!("this should never happen"),
                }
            }
        }
    }

    fn take_action(&self, action: &Action, mut row: usize, mut col: usize, dir: Direction) -> (usize,usize,Direction){
        match action {
            Action::Turn(t) => { return (row,col,turn(dir,*t)); },
            Action::Walk(steps) => {
                for _ in 0..*steps {
                    (row,col) = self.get_next(row,col,dir);
                }
                return (row,col,dir);
            }
        }
    }
}

impl Action {
    fn from(s: &str) -> Vec<Self> {
        let mut chs = s.chars().peekable();
        let mut actions = Vec::new();
        while let Some(c) = chs.next() {
            match c {
                'L' => { actions.push(Action::Turn(Turn::Left)); },
                'R' => { actions.push(Action::Turn(Turn::Right)); },
                '\n' => {},
                _ => {
                    let mut num_str = String::from(c);

                    while let Some(&c) = chs.peek() {
                        if c == 'L' || c == 'R' || c == '\n' {break;}
                        num_str.push(c);
                        chs.next();
                    }

                    let num = num_str.parse::<usize>().unwrap();
                    actions.push(Action::Walk(num))
                }
            }
        }

        return actions;
    }
}




fn turn(current: Direction, turn: Turn) -> Direction {
    let dir = match current {
        Direction::Up => 0,
        Direction::Right => 1,
        Direction::Down => 2,
        Direction::Left => 3,
    };

    let turn = match turn {
        Turn::Left => -1,
        Turn::Right => 1,
    };

    let new_dir = (((dir + turn) % 4) + 4) % 4;

    match new_dir {
        0 => Direction::Up,
        1 => Direction::Right,
        2 => Direction::Down,
        3 => Direction::Left,
        _ => panic!("this should never happen"),
    }
}
