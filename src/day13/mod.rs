use std::iter::Peekable;
use std::cmp::Ordering;

#[test]
fn day13() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");

    // part 1
    let pairs = input.split("\n\n").map(|s| parse_pair(s) );
    let count = pairs.enumerate().map(|(i,p)| {
        if is_ordered_bool(&p.0, &p.1) {
            i+1
        } else {
            0
        }
    }).sum::<usize>();
    println!("{}", count);

    // part 2
    let packets = input
        .split("\n\n")
        .flat_map(|s| s.lines())
        .map(|line| parse_entry(&mut line.chars().peekable()).unwrap() );

    let dp1 = List(vec![ List(vec![Int(2)]) ]);
    let dp2 = List(vec![ List(vec![Int(6)]) ]);
    let dp1_pos = packets.clone().filter(|p| is_ordered_bool(p, &dp1)).count() + 1;
    let dp2_pos = packets.clone().filter(|p| is_ordered_bool(p, &dp2)).count() + 2;
    println!("{}*{} = {}", dp1_pos, dp2_pos, dp1_pos*dp2_pos);


}

#[derive(Debug)]
enum Entry {
    Int(u32),
    List(Vec<Entry>),
}
use Entry::*;

fn is_ordered_bool(first: &Entry, second: &Entry) -> bool {
    match is_ordered(first, second) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => panic!("{:?}, {:?}", first, second),
    }
}

fn is_ordered(first: &Entry, second: &Entry) -> Ordering {
    match (first, second) {
        (Int(f), Int(s)) => {
            return f.cmp(s)
        },
        (Int(f), List(_)) => {
            return is_ordered(&List(vec![Int(*f)]),&second);
        },
        (List(_), Int(s)) => {
            return is_ordered(&first, &List(vec![Int(*s)]));
        },
        (List(f), List(s)) => {
            let (f_len, s_len) = (f.len(), s.len());
            let (mut f_it, mut s_it) = (f.iter(), s.iter());

            while let (Some(fe), Some(se)) = (f_it.next(), s_it.next()) {
                match is_ordered(fe,se) {
                    Ordering::Equal => {},
                    ord => return ord,
                }
            }
            return f_len.cmp(&s_len);
        },
    };
}

fn parse_pair(s: &str) -> (Entry,Entry) {
    let mut entries = s.lines().map(|l| {
        parse_entry(&mut l.chars().peekable()).unwrap()
    });
    (entries.next().unwrap(), entries.next().unwrap())
}

fn parse_entry<I>(chars: &mut Peekable<I>) -> Option<Entry>
where
    I: Iterator<Item = char>,
{
    let &next = chars.peek()?;

    if next.is_ascii_digit() {
        let mut int_s = String::new();
        while let Some(&c) = chars.peek() {
            if !c.is_ascii_digit() { break; }
            int_s.push(c);
            chars.next();
        }
        return Some(Int(int_s.parse::<u32>().unwrap()));

    } else if next == '[' {
        chars.next();

        let mut packet: Vec<Entry> = Vec::new();
        while let Some(e) = parse_entry(chars) {
            packet.push(e);
        }
        return Some(List(packet));

    } else if next == ']' {
        chars.next();
        return None;

    } else if next == ',' {
        chars.next();
        return parse_entry(chars);
    }

    None
}
