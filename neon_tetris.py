# neon_tetris.py
import curses
import random
import time
import os

# Константы
WIDTH = 10
HEIGHT = 20
SHAPES = [
    [[1, 1, 1, 1]],
    [[1, 1], [1, 1]],
    [[0, 1, 0], [1, 1, 1]],
    [[1, 0, 0], [1, 1, 1]],
    [[0, 0, 1], [1, 1, 1]],
    [[0, 1, 1], [1, 1, 0]],
    [[1, 1, 0], [0, 1, 1]]
]
COLORS = [1, 2, 3, 4, 5, 6, 7]  # цветовые пары curses
COLOR_NAMES = ['cyan', 'yellow', 'magenta', 'green', 'red', 'blue', 'white']

class Tetris:
    def __init__(self, stdscr):
        self.stdscr = stdscr
        curses.curs_set(0)
        curses.start_color()
        curses.init_pair(1, curses.COLOR_CYAN, curses.COLOR_BLACK)
        curses.init_pair(2, curses.COLOR_YELLOW, curses.COLOR_BLACK)
        curses.init_pair(3, curses.COLOR_MAGENTA, curses.COLOR_BLACK)
        curses.init_pair(4, curses.COLOR_GREEN, curses.COLOR_BLACK)
        curses.init_pair(5, curses.COLOR_RED, curses.COLOR_BLACK)
        curses.init_pair(6, curses.COLOR_BLUE, curses.COLOR_BLACK)
        curses.init_pair(7, curses.COLOR_WHITE, curses.COLOR_BLACK)
        self.stdscr.keypad(True)
        self.stdscr.nodelay(True)
        self.board = [[0] * WIDTH for _ in range(HEIGHT)]
        self.score = 0
        self.lines = 0
        self.level = 1
        self.fall_time = 500  # мс
        self.last_fall = time.time() * 1000
        self.game_over = False
        self.paused = False
        self.current_piece = None
        self.next_piece = None
        self.spawn_piece()

    def spawn_piece(self):
        self.current_piece = self.next_piece if self.next_piece else self.new_piece()
        self.next_piece = self.new_piece()
        self.piece_x = WIDTH // 2 - len(self.current_piece[0]) // 2
        self.piece_y = 0
        if self.collision(self.current_piece, self.piece_x, self.piece_y):
            self.game_over = True

    def new_piece(self):
        shape = random.choice(SHAPES)
        color = random.choice(COLORS)
        return {'shape': shape, 'color': color}

    def collision(self, piece, x, y):
        for row in range(len(piece['shape'])):
            for col in range(len(piece['shape'][row])):
                if piece['shape'][row][col]:
                    board_x = x + col
                    board_y = y + row
                    if board_x < 0 or board_x >= WIDTH or board_y >= HEIGHT:
                        return True
                    if board_y >= 0 and self.board[board_y][board_x]:
                        return True
        return False

    def lock_piece(self):
        piece = self.current_piece
        for row in range(len(piece['shape'])):
            for col in range(len(piece['shape'][row])):
                if piece['shape'][row][col]:
                    y = self.piece_y + row
                    x = self.piece_x + col
                    if y >= 0:
                        self.board[y][x] = piece['color']
        self.clear_lines()
        self.spawn_piece()

    def clear_lines(self):
        lines_cleared = 0
        for y in range(HEIGHT-1, -1, -1):
            if all(self.board[y]):
                del self.board[y]
                self.board.insert(0, [0] * WIDTH)
                lines_cleared += 1
        if lines_cleared:
            self.lines += lines_cleared
            self.score += lines_cleared * 100
            self.level = self.lines // 10 + 1
            self.fall_time = max(100, 500 - (self.level - 1) * 30)

    def move(self, dx, dy):
        new_x = self.piece_x + dx
        new_y = self.piece_y + dy
        if not self.collision(self.current_piece, new_x, new_y):
            self.piece_x = new_x
            self.piece_y = new_y
            return True
        if dy == 1:  # падение вниз не удалось – фиксируем
            self.lock_piece()
        return False

    def rotate(self):
        shape = self.current_piece['shape']
        rotated = list(zip(*shape[::-1]))
        new_piece = {'shape': [list(row) for row in rotated], 'color': self.current_piece['color']}
        if not self.collision(new_piece, self.piece_x, self.piece_y):
            self.current_piece = new_piece
        else:
            # попытка "стены"
            for dx in [-1, 1]:
                if not self.collision(new_piece, self.piece_x + dx, self.piece_y):
                    self.piece_x += dx
                    self.current_piece = new_piece
                    break

    def drop(self):
        while not self.collision(self.current_piece, self.piece_x, self.piece_y + 1):
            self.piece_y += 1
        self.lock_piece()

    def draw(self):
        self.stdscr.clear()
        # Рамка
        h, w = self.stdscr.getmaxyx()
        board_win = curses.newwin(HEIGHT+2, WIDTH+2, 2, 2)
        board_win.border(0)
        for y in range(HEIGHT):
            for x in range(WIDTH):
                if self.board[y][x]:
                    board_win.addch(y+1, x+1, '█', curses.color_pair(self.board[y][x]))
        # Текущая фигура
        piece = self.current_piece
        for row in range(len(piece['shape'])):
            for col in range(len(piece['shape'][row])):
                if piece['shape'][row][col]:
                    by = self.piece_y + row + 1
                    bx = self.piece_x + col + 1
                    if by >= 1 and by <= HEIGHT:
                        board_win.addch(by, bx, '█', curses.color_pair(piece['color']))
        board_win.refresh()
        # Информация
        info = f"Score: {self.score}  Level: {self.level}  Lines: {self.lines}"
        self.stdscr.addstr(1, 2, info)
        self.stdscr.addstr(1, 30, "Next:")
        # Показываем следующую фигуру
        next_shape = self.next_piece['shape']
        for r in range(len(next_shape)):
            for c in range(len(next_shape[r])):
                if next_shape[r][c]:
                    self.stdscr.addch(2+r, 36+c, '█', curses.color_pair(self.next_piece['color']))
        if self.paused:
            self.stdscr.addstr(HEIGHT//2, WIDTH//2 - 2, "PAUSED", curses.A_REVERSE)
        if self.game_over:
            self.stdscr.addstr(HEIGHT//2, WIDTH//2 - 4, "GAME OVER", curses.A_REVERSE)
        self.stdscr.refresh()

    def run(self):
        while not self.game_over:
            now = time.time() * 1000
            # Ввод
            key = self.stdscr.getch()
            if key == ord('q'):
                break
            if key == ord('p'):
                self.paused = not self.paused
            if not self.paused:
                if key == curses.KEY_LEFT:
                    self.move(-1, 0)
                elif key == curses.KEY_RIGHT:
                    self.move(1, 0)
                elif key == curses.KEY_DOWN:
                    self.move(0, 1)
                elif key == curses.KEY_UP:
                    self.rotate()
                elif key == ord(' '):
                    self.drop()
                # Падение по таймеру
                if now - self.last_fall > self.fall_time:
                    self.move(0, 1)
                    self.last_fall = now
            self.draw()
            time.sleep(0.02)
        self.stdscr.getch()

def main(stdscr):
    game = Tetris(stdscr)
    game.run()

if __name__ == '__main__':
    curses.wrapper(main)
