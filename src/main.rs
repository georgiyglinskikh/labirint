use rand::random;

type Cell = u8;

// Cell possible states
const NONE: Cell = 0b00;
const RIGHT: Cell = 0b01;
const DOWN: Cell = 0b10;
const BOTH: Cell = 0b11;

type Line = Vec<Cell>;

struct Labirint {
    width: usize,
    height: usize,
    lines: Vec<Line>,
    counter: u8,
    sets: Line,
    walls: Line,
}

impl Labirint {
    fn new(width: usize, height: usize) -> Self {
        Labirint {
            width,
            height,
            counter: 0,
            lines: Vec::with_capacity(width),
            sets: vec![0; width],
            walls: vec![BOTH; width],
        }
    }

    fn generate(&mut self) {
        for row in 0..self.height {
            self.refresh_sets();

            self.build_walls();

            self.connect_sets_right();

            if row != self.height - 1 {
                self.connect_sets_down();
            } else {
                self.break_walls();
            }

            self.lines.push(self.walls.clone());
        }
    }

    fn refresh_sets(&mut self) {
        for cell in self.sets.iter_mut() {
            if *cell == 0 {
                *cell = self.counter;
                self.counter += 1;
            }
        }
    }

    fn build_walls(&mut self) {
        self.walls = vec![BOTH; self.width];
    }

    fn connect_sets_right(&mut self) {
        for x in 0..self.width - 1 {
            let connect_right = random::<u8>() % 2 == 0;
            let equal = self.sets[x] == self.sets[x + 1];
            if connect_right && !equal {
                self.walls[x] ^= RIGHT;

                self.sets[x + 1] = self.sets[x];
            }
        }
    }

    fn connect_sets_down(&mut self) {
        let mut connected_sets: Line = Line::with_capacity(self.width);

        for x in 0..self.width {
            let connect_down = random::<u8>() % 2 == 0;
            let connected = connected_sets.contains(&self.sets[x]);
            if connect_down || !connected {
                self.walls[x] ^= DOWN;
                connected_sets.push(self.sets[x]);
            } else {
                self.sets[x] = 0;
            }
        }
    }

    fn break_walls(&mut self) {
        for x in 0..self.width - 1 {
            if self.sets[x] != self.sets[x + 1] && self.walls[x] & RIGHT != 0 {
                self.walls[x] ^= RIGHT;
            }
            self.sets[x + 1] = self.sets[x];
        }
    }
}

impl ToString for Labirint {
    fn to_string(&self) -> String {
        let mut buf = String::new();

        let new_line = '\n';
        let none = "  ";
        let right = " |";
        let down = "__";
        let both = "_|";

        for row in self.lines.iter() {
            for cell in row.iter() {
                String::push_str(
                    &mut buf,
                    match *cell {
                        NONE => none,
                        RIGHT => right,
                        DOWN => down,
                        BOTH => both,
                        _ => continue,
                    },
                );
            }
            String::push(&mut buf, new_line);
        }

        buf
    }
}

fn main() {
    let mut lab = Labirint::new(10, 10);
    lab.generate();

    println!("{}", "**".repeat(lab.width));
    print!("{}", lab.to_string());
    println!("{}", "**".repeat(lab.width));
}
