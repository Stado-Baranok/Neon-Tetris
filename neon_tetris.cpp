// neon_tetris.cpp
#include <iostream>
#include <curses.h>
#include <unistd.h>
#include <cstdlib>
#include <ctime>
#include <vector>
#include <algorithm>

using namespace std;

const int WIDTH = 10;
const int HEIGHT = 20;

// Фигуры (матрицы)
vector<vector<vector<int>>> SHAPES = {
    {{1,1,1,1}},
    {{1,1},{1,1}},
    {{0,1,0},{1,1,1}},
    {{1,0,0},{1,1,1}},
    {{0,0,1},{1,1,1}},
    {{0,1,1},{1,1,0}},
    {{1,1,0},{0,1,1}}
};

// Цветовые пары
int COLORS[] = {1,2,3,4,5,6,7};

class Tetris {
public:
    Tetris() {
        initscr();
        start_color();
        init_pair(1, COLOR_CYAN, COLOR_BLACK);
        init_pair(2, COLOR_YELLOW, COLOR_BLACK);
        init_pair(3, COLOR_MAGENTA, COLOR_BLACK);
        init_pair(4, COLOR_GREEN, COLOR_BLACK);
        init_pair(5, COLOR_RED, COLOR_BLACK);
        init_pair(6, COLOR_BLUE, COLOR_BLACK);
        init_pair(7, COLOR_WHITE, COLOR_BLACK);
        cbreak();
        noecho();
        keypad(stdscr, TRUE);
        curs_set(0);
        nodelay(stdscr, TRUE);
        timeout(20);
        srand(time(nullptr));
        board.assign(HEIGHT, vector<int>(WIDTH, 0));
        score = 0; lines = 0; level = 1;
        fallTime = 500;
        lastFall = clock();
        gameOver = false; paused = false;
        nextPiece = randomPiece();
        spawnPiece();
    }

    ~Tetris() {
        endwin();
    }

    void run() {
        while (!gameOver) {
            if (!paused) {
                clock_t now = clock();
                if ((now - lastFall) * 1000 / CLOCKS_PER_SEC > fallTime) {
                    movePiece(0, 1);
                    lastFall = now;
                }
            }
            draw();
            handleInput();
            usleep(20000);
        }
        mvprintw(HEIGHT/2, WIDTH/2 - 4, "GAME OVER");
        refresh();
        getch();
    }

private:
    vector<vector<int>> board;
    vector<vector<int>> currentShape;
    vector<vector<int>> nextShape;
    int currentColor, nextColor;
    int pieceX, pieceY;
    int score, lines, level;
    int fallTime;
    clock_t lastFall;
    bool gameOver, paused;

    vector<vector<int>> randomPiece() {
        int idx = rand() % SHAPES.size();
        return SHAPES[idx];
    }

    void spawnPiece() {
        currentShape = nextShape;
        currentColor = nextColor;
        nextShape = randomPiece();
        nextColor = rand() % 7 + 1;
        pieceX = WIDTH / 2 - currentShape[0].size() / 2;
        pieceY = 0;
        if (collision(currentShape, pieceX, pieceY)) gameOver = true;
    }

    bool collision(const vector<vector<int>>& shape, int x, int y) {
        for (int row = 0; row < shape.size(); row++) {
            for (int col = 0; col < shape[row].size(); col++) {
                if (shape[row][col]) {
                    int bx = x + col;
                    int by = y + row;
                    if (bx < 0 || bx >= WIDTH || by >= HEIGHT) return true;
                    if (by >= 0 && board[by][bx]) return true;
                }
            }
        }
        return false;
    }

    void lockPiece() {
        for (int row = 0; row < currentShape.size(); row++) {
            for (int col = 0; col < currentShape[row].size(); col++) {
                if (currentShape[row][col]) {
                    int by = pieceY + row;
                    int bx = pieceX + col;
                    if (by >= 0) board[by][bx] = currentColor;
                }
            }
        }
        clearLines();
        spawnPiece();
    }

    void clearLines() {
        int cleared = 0;
        for (int y = HEIGHT - 1; y >= 0; y--) {
            bool full = true;
            for (int x = 0; x < WIDTH; x++) if (!board[y][x]) { full = false; break; }
            if (full) {
                for (int ny = y; ny > 0; ny--) board[ny] = board[ny-1];
                board[0].assign(WIDTH, 0);
                cleared++;
                y++;
            }
        }
        if (cleared) {
            lines += cleared;
            score += cleared * 100;
            level = lines / 10 + 1;
            fallTime = max(100, 500 - (level - 1) * 30);
        }
    }

    void movePiece(int dx, int dy) {
        int nx = pieceX + dx;
        int ny = pieceY + dy;
        if (!collision(currentShape, nx, ny)) {
            pieceX = nx;
            pieceY = ny;
            return;
        }
        if (dy == 1) lockPiece();
    }

    void rotate() {
        int rows = currentShape.size();
        int cols = currentShape[0].size();
        vector<vector<int>> rotated(cols, vector<int>(rows));
        for (int i = 0; i < rows; i++)
            for (int j = 0; j < cols; j++)
                rotated[j][rows - 1 - i] = currentShape[i][j];
        if (!collision(rotated, pieceX, pieceY)) {
            currentShape = rotated;
        } else {
            for (int dx : {-1, 1}) {
                if (!collision(rotated, pieceX + dx, pieceY)) {
                    pieceX += dx;
                    currentShape = rotated;
                    break;
                }
            }
        }
    }

    void drop() {
        while (!collision(currentShape, pieceX, pieceY + 1)) pieceY++;
        lockPiece();
    }

    void draw() {
        clear();
        // рамка
        mvprintw(0, 0, "+");
        for (int i = 0; i < WIDTH; i++) addch('-');
        addch('+');
        for (int y = 0; y < HEIGHT; y++) {
            mvaddch(y+1, 0, '|');
            for (int x = 0; x < WIDTH; x++) {
                int color = 0;
                char ch = ' ';
                if (board[y][x]) {
                    color = board[y][x];
                    ch = '█';
                }
                // current
                for (int row = 0; row < currentShape.size(); row++) {
                    for (int col = 0; col < currentShape[row].size(); col++) {
                        if (currentShape[row][col]) {
                            int by = pieceY + row;
                            int bx = pieceX + col;
                            if (by == y && bx == x) {
                                color = currentColor;
                                ch = '█';
                            }
                        }
                    }
                }
                if (ch != ' ') {
                    attron(COLOR_PAIR(color));
                    mvaddch(y+1, x+1, ch);
                    attroff(COLOR_PAIR(color));
                } else mvaddch(y+1, x+1, ' ');
            }
            mvaddch(y+1, WIDTH+1, '|');
        }
        mvprintw(HEIGHT+1, 0, "+");
        for (int i = 0; i < WIDTH; i++) addch('-');
        addch('+');
        mvprintw(HEIGHT+2, 0, "Score: %d  Level: %d  Lines: %d", score, level, lines);
        if (paused) mvprintw(HEIGHT/2, WIDTH/2 - 3, "PAUSED");
        if (gameOver) mvprintw(HEIGHT/2, WIDTH/2 - 4, "GAME OVER");
        refresh();
    }

    void handleInput() {
        int ch = getch();
        if (ch == 'q') { gameOver = true; return; }
        if (ch == 'p') { paused = !paused; return; }
        if (paused) return;
        switch (ch) {
            case KEY_LEFT: movePiece(-1, 0); break;
            case KEY_RIGHT: movePiece(1, 0); break;
            case KEY_DOWN: movePiece(0, 1); break;
            case KEY_UP: rotate(); break;
            case ' ': drop(); break;
        }
    }
};

int main() {
    Tetris game;
    game.run();
    return 0;
}
