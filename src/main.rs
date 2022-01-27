fn main() {
    // Field size
    const SIZE: usize = 10;

    // Cell possible states
    const NONE: u8 = 0b00;
    const RIGHT: u8 = 0b01;
    const DOWN: u8 = 0b10;
    const BOTH: u8 = 0b11;

    let mut labirint = [[0u8; SIZE]; SIZE];

    let mut sets = [0u8; SIZE];
    let mut walls = [RIGHT; SIZE];

    let mut counter = 1;
    for row in 0..labirint.len() {
        for cell in sets.iter_mut() {
            if *cell == 0 {
                *cell = counter;
                counter += 1;
            }
        }

        walls = [BOTH; SIZE];

        for x in 0..(SIZE - 1) {
            use rand::random;

            let connect_right = random::<u8>() % 2 == 0;
            let equal = sets[x] == sets[x + 1];
            if connect_right && !equal {
                walls[x] ^= RIGHT;

                sets[x + 1] = sets[x];
            }
        }

        if row != labirint.len() - 1 {
            let mut connected_sets: Vec<u8> = Vec::new();

            for x in 0..SIZE - 1 {
                use rand::random;

                let connect_down = random::<u8>() % 2 == 0;
                let connected = connected_sets.contains(&sets[x]);
                if connect_down || !connected {
                    walls[x] ^= DOWN;
                    connected_sets.push(sets[x]);
                } else {
                    sets[x] = 0;
                }
            }
        } else {
            for x in 0..(SIZE - 1) {
                if sets[x] != sets[x + 1] && walls[x] & RIGHT != 0 {
                    walls[x] ^= RIGHT;
                }
                sets[x + 1] = sets[x];
            }
        }

        labirint[row] = walls;
    }

    // Print field to the screen
    println!("{}", "**".repeat(SIZE));
    for row in labirint.iter() {
        for cell in row.iter() {
            match *cell {
                NONE => print!("  "),
                RIGHT => print!(" |"),
                DOWN => print!("_ "),
                BOTH => print!("_|"),
                _ => continue,
            }
        }
        println!();
    }
    println!("{}", "**".repeat(SIZE));
}
