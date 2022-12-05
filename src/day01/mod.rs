use std::fs;

#[test]
fn day01() {
    let input = fs::read_to_string("src/day01/input.txt").unwrap();
    let elfs = input.split("\n\n").map(|elf| {
        elf.trim_end()
            .split('\n')
            .map(|s| s.parse::<u32>().unwrap())
    });

    let max_tot = elfs.clone().map(|e| e.sum::<u32>()).max().unwrap();

    println!("{:#?}", max_tot);

    let mut elfs_tots = elfs.clone().map(|e| e.sum::<u32>()).collect::<Vec<_>>();
    elfs_tots.sort();
    let max_tot_3 = elfs_tots.iter().rev().take(3).sum::<u32>();

    println!("{:#?}", max_tot_3);
}
