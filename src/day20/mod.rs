#[test]
#[ignore]
fn day20() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");


    // part 1
    let mut list = parse(input);
    for idx in 0..list.len() {
        move_elem(idx, &mut list);
    }

    let (x,y,z) = get_coords(&list);
    println!("{:?}", x + y + z);

    // part 2
    let mut list = parse(input);
    for idx in 0..list.len() {
        list[idx].value *= 811_589_153;
    }

    for _ in 0..10 {
        for idx in 0..list.len() {
            move_elem(idx, &mut list);
        }
    }

    let (x,y,z) = get_coords(&list);
    println!("{:?}", x + y + z);
}

#[derive(Debug)]
struct ListElem {
    value: i64,
    idx: usize,
    next: usize,
    prev: usize,
}

fn parse(s: &str) -> Vec<ListElem> {
    let mut list = s
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            let value = l.parse::<i64>().unwrap();
            let next = idx + 1; // for the last node this will be set later
            let prev = if idx > 0 {
                idx - 1
            } else {
                idx // for the first node this will be set later
            };
            ListElem {
                value,
                idx,
                next,
                prev,
            }
        })
    .collect::<Vec<_>>();

    let end = list.len() - 1;
    list[0].prev = end;
    list[end].next = 0;
    return list;
}

fn move_elem(elem: usize, list: &mut Vec<ListElem>) {
    // unlink from start position
    let next = list[elem].next;
    let prev = list[elem].prev;
    list[prev].next = list[elem].next;
    list[next].prev = list[elem].prev;

    // find element to insert behind
    let insert_behind = {
        let direction = list[elem].value.signum();
        let to_skip = list[elem].value.abs() % (list.len() as i64 - 1);
        let mut insert_behind = list[elem].prev;
        if direction >= 0 {
            for _ in 0..to_skip {
                insert_behind = list[insert_behind].next;
            }
        } else {
            for _ in 0..to_skip {
                insert_behind = list[insert_behind].prev;
            }
        }
        insert_behind
    };

    // insert at new position
    let new_next = list[insert_behind].next;
    let new_prev = insert_behind;
    list[new_prev].next = elem;
    list[new_next].prev = elem;
    list[elem].prev = new_prev;
    list[elem].next = new_next;
}

#[allow(dead_code)]
fn print_list(list: &Vec<ListElem>) {
    let mut curr = list[0].idx;
    for _ in 0..list.len() {
        println!("{},", list[curr].value);
        curr = list[curr].next;
    }
}

fn get_coords(list: &Vec<ListElem>) -> (i64,i64,i64) {
    // find the zero element
    let mut idx = 0;
    while list[idx].value != 0 { idx += 1};

    for _ in 0..1000 { idx = list[idx].next; }
    let x = list[idx].value;

    for _ in 0..1000 { idx = list[idx].next; }
    let y = list[idx].value;

    for _ in 0..1000 { idx = list[idx].next; }
    let z = list[idx].value;

    return (x,y,z)
}

