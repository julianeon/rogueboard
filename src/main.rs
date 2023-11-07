use std::io;

struct Game {
    board_size: usize,
    player_x: usize,
    player_y: usize,
    is_running: bool,
}

impl Game {
    fn new(board_size: usize) -> Self {
        let player_x = board_size / 2;
        let player_y = board_size / 2;
        let is_running = true;

        Self {
            board_size,
            player_x,
            player_y,
            is_running,
        }
    }

    fn print_board(&self) {
        for y in 0..self.board_size {
            for x in 0..self.board_size {
                if x == self.player_x && y == self.player_y {
                    print!("@");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    fn handle_input(&mut self, input: &str) {
        match input.trim() {
            "X" => self.is_running = false,
            "w" if self.player_y > 0 => self.player_y -= 1,
            "a" if self.player_x > 0 => self.player_x -= 1,
            "s" if self.player_y < self.board_size - 1 => self.player_y += 1,
            "d" if self.player_x < self.board_size - 1 => self.player_x += 1,
            _ => {},
        }
    }

    fn run(&mut self) {
        while self.is_running {
            self.print_board();

            println!("Enter a move (w/a/s/d) or 'X' to quit:");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");

            self.handle_input(&input);

            // Clear the console screen to redraw the updated board.
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
    }
}

fn main() {
    let board_size = 10;
    let mut game = Game::new(board_size);
    game.run();
}
