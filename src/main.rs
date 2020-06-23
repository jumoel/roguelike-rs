use std::io::{self, Read};
struct Player {
    x: i32,
    y: i32,
}

fn main() {
    let width = 40;
    let height = 25;

    let mut p = Player {
        x: width / 2,
        y: height / 2,
    };

    loop {
        // Position the cursor at (1,1)
        print!("\x1B[1;1H");
        // Clear the screen
        print!("\x1B[2J");

        for y in -1..(height + 1) {
            for x in -1..(width + 1) {
                if x == -1 && y == -1 {
                    print!("┌");
                } else if x == width && y == -1 {
                    print!("┐")
                } else if x == -1 && y == height {
                    print!("└")
                } else if x == width && y == height {
                    print!("┘")
                } else if x == -1 || x == width {
                    print!("│")
                } else if y == -1 || y == height {
                    print!("─")
                } else if x == p.x && y == p.y {
                    print!("@")
                } else {
                    print!(".")
                }
            }
            print!("\n");
        }

        println!("Command [h/j/k/l/q]: ");

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        match s.as_str().trim() {
            "q" => break,
            "h" => p.x -= 1,
            "l" => p.x += 1,
            "j" => p.y += 1,
            "k" => p.y -= 1,
            _ => (),
        }
    }
}
