use crossterm::{
    self, cursor,
    event::{read, Event, KeyCode, KeyModifiers},
    style::Print,
    terminal, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

struct Player {
    x: i32,
    y: i32,
}

fn get_term_pos() -> (i32, i32) {
    match cursor::position() {
        Ok((x, y)) => (x as i32, y as i32),
        _ => (0, 0),
    }
}

fn draw_border(stdout: &mut Stdout, width: i32, height: i32) {
    stdout.queue(cursor::SavePosition);

    let (start_x, start_y) = get_term_pos();

    for x in start_x..(width + start_x) {
        if x == start_x {
            stdout.queue(Print("┌"));
        } else if x == (start_x + width - 1) {
            stdout.queue(Print("┐"));
        } else {
            stdout.queue(Print("─"));
        }
    }

    for y in (start_y + 1)..(height + start_y - 1) {
        stdout.queue(cursor::MoveTo(start_x as u16, y as u16));
        stdout.queue(Print("│"));
        stdout.queue(cursor::MoveToColumn((start_x + width) as u16));
        stdout.queue(Print("│"));
    }

    stdout.queue(cursor::MoveTo(
        start_x as u16,
        (start_y + height - 1) as u16,
    ));
    for x in start_x..(width + start_x) {
        if x == start_x {
            stdout.queue(Print("└"));
        } else if x == (start_x + width - 1) {
            stdout.queue(Print("┘"));
        } else {
            stdout.queue(Print("─"));
        }
    }

    stdout.queue(cursor::RestorePosition);
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
enum GameEvent {
    Move(Dir),
    Quit,
    NoOp,
}

fn get_game_event() -> GameEvent {
    let event = match read() {
        Ok(Event::Key(event)) => Some((event.code, event.modifiers)),
        _ => None,
    };

    match event {
        Some((KeyCode::Char('q'), _)) => GameEvent::Quit,
        Some((KeyCode::Char('c'), KeyModifiers::CONTROL)) => GameEvent::Quit,
        Some((KeyCode::Up, _)) => GameEvent::Move(Dir::Up),
        Some((KeyCode::Down, _)) => GameEvent::Move(Dir::Down),
        Some((KeyCode::Left, _)) => GameEvent::Move(Dir::Left),
        Some((KeyCode::Right, _)) => GameEvent::Move(Dir::Right),
        _ => GameEvent::NoOp,
    }
}

fn main() {
    let mut stdout = stdout();

    let width = 50;
    let height = 30;

    let mut p = Player {
        x: width / 2,
        y: height / 2,
    };

    let orig_size = terminal::size().unwrap();
    let (width_canvas, height_canvas) = orig_size;

    stdout.queue(terminal::Clear(terminal::ClearType::All));
    stdout.queue(terminal::SetSize(width_canvas as u16, height_canvas as u16));
    stdout.queue(cursor::Hide);
    terminal::enable_raw_mode();
    stdout.flush();

    loop {
        stdout.queue(cursor::MoveTo(0, 0));

        draw_border(&mut stdout, width_canvas as i32, height_canvas as i32);
        stdout.queue(cursor::MoveTo(1, 1));
        draw_border(&mut stdout, (width + 2), (height + 2));
        stdout.queue(cursor::MoveTo(2, 2));

        let (start_x, start_y) = get_term_pos();

        for y in start_y..height + start_y {
            stdout.queue(cursor::MoveTo(start_x as u16, y as u16));

            for x in start_x..width + start_x {
                if x == p.x && y == p.y {
                    stdout.queue(Print("@"));
                } else {
                    stdout.queue(Print("."));
                }
            }
        }

        stdout.flush();

        let ev = get_game_event();

        match ev {
            GameEvent::Quit => break,
            GameEvent::NoOp => (),
            GameEvent::Move(dir) => match dir {
                Dir::Up => p.y -= 1,
                Dir::Down => p.y += 1,
                Dir::Left => p.x -= 1,
                Dir::Right => p.x += 1,
            },
        }
    }

    stdout.queue(cursor::MoveTo(0, 0));
    stdout.queue(terminal::Clear(terminal::ClearType::All));
    stdout.queue(cursor::Show);
    stdout.queue(terminal::SetSize(orig_size.0, orig_size.1));
    stdout.flush();
}
