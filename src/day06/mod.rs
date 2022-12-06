#[test]
fn day06() {
    let input = include_str!("input.txt").trim_end();

    let pos = scan_start_of_packet(input, 4);
    println!("{}", pos);

    let pos = scan_start_of_packet(input, 14);
    println!("{}", pos);
}

fn scan_start_of_packet(s: &str, window_size: usize) -> usize {
    let mut latest_duplicate = -1;
    for (idx, c) in s.chars().enumerate() {
        latest_duplicate = match &s[..idx].rfind(c) {
            None => latest_duplicate,
            Some(dup_idx) => latest_duplicate.max((*dup_idx) as i32),
        };

        if (latest_duplicate + window_size as i32) <= (idx as i32) {
            return idx + 1; // problem was 1 based indexing
        }
    }

    panic!("input error");
}
