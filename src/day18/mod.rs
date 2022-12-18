#[test]
fn day18() {
    let input = include_str!("input.txt");
    // let input = include_str!("input_test.txt");
    // println!("{}", input);

    let coords = parse_coords(input);
    let droplet = Droplet::from(&coords);

    // part 1
    let mut surface = 0;
    for &(x,y,z) in &coords {
        surface += 6 - droplet.count_lava_neighbours(x,y,z);
    }
    println!("{}", surface);

    // part 2
    let mut surface = 0;
    for &(x,y,z) in &coords {
        surface += droplet.count_water_neighbours(x,y,z);
    }
    println!("{}", surface);
}

const SIZE: usize = 20;

struct Droplet {
    map: [[[char; SIZE]; SIZE]; SIZE] // L for lava, W for water, space for air
}

impl Droplet {
    fn from(coords: &Vec<(usize,usize,usize)>) -> Self {
        let mut droplet = Droplet{map: [[[' '; SIZE]; SIZE]; SIZE]};
        for &(x,y,z) in coords {
            droplet.map[x][y][z] = 'L';
        }

        // flood water from the faces of the volume
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if x == 0 || y == 0 || z == 0 || x == SIZE-1 || y == SIZE-1 || z == SIZE-1 {
                        droplet.flood_water(x,y,z);
                    }
                }
            }
        }

        droplet
    }

    fn count_lava_neighbours(&self, x: usize, y: usize, z: usize) -> u64 {
        let mut count = 0;

        if x > 0 && self.map[x-1][y][z] == 'L' { count += 1; }
        if y > 0 && self.map[x][y-1][z] == 'L' { count += 1; }
        if z > 0 && self.map[x][y][z-1] == 'L' { count += 1; }

        if x < SIZE-1 && self.map[x+1][y][z] == 'L' { count += 1; }
        if y < SIZE-1 && self.map[x][y+1][z] == 'L' { count += 1; }
        if z < SIZE-1 && self.map[x][y][z+1] == 'L' { count += 1; }

        count
    }

    fn count_water_neighbours(&self, x: usize, y: usize, z: usize) -> u64 {
        let mut count = 0;

        if x == 0 || self.map[x-1][y][z] == 'W' { count += 1; }
        if y == 0 || self.map[x][y-1][z] == 'W' { count += 1; }
        if z == 0 || self.map[x][y][z-1] == 'W' { count += 1; }

        if x == SIZE-1 || self.map[x+1][y][z] == 'W' { count += 1; }
        if y == SIZE-1 || self.map[x][y+1][z] == 'W' { count += 1; }
        if z == SIZE-1 || self.map[x][y][z+1] == 'W' { count += 1; }

        count
    }

    fn flood_water(&mut self, x: usize, y: usize, z: usize) {
        let mut front = Vec::new();
        front.push((x,y,z));

        while let Some((x,y,z)) = front.pop() {
            if self.map[x][y][z] != ' ' { continue; } // water or laval already exists on the space

            self.map[x][y][z] = 'W'; // W for water

            if x > 0 && self.map[x-1][y][z] == ' ' { front.push((x-1,y,z)); }
            if y > 0 && self.map[x][y-1][z] == ' ' { front.push((x,y-1,z)); }
            if z > 0 && self.map[x][y][z-1] == ' ' { front.push((x,y,z-1)); }

            if x < SIZE-1 && self.map[x+1][y][z] == ' ' { front.push((x+1,y,z)); }
            if y < SIZE-1 && self.map[x][y+1][z] == ' ' { front.push((x,y+1,z)); }
            if z < SIZE-1 && self.map[x][y][z+1] == ' ' { front.push((x,y,z+1)); }
        }
    }
}

fn parse_coords(input: &str) -> Vec<(usize,usize,usize)> {
    let mut coords = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',').map(|s| s.parse::<usize>().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();
        coords.push((x,y,z));
    }
    coords
}
