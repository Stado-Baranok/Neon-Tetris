// NeonTetris.java
import com.googlecode.lanterna.TerminalSize;
import com.googlecode.lanterna.input.KeyStroke;
import com.googlecode.lanterna.input.KeyType;
import com.googlecode.lanterna.screen.Screen;
import com.googlecode.lanterna.screen.TerminalScreen;
import com.googlecode.lanterna.terminal.DefaultTerminalFactory;
import com.googlecode.lanterna.terminal.Terminal;

import java.io.IOException;
import java.util.Random;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

public class NeonTetris {
    private static final int WIDTH = 10;
    private static final int HEIGHT = 20;
    private static final int[][][] SHAPES = {
        {{1,1,1,1}},
        {{1,1},{1,1}},
        {{0,1,0},{1,1,1}},
        {{1,0,0},{1,1,1}},
        {{0,0,1},{1,1,1}},
        {{0,1,1},{1,1,0}},
        {{1,1,0},{0,1,1}}
    };
    private static final String[] COLORS = {"cyan", "yellow", "magenta", "green", "red", "blue", "white"};

    private Screen screen;
    private int[][] board = new int[HEIGHT][WIDTH];
    private int score = 0, lines = 0, level = 1;
    private int fallTime = 500;
    private long lastFall = System.currentTimeMillis();
    private boolean gameOver = false, paused = false;
    private int[][] currentShape;
    private String currentColor;
    private int pieceX, pieceY;
    private int[][] nextShape;
    private String nextColor;
    private Random rand = new Random();

    public NeonTetris() throws IOException {
        Terminal terminal = new DefaultTerminalFactory().createTerminal();
        screen = new TerminalScreen(terminal);
        screen.startScreen();
        screen.setCursorPosition(null);
        screen.clear();
        nextShape = SHAPES[rand.nextInt(SHAPES.length)];
        nextColor = COLORS[rand.nextInt(COLORS.length)];
        spawnPiece();
        Executors.newSingleThreadScheduledExecutor().scheduleAtFixedRate(this::tick, 0, 20, TimeUnit.MILLISECONDS);
        handleInput();
    }

    private void spawnPiece() {
        currentShape = nextShape;
        currentColor = nextColor;
        nextShape = SHAPES[rand.nextInt(SHAPES.length)];
        nextColor = COLORS[rand.nextInt(COLORS.length)];
        pieceX = WIDTH / 2 - currentShape[0].length / 2;
        pieceY = 0;
        if (collision(currentShape, pieceX, pieceY)) gameOver = true;
    }

    private boolean collision(int[][] shape, int x, int y) {
        for (int row = 0; row < shape.length; row++)
            for (int col = 0; col < shape[row].length; col++)
                if (shape[row][col] != 0) {
                    int bx = x + col;
                    int by = y + row;
                    if (bx < 0 || bx >= WIDTH || by >= HEIGHT) return true;
                    if (by >= 0 && board[by][bx] != 0) return true;
                }
        return false;
    }

    private void lockPiece() {
        for (int row = 0; row < currentShape.length; row++)
            for (int col = 0; col < currentShape[row].length; col++)
                if (currentShape[row][col] != 0) {
                    int by = pieceY + row;
                    int bx = pieceX + col;
                    if (by >= 0) board[by][bx] = 1;
                }
        clearLines();
        spawnPiece();
    }

    private void clearLines() {
        int cleared = 0;
        for (int y = HEIGHT - 1; y >= 0; y--) {
            boolean full = true;
            for (int x = 0; x < WIDTH; x++) if (board[y][x] == 0) { full = false; break; }
            if (full) {
                System.arraycopy(board, 0, board, 1, y);
                board[0] = new int[WIDTH];
                cleared++;
                y++;
            }
        }
        if (cleared > 0) {
            lines += cleared;
            score += cleared * 100;
            level = lines / 10 + 1;
            fallTime = Math.max(100, 500 - (level - 1) * 30);
        }
    }

    private void movePiece(int dx, int dy) {
        int nx = pieceX + dx;
        int ny = pieceY + dy;
        if (!collision(currentShape, nx, ny)) {
            pieceX = nx;
            pieceY = ny;
            return;
        }
        if (dy == 1) lockPiece();
    }

    private void rotate() {
        int rows = currentShape.length;
        int cols = currentShape[0].length;
        int[][] rotated = new int[cols][rows];
        for (int i = 0; i < rows; i++)
            for (int j = 0; j < cols; j++)
                rotated[j][rows - 1 - i] = currentShape[i][j];
        if (!collision(rotated, pieceX, pieceY)) {
            currentShape = rotated;
        } else {
            for (int dx : new int[]{-1, 1}) {
                if (!collision(rotated, pieceX + dx, pieceY)) {
                    pieceX += dx;
                    currentShape = rotated;
                    break;
                }
            }
        }
    }

    private void drop() {
        while (!collision(currentShape, pieceX, pieceY + 1)) pieceY++;
        lockPiece();
    }

    private void tick() {
        if (!paused && !gameOver) {
            long now = System.currentTimeMillis();
            if (now - lastFall > fallTime) {
                movePiece(0, 1);
                lastFall = now;
            }
            draw();
        }
    }

    private void draw() {
        try {
            screen.clear();
            // рамка
            for (int y = 0; y < HEIGHT + 2; y++) {
                for (int x = 0; x < WIDTH + 2; x++) {
                    if (y == 0 || y == HEIGHT + 1 || x == 0 || x == WIDTH + 1)
                        screen.setCharacter(x, y, ' ');
                }
            }
            // поле
            for (int y = 0; y < HEIGHT; y++) {
                for (int x = 0; x < WIDTH; x++) {
                    if (board[y][x] != 0) {
                        screen.setCharacter(x + 1, y + 1, '█');
                    }
                }
            }
            // current
            for (int row = 0; row < currentShape.length; row++) {
                for (int col = 0; col < currentShape[row].length; col++) {
                    if (currentShape[row][col] != 0) {
                        int by = pieceY + row + 1;
                        int bx = pieceX + col + 1;
                        if (by >= 1 && by <= HEIGHT)
                            screen.setCharacter(bx, by, '█');
                    }
                }
            }
            // info
            String info = String.format("Score: %d  Level: %d  Lines: %d", score, level, lines);
            for (int i = 0; i < info.length(); i++) {
                screen.setCharacter(WIDTH + 5 + i, 1, info.charAt(i));
            }
            if (paused) {
                String p = "PAUSED";
                for (int i = 0; i < p.length(); i++)
                    screen.setCharacter(WIDTH/2 + i, HEIGHT/2, p.charAt(i));
            }
            if (gameOver) {
                String go = "GAME OVER";
                for (int i = 0; i < go.length(); i++)
                    screen.setCharacter(WIDTH/2 + i, HEIGHT/2, go.charAt(i));
            }
            screen.refresh();
        } catch (IOException e) { e.printStackTrace(); }
    }

    private void handleInput() {
        try {
            while (true) {
                KeyStroke key = screen.pollInput();
                if (key == null) {
                    Thread.sleep(20);
                    continue;
                }
                if (key.getKeyType() == KeyType.Character && key.getCharacter() == 'q') break;
                if (key.getKeyType() == KeyType.Character && key.getCharacter() == 'p') paused = !paused;
                if (paused) continue;
                switch (key.getKeyType()) {
                    case ArrowLeft: movePiece(-1, 0); break;
                    case ArrowRight: movePiece(1, 0); break;
                    case ArrowDown: movePiece(0, 1); break;
                    case ArrowUp: rotate(); break;
                }
                if (key.getKeyType() == KeyType.Character && key.getCharacter() == ' ') drop();
            }
        } catch (Exception e) { e.printStackTrace(); }
        try { screen.stopScreen(); } catch (IOException e) { e.printStackTrace(); }
        System.exit(0);
    }

    public static void main(String[] args) throws IOException {
        new NeonTetris();
    }
}
