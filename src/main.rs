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
    for row in labirint.iter_mut() {
        for cell in sets.iter_mut() {
            if *cell == 0 {
                *cell = counter;
                counter += 1;
            }
        }

        for wall in walls.iter_mut() {
            *wall = NONE;
        }

        for x in 0..(SIZE - 1) {
            use rand::random;

            let place_right = random::<u8>() % 2 == 0;
            if place_right {
                walls[x] |= RIGHT;
            } else {
                sets[x + 1] = sets[x];
            }
        }

        for x in 0..SIZE {
            use rand::random;

            let place_down = random::<u8>() % 2 == 0;
            if place_down {
                walls[x] |= DOWN;
            }
        }

        *row = walls;
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
