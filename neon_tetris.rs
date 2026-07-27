// neon_tetris.rs
use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEventKind},
    execute, terminal,
};
use std::io::stdout;
use std::time::{Duration, Instant};
use rand::Rng;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

const SHAPES: [[[u8; 4]; 4]; 7] = [
    // I
    [[0,0,0,0],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
    // O
    [[1,1],[1,1]],
    // T
    [[0,1,0],[1,1,1]],
    // L
    [[1,0,0],[1,1,1]],
    // J
    [[0,0,1],[1,1,1]],
    // S
    [[0,1,1],[1,1,0]],
    // Z
    [[1,1,0],[0,1,1]],
];

struct Piece {
    shape: Vec<Vec<u8>>,
    color: u16, // цвет для терминала
}

impl Piece {
    fn new(idx: usize) -> Self {
        let raw = SHAPES[idx];
        let mut shape = Vec::new();
        for row in raw.iter() {
            let mut r = Vec::new();
            for &v in row {
                if v != 0 { r.push(1); } else { r.push(0); }
            }
            if !r.is_empty() { shape.push(r); }
        }
        let color = 33 + (idx % 6) as u16; // ANSI коды цветов
        Piece { shape, color }
    }
}

struct Tetris {
    board: [[u8; WIDTH]; HEIGHT],
    score: u32,
    lines: u32,
    level: u32,
    fall_time: u64,
    last_fall: Instant,
    game_over: bool,
    paused: bool,
    current: Piece,
    next: Piece,
    piece_x: usize,
    piece_y: usize,
}

impl Tetris {
    fn new() -> Self {
        let mut board = [[0u8; WIDTH]; HEIGHT];
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..SHAPES.len());
        let current = Piece::new(idx);
        let next = Piece::new(rng.gen_range(0..SHAPES.len()));
        Tetris {
            board,
            score: 0,
            lines: 0,
            level: 1,
            fall_time: 500,
            last_fall: Instant::now(),
            game_over: false,
            paused: false,
            current,
            next,
            piece_x: WIDTH/2 - 1,
            piece_y: 0,
        }
    }

    fn spawn_piece(&mut self) {
        self.current = self.next;
        let mut rng = rand::thread_rng();
        self.next = Piece::new(rng.gen_range(0..SHAPES.len()));
        self.piece_x = WIDTH/2 - self.current.shape[0].len()/2;
        self.piece_y = 0;
        if self.collision(&self.current, self.piece_x, self.piece_y) {
            self.game_over = true;
        }
    }

    fn collision(&self, piece: &Piece, x: usize, y: usize) -> bool {
        for row in 0..piece.shape.len() {
            for col in 0..piece.shape[row].len() {
                if piece.shape[row][col] == 1 {
                    let bx = x + col;
                    let by = y + row;
                    if bx >= WIDTH || by >= HEIGHT {
                        return true;
                    }
                    if by < HEIGHT && self.board[by][bx] != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn lock_piece(&mut self) {
        for row in 0..self.current.shape.len() {
            for col in 0..self.current.shape[row].len() {
                if self.current.shape[row][col] == 1 {
                    let by = self.piece_y + row;
                    let bx = self.piece_x + col;
                    if by < HEIGHT {
                        self.board[by][bx] = 1;
                    }
                }
            }
        }
        self.clear_lines();
        self.spawn_piece();
    }

    fn clear_lines(&mut self) {
        let mut cleared = 0;
        let mut y = HEIGHT as i32 - 1;
        while y >= 0 {
            let mut full = true;
            for x in 0..WIDTH {
                if self.board[y as usize][x] == 0 {
                    full = false;
                    break;
                }
            }
            if full {
                // сдвигаем строки вниз
                for ny in (1..=y as usize).rev() {
                    self.board[ny] = self.board[ny-1];
                }
                self.board[0] = [0; WIDTH];
                cleared += 1;
                // остаемся на той же строке
                y += 1;
            }
            y -= 1;
        }
        if cleared > 0 {
            self.lines += cleared;
            self.score += cleared * 100;
            self.level = self.lines / 10 + 1;
            self.fall_time = std::cmp::max(100, 500 - (self.level - 1) * 30);
        }
    }

    fn move_piece(&mut self, dx: i32, dy: i32) {
        let nx = (self.piece_x as i32 + dx) as usize;
        let ny = (self.piece_y as i32 + dy) as usize;
        if !self.collision(&self.current, nx, ny) {
            self.piece_x = nx;
            self.piece_y = ny;
            return;
        }
        if dy == 1 {
            self.lock_piece();
        }
    }

    fn rotate(&mut self) {
        // поворот матрицы
        let rows = self.current.shape.len();
        let cols = self.current.shape[0].len();
        let mut rotated = vec![vec![0u8; rows]; cols];
        for i in 0..rows {
            for j in 0..cols {
                rotated[j][rows-1-i] = self.current.shape[i][j];
            }
        }
        let new_piece = Piece { shape: rotated, color: self.current.color };
        if !self.collision(&new_piece, self.piece_x, self.piece_y) {
            self.current = new_piece;
        } else {
            // wallkick - пробуем сдвинуть
            for dx in vec![-1, 1] {
                let nx = (self.piece_x as i32 + dx) as usize;
                if !self.collision(&new_piece, nx, self.piece_y) {
                    self.piece_x = nx;
                    self.current = new_piece;
                    break;
                }
            }
        }
    }

    fn drop(&mut self) {
        while !self.collision(&self.current, self.piece_x, self.piece_y + 1) {
            self.piece_y += 1;
        }
        self.lock_piece();
    }

    fn draw(&self) {
        let mut output = String::new();
        // очистка
        output.push_str("\x1b[2J\x1b[H");
        // рамка
        output.push_str("+");
        for _ in 0..WIDTH { output.push('-'); }
        output.push_str("+\n");
        for y in 0..HEIGHT {
            output.push('|');
            for x in 0..WIDTH {
                let mut ch = ' ';
                let mut color = 0;
                // поле
                if self.board[y][x] != 0 {
                    ch = '█';
                    color = 33 + (x % 6) as u16;
                }
                // текущая фигура
                for row in 0..self.current.shape.len() {
                    for col in 0..self.current.shape[row].len() {
                        if self.current.shape[row][col] == 1 {
                            let by = self.piece_y + row;
                            let bx = self.piece_x + col;
                            if by == y && bx == x {
                                ch = '█';
                                color = self.current.color;
                            }
                        }
                    }
                }
                if ch != ' ' {
                    output.push_str(&format!("\x1b[38;5;{}m{}\x1b[0m", color, ch));
                } else {
                    output.push(' ');
                }
            }
            output.push_str("|\n");
        }
        output.push_str("+");
        for _ in 0..WIDTH { output.push('-'); }
        output.push_str("+\n");
        // info
        output.push_str(&format!("Score: {}  Level: {}  Lines: {}\n", self.score, self.level, self.lines));
        if self.paused { output.push_str("PAUSED\n"); }
        if self.game_over { output.push_str("GAME OVER\n"); }
        print!("{}", output);
    }

    fn run(&mut self) {
        let mut stdout = stdout();
        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
        terminal::enable_raw_mode().unwrap();

        let mut tick_counter = 0;
        loop {
            self.draw();

            if self.game_over {
                break;
            }

            // обработка ввода
            if event::poll(Duration::from_millis(20)).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('p') => self.paused = !self.paused,
                            _ if !self.paused => {
                                match key.code {
                                    KeyCode::Left => self.move_piece(-1, 0),
                                    KeyCode::Right => self.move_piece(1, 0),
                                    KeyCode::Down => self.move_piece(0, 1),
                                    KeyCode::Up => self.rotate(),
                                    KeyCode::Char(' ') => self.drop(),
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            // падение
            let now = Instant::now();
            if !self.paused && now.duration_since(self.last_fall) >= Duration::from_millis(self.fall_time) {
                self.move_piece(0, 1);
                self.last_fall = now;
            }

            std::thread::sleep(Duration::from_millis(20));
        }

        terminal::disable_raw_mode().unwrap();
        execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show).unwrap();
    }
}

fn main() {
    let mut game = Tetris::new();
    game.run();
}
