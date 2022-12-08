#[test]
fn day08() {
    let input = include_str!("input.txt");
    // let input = "30373\n25512\n65332\n33549\n35390";
    let trees = string_to_trees(input);

    let visible = count_visible(&trees);
    println!("{}", visible);

    let max_score = max_scenic_score(&trees);
    println!("{}", max_score);
}

#[derive(Debug)]
struct Tree {
    height: i32,
    max_u: i32,
    max_l: i32,
    max_d: i32,
    max_r: i32,
}

impl Tree {
    fn new(height: i32) -> Self {
        Tree{height, max_u: -1, max_l: -1, max_d: -1, max_r: -1}
    }

    fn visible(&self) -> bool {
        self.height > self.max_u
            || self.height > self.max_l
            || self.height > self.max_d
            || self.height > self.max_r
    }
}

fn string_to_trees(s: &str) -> Vec<Vec<Tree>> {
    let mut trees = s.lines()
        .map(|line| {
            line.chars()
                .map(|c| Tree::new((c as i32) - ('0' as i32)))
                .collect()
        })
        .collect::<Vec<Vec<Tree>>>();

    find_sightlines(&mut trees);
    trees
}

fn find_sightlines(trees: &mut Vec<Vec<Tree>>) {
    let n_row = trees.len();
    let n_col = trees[0].len();

    // loop right and down and look behind
    for row in 0..n_row {
        for col in 0..n_col {
            let max_u = if row == 0 {
                -1
            } else {
                blocking_height(&trees[row-1][col], Direction::Up)
            };

            let max_l = if col == 0 {
                -1
            } else {
                blocking_height(&trees[row][col-1], Direction::Left)
            };

            let t = &mut trees[row][col];
            t.max_u = max_u;
            t.max_l = max_l;
        }
    }

    // loop left and up and look behind
    for row in (0..n_row).rev() {
        for col in (0..n_col).rev() {
            let max_d = if row == n_row-1 {
                -1
            } else {
                blocking_height(&trees[row+1][col], Direction::Down)
            };

            let max_r = if col == n_col-1 {
                -1
            } else {
                blocking_height(&trees[row][col+1], Direction::Right)
            };

            let t = &mut trees[row][col];
            t.max_d = max_d;
            t.max_r = max_r;
        }
    }
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn blocking_height(t: &Tree, dir: Direction) -> i32 {
    let height_beyond = match dir {
        Direction::Up => t.max_u,
        Direction::Left => t.max_l,
        Direction::Down => t.max_d,
        Direction::Right => t.max_r,
    };
    t.height.max(height_beyond)
}

fn scenic_score(row: usize, col: usize, trees: &Vec<Vec<Tree>>) -> u32 {
    let n_row = trees.len();
    let n_col = trees[0].len();
    let height = trees[row][col].height;

    let mut up = 0;
    for r in (0..row).rev() {
        up += 1;
        if height <= trees[r][col].height {
            break;
        }
    }

    let mut down = 0;
    for r in row+1..n_row {
        down += 1;
        if height <= trees[r][col].height {
            break;
        }
    }

    let mut right = 0;
    for c in col+1..n_col {
        right += 1;
        if height <= trees[row][c].height {
            break;
        }
    }

    let mut left = 0;
    for c in (0..col).rev() {
        left += 1;
        if height <= trees[row][c].height {
            break;
        }
    }

    // println!("{},{}: {},{},{},{}", row, col, up, left, down, right);
    up*left*down*right
}

fn count_visible(trees: &Vec<Vec<Tree>>) -> u32 {
    trees.iter().flat_map(|r| r.iter())
        .map(|t| t.visible() as u32)
        .sum::<u32>()
}

fn max_scenic_score(trees: &Vec<Vec<Tree>>) -> u32 {
    let n_row = trees.len();
    let n_col = trees[0].len();

    (0..n_row).map(|row| {
        (0..n_col).map(|col| {
            scenic_score(row, col, &trees)
        }).max().unwrap()
    }).max().unwrap()
}

