#[test]
fn day15() {
    let input = include_str!("input.txt");
    let y = 2_000_000;
    let max_coord = 4_000_000;

    // let input = include_str!("input_test.txt");
    // let y = 10;
    // let max_coord = 20;

    let sensors = parse_input(input);

    // part 1
    let count = count_cannot_contain_beacon(&sensors, y);
    println!("{}", count);

    // part 2
    let mut x = -1;
    let mut y = -1;
    for odd_even in 0..=1 { // need to consider odd and even diag values separately,
        let mut diag = odd_even;
        let mut cover_until;

        while diag <= max_coord*2 {
            (x,y,cover_until) = free_on_diagonal(&sensors, diag, max_coord);

            let steps = (cover_until - diag)/2 + 1;
            diag += 2*steps;

            if x >= 0 { break; }
        }
        if x >= 0 { break; }
    }
    println!("{}", 4_000_000*x + y);
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    bx: i64,
    by: i64,
    radius: i64,
}

fn count_cannot_contain_beacon(sensors: &Vec<Sensor>, y: i64) -> i64 {
    let mut segments = Vec::new();
    for sensor in sensors.iter() { // find all intersecting line segments
        let dy = (sensor.y - y).abs();
        if dy > sensor.radius { continue; } // row is outside radius => no intersection

        // find start and end of the intersecting line segment
        let dx = sensor.radius - dy;
        let start = sensor.x - dx;
        let end = sensor.x + dx;

        // check if there is a beacon on the row, if so, adjust endpoints of the segment
        if sensor.by != y {
            segments.push((start,end));
        } else {
            if sensor.bx == start {
                segments.push((start+1,end));
            } else if sensor.bx == end {
                segments.push((start,end-1));
            } else {
                panic!("If beacon is on the line it should be at one of the ends.");
            }
        }
    }
    segments.sort_by_key(|seg| seg.0);

    let mut seg_it = segments.iter();
    let (start_prev, mut end_prev) = seg_it.next().unwrap();
    let mut count = end_prev - start_prev + 1;

    while let Some(&(start,end)) = seg_it.next() {
        if end <= end_prev { continue; } // segment fully contained in previous

        let len = end - start + 1;
        let overlap = (end_prev - start + 1).max(0).min(len);
        count += len - overlap;

        end_prev = end;
    }
    return count;
}


fn free_on_diagonal(sensors: &Vec<Sensor>, diag: i64, max_coord: i64) -> (i64,i64,i64) {
    // examines the line given by x + y = diag

    let mut segments = Vec::new();
    for sensor in sensors.iter() { // find all intersecting line segments on the given diagonal
        let diag_max = sensor.x + sensor.y + sensor.radius; // how large diag can be and still be covered
        let diag_min = sensor.x + sensor.y - sensor.radius; // how small diag can be and still be covered

        if diag_max < diag || diag_min > diag { // no intersection
            continue;
        }

        // find x coordinates of intersections
        let x_1_num = sensor.x - sensor.y - sensor.radius + diag;
        let x_1 = if x_1_num % 2 == 0 {
            x_1_num / 2
        } else {
            (x_1_num + 1) / 2
        };

        let x_2_num = sensor.x - sensor.y + sensor.radius + diag;
        let x_2 = if x_2_num % 2 == 0 {
            x_2_num / 2
        } else {
            (x_2_num - 1) / 2
        };

        // let y_1 = diag - x_1;
        // let y_2 = diag - x_2;

        segments.push((x_1,x_2,diag_max))

    }
    segments.sort_by_key(|seg| seg.0);

    let mut seg_it = segments.iter();
    let (_, mut end_prev, mut diag_max_min) = seg_it.next().unwrap();

    while let Some(&(start,end,diag_max)) = seg_it.next() {
        if end_prev < 0 { // skip until we are in the searched for area
            end_prev = end;
            diag_max_min = diag_max;
            continue;
        }
        if end_prev >= max_coord { break; } // end if passed the area
        if end <= end_prev { continue; } // segment fully contained in previous

        if start > end_prev + 1 { // no overlap
            let x = end_prev + 1;
            let y = diag - x;
            return (x,y,0);
        }

        end_prev = end;
        diag_max_min = diag_max_min.min(diag_max);
    }
    return (-1,-1,diag_max_min);
}


fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(['=',',',':']);

        let x = parts.nth(1).unwrap().parse::<i64>().unwrap();
        let y = parts.nth(1).unwrap().parse::<i64>().unwrap();
        let bx = parts.nth(1).unwrap().parse::<i64>().unwrap();
        let by = parts.nth(1).unwrap().parse::<i64>().unwrap();
        let radius = (x-bx).abs() + (y-by).abs();
        let sensor = Sensor{x,y,bx,by,radius};

        sensors.push(sensor);
    }
    sensors
}
