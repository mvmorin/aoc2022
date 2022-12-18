#[test]
fn day03() {
    let input = include_str!("input.txt");
    let tot_bp_priority = input
        .trim_end()
        .split('\n')
        .map(|backpack| get_backpack_priority(backpack))
        .sum::<u32>();

    println!("{}", tot_bp_priority);

    let backpacks = input.trim_end().split('\n').collect::<Vec<_>>();

    let mut tot_badge_priority = 0;
    for i in (0..backpacks.len()).step_by(3) {
        tot_badge_priority += get_badge_priority(&backpacks[i..i+3]);
    }

    println!("{}", tot_badge_priority);
}

fn get_badge_priority(bps: &[&str]) -> u32 {
    for c in bps[0].chars() {
        if bps[1].contains(c) && bps[2].contains(c) {
            return get_priority(c);
        }
    }

    panic!("input error");
}

fn get_backpack_priority(bp: &str) -> u32 {
    // println!("{}", bp);

    let n = bp.len() / 2;
    let comp1 = &bp[0..n];
    let comp2 = &bp[n..];

    for c in comp1.chars() {
        if comp2.contains(c) {
            return get_priority(c);
        }
    }

    panic!("input error");
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        return (c as u32) - 97 + 1;
    } else {
        return (c as u32) - 65 + 27;
    }
}
