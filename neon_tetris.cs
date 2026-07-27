// neon_tetris.cs
using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;

class NeonTetris
{
    const int WIDTH = 10;
    const int HEIGHT = 20;
    static readonly int[][][] SHAPES = new int[][][]
    {
        new int[][] { new int[] {1,1,1,1} },
        new int[][] { new int[] {1,1}, new int[] {1,1} },
        new int[][] { new int[] {0,1,0}, new int[] {1,1,1} },
        new int[][] { new int[] {1,0,0}, new int[] {1,1,1} },
        new int[][] { new int[] {0,0,1}, new int[] {1,1,1} },
        new int[][] { new int[] {0,1,1}, new int[] {1,1,0} },
        new int[][] { new int[] {1,1,0}, new int[] {0,1,1} }
    };
    static readonly ConsoleColor[] COLORS = new ConsoleColor[]
    {
        ConsoleColor.Cyan, ConsoleColor.Yellow, ConsoleColor.Magenta,
        ConsoleColor.Green, ConsoleColor.Red, ConsoleColor.Blue, ConsoleColor.White
    };

    static int[][] board = new int[HEIGHT][];
    static int score = 0, lines = 0, level = 1;
    static int fallTime = 500;
    static DateTime lastFall = DateTime.Now;
    static bool gameOver = false, paused = false;
    static int[][] currentShape;
    static ConsoleColor currentColor;
    static int pieceX, pieceY;
    static int[][] nextShape;
    static ConsoleColor nextColor;

    static void Main()
    {
        Console.CursorVisible = false;
        Console.Title = "Neon Tetris";
        Console.BackgroundColor = ConsoleColor.Black;
        Console.Clear();

        for (int i = 0; i < HEIGHT; i++) board[i] = new int[WIDTH];

        Random rand = new Random();
        nextShape = SHAPES[rand.Next(SHAPES.Length)];
        nextColor = COLORS[rand.Next(COLORS.Length)];
        SpawnPiece(rand);

        Task.Run(() => InputLoop());

        while (!gameOver)
        {
            if (!paused)
            {
                if ((DateTime.Now - lastFall).TotalMilliseconds > fallTime)
                {
                    MovePiece(0, 1);
                    lastFall = DateTime.Now;
                }
            }
            Draw();
            Thread.Sleep(20);
        }
        Console.SetCursorPosition(0, HEIGHT + 4);
        Console.WriteLine("GAME OVER");
        Console.ReadKey();
    }

    static void SpawnPiece(Random rand)
    {
        currentShape = nextShape;
        currentColor = nextColor;
        nextShape = SHAPES[rand.Next(SHAPES.Length)];
        nextColor = COLORS[rand.Next(COLORS.Length)];
        pieceX = WIDTH / 2 - currentShape[0].Length / 2;
        pieceY = 0;
        if (Collision(currentShape, pieceX, pieceY)) gameOver = true;
    }

    static bool Collision(int[][] shape, int x, int y)
    {
        for (int row = 0; row < shape.Length; row++)
            for (int col = 0; col < shape[row].Length; col++)
                if (shape[row][col] != 0)
                {
                    int bx = x + col;
                    int by = y + row;
                    if (bx < 0 || bx >= WIDTH || by >= HEIGHT) return true;
                    if (by >= 0 && board[by][bx] != 0) return true;
                }
        return false;
    }

    static void LockPiece()
    {
        for (int row = 0; row < currentShape.Length; row++)
            for (int col = 0; col < currentShape[row].Length; col++)
                if (currentShape[row][col] != 0)
                {
                    int by = pieceY + row;
                    int bx = pieceX + col;
                    if (by >= 0) board[by][bx] = 1;
                }
        ClearLines();
        Random rand = new Random();
        SpawnPiece(rand);
    }

    static void ClearLines()
    {
        int cleared = 0;
        for (int y = HEIGHT - 1; y >= 0; y--)
        {
            bool full = true;
            for (int x = 0; x < WIDTH; x++) if (board[y][x] == 0) { full = false; break; }
            if (full)
            {
                for (int ny = y; ny > 0; ny--) board[ny] = board[ny - 1];
                board[0] = new int[WIDTH];
                cleared++;
                y++;
            }
        }
        if (cleared > 0)
        {
            lines += cleared;
            score += cleared * 100;
            level = lines / 10 + 1;
            fallTime = Math.Max(100, 500 - (level - 1) * 30);
        }
    }

    static void MovePiece(int dx, int dy)
    {
        int nx = pieceX + dx;
        int ny = pieceY + dy;
        if (!Collision(currentShape, nx, ny))
        {
            pieceX = nx;
            pieceY = ny;
            return;
        }
        if (dy == 1) LockPiece();
    }

    static void Rotate()
    {
        int rows = currentShape.Length;
        int cols = currentShape[0].Length;
        int[][] rotated = new int[cols][];
        for (int i = 0; i < cols; i++) rotated[i] = new int[rows];
        for (int i = 0; i < rows; i++)
            for (int j = 0; j < cols; j++)
                rotated[j][rows - 1 - i] = currentShape[i][j];
        if (!Collision(rotated, pieceX, pieceY))
        {
            currentShape = rotated;
        }
        else
        {
            foreach (int dx in new int[] { -1, 1 })
                if (!Collision(rotated, pieceX + dx, pieceY))
                {
                    pieceX += dx;
                    currentShape = rotated;
                    break;
                }
        }
    }

    static void Drop()
    {
        while (!Collision(currentShape, pieceX, pieceY + 1)) pieceY++;
        LockPiece();
    }

    static void InputLoop()
    {
        while (true)
        {
            var key = Console.ReadKey(true).Key;
            if (key == ConsoleKey.Q) Environment.Exit(0);
            if (key == ConsoleKey.P) paused = !paused;
            if (paused) continue;
            switch (key)
            {
                case ConsoleKey.LeftArrow: MovePiece(-1, 0); break;
                case ConsoleKey.RightArrow: MovePiece(1, 0); break;
                case ConsoleKey.DownArrow: MovePiece(0, 1); break;
                case ConsoleKey.UpArrow: Rotate(); break;
                case ConsoleKey.Spacebar: Drop(); break;
            }
        }
    }

    static void Draw()
    {
        Console.SetCursorPosition(0, 0);
        // Рамка
        Console.Write("┌");
        for (int i = 0; i < WIDTH; i++) Console.Write("─");
        Console.WriteLine("┐");
        for (int y = 0; y < HEIGHT; y++)
        {
            Console.Write("│");
            for (int x = 0; x < WIDTH; x++)
            {
                char ch = ' ';
                ConsoleColor color = ConsoleColor.White;
                if (board[y][x] != 0) { ch = '█'; color = COLORS[(x + y) % COLORS.Length]; }
                // current
                for (int row = 0; row < currentShape.Length; row++)
                    for (int col = 0; col < currentShape[row].Length; col++)
                        if (currentShape[row][col] != 0)
                        {
                            int by = pieceY + row;
                            int bx = pieceX + col;
                            if (by == y && bx == x) { ch = '█'; color = currentColor; }
                        }
                if (ch != ' ')
                {
                    Console.ForegroundColor = color;
                    Console.Write(ch);
                    Console.ResetColor();
                }
                else Console.Write(' ');
            }
            Console.WriteLine("│");
        }
        Console.Write("└");
        for (int i = 0; i < WIDTH; i++) Console.Write("─");
        Console.WriteLine("┘");
        Console.WriteLine($"Score: {score}  Level: {level}  Lines: {lines}");
        if (paused) Console.WriteLine("PAUSED");
        if (gameOver) Console.WriteLine("GAME OVER");
    }
}
