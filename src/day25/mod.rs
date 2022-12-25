#[test]
fn day25() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    let snafus = parse_snafus(input);
    let sum = snafus.iter().map(|s| snafu_to_i64(s)).sum::<i64>();
    let sum_snafu = i64_to_snafu(sum);
    let sum_string = snafu_to_string(&sum_snafu);
    println!("{}", sum_string);
}

fn parse_snafus(s: &str) -> Vec<Vec<i64>> {
    let mut snafus = Vec::new();

    for line in s.lines() {
        let mut snafu = Vec::new();
        for c in line.chars() {
            let val = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                c => panic!("invalid input: {}", c),
            };
            snafu.insert(0,val);
        }
        snafus.push(snafu);
    }
    snafus
}

fn snafu_to_i64(snafu: &Vec<i64>) -> i64 {
    let mut res = 0;
    for (exp, digit) in snafu.iter().enumerate() {
        res += 5i64.pow(exp as u32)*digit;
    }
    res
}

fn i64_to_snafu(mut num: i64) -> Vec<i64> {
    let mut base5 = Vec::new();
    if num == 0 { base5.push(0); }

    while num > 0 {
        let rem = num % 5;
        num = num / 5;
        base5.push(rem);
    }

    let mut carry = 0;
    let mut snafu = Vec::new();
    for digit in base5.iter() {
        let tot = digit + carry;
        if tot <= 2 {
            snafu.push(tot);
            carry = 0;
        } else {
            snafu.push(tot - 5);
            carry = 1;
        }
    }
    if carry > 0 {
        snafu.push(1);
    }

    snafu
}

fn snafu_to_string(snafu: &Vec<i64>) -> String {
    let mut string = String::new();
    for digit in snafu.iter().rev() {
        let c = match digit {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            d => panic!("invalid snafu digit: {}", d),
        };
        string.push(c);
    }
    string
}
