// neon_tetris.go
package main

import (
	"math/rand"
	"os"
	"time"

	"github.com/gdamore/tcell/v2"
)

const (
	WIDTH  = 10
	HEIGHT = 20
)

var shapes = [][][]int{
	{{1, 1, 1, 1}},
	{{1, 1}, {1, 1}},
	{{0, 1, 0}, {1, 1, 1}},
	{{1, 0, 0}, {1, 1, 1}},
	{{0, 0, 1}, {1, 1, 1}},
	{{0, 1, 1}, {1, 1, 0}},
	{{1, 1, 0}, {0, 1, 1}},
}

var colors = []tcell.Color{
	tcell.ColorCyan,
	tcell.ColorYellow,
	tcell.ColorMagenta,
	tcell.ColorGreen,
	tcell.ColorRed,
	tcell.ColorBlue,
	tcell.ColorWhite,
}

type Piece struct {
	shape [][]int
	color tcell.Color
}

type Tetris struct {
	screen    tcell.Screen
	board     [][]int
	score     int
	lines     int
	level     int
	fallTime  int
	lastFall  time.Time
	gameOver  bool
	paused    bool
	current   *Piece
	next      *Piece
	pieceX    int
	pieceY    int
}

func NewTetris(s tcell.Screen) *Tetris {
	t := &Tetris{
		screen:   s,
		board:    make([][]int, HEIGHT),
		score:    0,
		lines:    0,
		level:    1,
		fallTime: 500,
		lastFall: time.Now(),
	}
	for i := range t.board {
		t.board[i] = make([]int, WIDTH)
	}
	t.spawnPiece()
	return t
}

func (t *Tetris) spawnPiece() {
	if t.next == nil {
		t.next = t.newPiece()
	}
	t.current = t.next
	t.next = t.newPiece()
	t.pieceX = WIDTH/2 - len(t.current.shape[0])/2
	t.pieceY = 0
	if t.collision(t.current, t.pieceX, t.pieceY) {
		t.gameOver = true
	}
}

func (t *Tetris) newPiece() *Piece {
	idx := rand.Intn(len(shapes))
	return &Piece{
		shape: shapes[idx],
		color: colors[idx],
	}
}

func (t *Tetris) collision(p *Piece, x, y int) bool {
	for row := 0; row < len(p.shape); row++ {
		for col := 0; col < len(p.shape[row]); col++ {
			if p.shape[row][col] != 0 {
				bx := x + col
				by := y + row
				if bx < 0 || bx >= WIDTH || by >= HEIGHT {
					return true
				}
				if by >= 0 && t.board[by][bx] != 0 {
					return true
				}
			}
		}
	}
	return false
}

func (t *Tetris) lockPiece() {
	p := t.current
	for row := 0; row < len(p.shape); row++ {
		for col := 0; col < len(p.shape[row]); col++ {
			if p.shape[row][col] != 0 {
				by := t.pieceY + row
				bx := t.pieceX + col
				if by >= 0 {
					t.board[by][bx] = 1 // цвет не важен, используем индекс
				}
			}
		}
	}
	t.clearLines()
	t.spawnPiece()
}

func (t *Tetris) clearLines() {
	cleared := 0
	for y := HEIGHT - 1; y >= 0; y-- {
		full := true
		for x := 0; x < WIDTH; x++ {
			if t.board[y][x] == 0 {
				full = false
				break
			}
		}
		if full {
			// удаляем строку
			copy(t.board[1:y+1], t.board[0:y])
			t.board[0] = make([]int, WIDTH)
			cleared++
			y++ // повторно проверим ту же строку
		}
	}
	if cleared > 0 {
		t.lines += cleared
		t.score += cleared * 100
		t.level = t.lines/10 + 1
		t.fallTime = max(100, 500-(t.level-1)*30)
	}
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func (t *Tetris) move(dx, dy int) {
	nx := t.pieceX + dx
	ny := t.pieceY + dy
	if !t.collision(t.current, nx, ny) {
		t.pieceX = nx
		t.pieceY = ny
		return
	}
	if dy == 1 { // падение
		t.lockPiece()
	}
}

func (t *Tetris) rotate() {
	shape := t.current.shape
	rows := len(shape)
	cols := len(shape[0])
	rotated := make([][]int, cols)
	for i := range rotated {
		rotated[i] = make([]int, rows)
	}
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			rotated[j][rows-1-i] = shape[i][j]
		}
	}
	newPiece := &Piece{shape: rotated, color: t.current.color}
	if !t.collision(newPiece, t.pieceX, t.pieceY) {
		t.current = newPiece
	} else {
		// wallkick - попробуем сдвинуть влево или вправо
		for _, dx := range []int{-1, 1} {
			if !t.collision(newPiece, t.pieceX+dx, t.pieceY) {
				t.pieceX += dx
				t.current = newPiece
				break
			}
		}
	}
}

func (t *Tetris) drop() {
	for !t.collision(t.current, t.pieceX, t.pieceY+1) {
		t.pieceY++
	}
	t.lockPiece()
}

func (t *Tetris) draw() {
	t.screen.Clear()
	// Рамка
	style := tcell.StyleDefault.Background(tcell.ColorBlack)
	for y := 0; y < HEIGHT+2; y++ {
		for x := 0; x < WIDTH+2; x++ {
			if y == 0 || y == HEIGHT+1 || x == 0 || x == WIDTH+1 {
				t.screen.SetContent(x, y, ' ', nil, style)
			}
		}
	}
	// Поле
	for y := 0; y < HEIGHT; y++ {
		for x := 0; x < WIDTH; x++ {
			if t.board[y][x] != 0 {
				t.screen.SetContent(x+1, y+1, '█', nil, tcell.StyleDefault.Foreground(colors[t.board[y][x]-1]))
			}
		}
	}
	// Текущая фигура
	p := t.current
	for row := 0; row < len(p.shape); row++ {
		for col := 0; col < len(p.shape[row]); col++ {
			if p.shape[row][col] != 0 {
				by := t.pieceY + row + 1
				bx := t.pieceX + col + 1
				if by >= 1 && by <= HEIGHT {
					t.screen.SetContent(bx, by, '█', nil, tcell.StyleDefault.Foreground(p.color))
				}
			}
		}
	}
	// Info
	t.screen.SetContent(WIDTH+5, 1, 'S', nil, style)
	t.screen.SetContent(WIDTH+6, 1, ':', nil, style)
	scoreStr := []rune("Score: " + string(rune(t.score)))
	for i, r := range []rune("Score: " + string(rune(t.score))) {
		t.screen.SetContent(WIDTH+5+i, 1, r, nil, style)
	}
	// Обновляем экран
	t.screen.Show()
}

func main() {
	rand.Seed(time.Now().UnixNano())
	s, err := tcell.NewScreen()
	if err != nil {
		panic(err)
	}
	if err := s.Init(); err != nil {
		panic(err)
	}
	defer s.Fini()
	s.EnableMouse()
	s.EnablePaste()
	s.Clear()
	game := NewTetris(s)

	go func() {
		for {
			ev := s.PollEvent()
			switch ev := ev.(type) {
			case *tcell.EventKey:
				if ev.Key() == tcell.KeyEscape || ev.Rune() == 'q' {
					os.Exit(0)
				}
				if ev.Rune() == 'p' {
					game.paused = !game.paused
				}
				if game.paused {
					continue
				}
				switch ev.Key() {
				case tcell.KeyLeft:
					game.move(-1, 0)
				case tcell.KeyRight:
					game.move(1, 0)
				case tcell.KeyDown:
					game.move(0, 1)
				case tcell.KeyUp:
					game.rotate()
				case tcell.KeySpace:
					game.drop()
				}
			}
		}
	}()

	for !game.gameOver {
		now := time.Now()
		if !game.paused && now.Sub(game.lastFall).Milliseconds() > int64(game.fallTime) {
			game.move(0, 1)
			game.lastFall = now
		}
		game.draw()
		time.Sleep(20 * time.Millisecond)
	}
}
