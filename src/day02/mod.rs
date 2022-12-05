use std::fs;

#[test]
fn day02() {
    let input = fs::read_to_string("src/day02/input.txt").unwrap();
    let score = input
        .trim_end()
        .split('\n')
        .map(|round| {
            calc_round_score_01(round)
        })
        .sum::<u32>();

    println!("{}", score);

    let score = input
        .trim_end()
        .split('\n')
        .map(|round| {
            calc_round_score_02(round)
        })
        .sum::<u32>();

    println!("{}", score);
}

fn calc_round_score_01(r: &str) -> u32 {
    let mut r = r.chars();
    let oppo = r.next().unwrap();
    let me = r.last().unwrap();

    let outcome_score = match (oppo, me) {
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('C', 'Y') => 0,
        _ => panic!("input error"),
    };

    let choice_score = match me {
        'X' =>  1,
        'Y' =>  2,
        'Z' =>  3,
        _ => panic!("input error"),
    };

    // println!("{}, {}", outcome_score, choice_score);

    outcome_score + choice_score
}

fn calc_round_score_02(r: &str) -> u32 {
    let mut r = r.chars();
    let oppo = r.next().unwrap();
    let outcome = r.last().unwrap();

    let choice_score = match (oppo, outcome) {
        ('A', 'X') => 3,
        ('B', 'X') => 1,
        ('C', 'X') => 2,
        ('A', 'Y') => 1,
        ('B', 'Y') => 2,
        ('C', 'Y') => 3,
        ('A', 'Z') => 2,
        ('B', 'Z') => 3,
        ('C', 'Z') => 1,
        _ => panic!("input error"),
    };

    let outcome_score = match outcome {
        'X' =>  0,
        'Y' =>  3,
        'Z' =>  6,
        _ => panic!("input error"),
    };

    outcome_score + choice_score
}
