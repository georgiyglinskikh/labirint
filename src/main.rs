use rand::random;

type Cell = u8;

// Cell possible states
const NONE: Cell = 0b00;
const RIGHT: Cell = 0b01;
const DOWN: Cell = 0b10;
const BOTH: Cell = 0b11;

type Line = Vec<Cell>;
type Labirint = Vec<Line>;

fn main() {
    let lab = generate_labitint(10, 10);

    print_labirint(&lab);
}

fn generate_labitint(width: usize, height: usize) -> Labirint {
    let mut labirint = Labirint::with_capacity(height);

    let mut sets: Line = vec![0; width];
    let mut walls: Line = vec![BOTH; width];

    let mut set_counter = 1;
    for row in 0..height {
        refresh_sets(&mut sets, &mut set_counter);

        build_walls(&mut walls, width);

        connect_sets_right(&mut sets, &mut walls, width);

        if row != height - 1 {
            connect_sets_down(&mut sets, &mut walls, width);
        } else {
            break_walls(&mut sets, &mut walls, width);
        }

        labirint.push(walls.clone());
    }

    labirint
}

fn build_walls(walls: &mut Line, width: usize) {
    *walls = vec![BOTH; width];
}

fn refresh_sets(sets: &mut Line, counter: &mut Cell) {
    for cell in sets.iter_mut() {
        if *cell == 0 {
            *cell = *counter;
            *counter += 1;
        }
    }
}

fn connect_sets_right(sets: &mut Line, walls: &mut Line, width: usize) {
    for x in 0..width - 1 {
        let connect_right = random::<u8>() % 2 == 0;
        let equal = sets[x] == sets[x + 1];
        if connect_right && !equal {
            walls[x] ^= RIGHT;

            sets[x + 1] = sets[x];
        }
    }
}

fn connect_sets_down(sets: &mut Line, walls: &mut Line, width: usize) {
    let mut connected_sets: Line = Line::with_capacity(width);

    for x in 0..width {
        let connect_down = random::<u8>() % 2 == 0;
        let connected = connected_sets.contains(&sets[x]);
        if connect_down || !connected {
            walls[x] ^= DOWN;
            connected_sets.push(sets[x]);
        } else {
            sets[x] = 0;
        }
    }
}

fn break_walls(sets: &mut Line, walls: &mut Line, width: usize) {
    for x in 0..width - 1 {
        if sets[x] != sets[x + 1] && walls[x] & RIGHT != 0 {
            walls[x] ^= RIGHT;
        }
        sets[x + 1] = sets[x];
    }
}

fn print_labirint(labirint: &Labirint) {
    println!("{}", "**".repeat(labirint.len()));
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
    println!("{}", "**".repeat(labirint.len()));
}
