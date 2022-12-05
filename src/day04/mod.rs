use itertools::Itertools;
use std::fs;
use std::ops::Range;

#[test]
fn day04() {
    let input = fs::read_to_string("src/day04/input.txt").unwrap();

    let n_fully_contained = input
        .trim_end()
        .split('\n')
        .map(|s| to_ranges(s))
        .filter(|(a, b)| range_contains(&a, &b) || range_contains(&b, &a))
        .count();

    println!("{}", n_fully_contained);

    let n_overlaped = input
        .trim_end()
        .split('\n')
        .map(|s| to_ranges(s))
        .filter(|(a, b)| !range_no_overlap(&a,&b) )
        .count();

    println!("{}", n_overlaped);
}

fn to_ranges(pair: &str) -> (Range<u32>, Range<u32>) {
    let mut pair = pair.split(['-', ',']).map(|v| v.parse::<u32>().unwrap());
    let (a_l, a_u, b_l, b_u) = pair.next_tuple().unwrap();
    (a_l..a_u, b_l..b_u)
}

fn range_contains<T: PartialOrd>(a: &Range<T>, b: &Range<T>) -> bool {
    (a.start <= b.start) && (b.end <= a.end)
}

fn range_no_overlap<T: PartialOrd>(a: &Range<T>, b: &Range<T>) -> bool {
    a.start > b.end || a.end < b.start
}
