// neon_tetris.js
const blessed = require('blessed');
const screen = blessed.screen({
    smartCSR: true,
    title: 'Neon Tetris'
});

const WIDTH = 10;
const HEIGHT = 20;
const SHAPES = [
    [[1,1,1,1]],
    [[1,1],[1,1]],
    [[0,1,0],[1,1,1]],
    [[1,0,0],[1,1,1]],
    [[0,0,1],[1,1,1]],
    [[0,1,1],[1,1,0]],
    [[1,1,0],[0,1,1]]
];
const COLORS = ['cyan', 'yellow', 'magenta', 'green', 'red', 'blue', 'white'];

const board = Array.from({length: HEIGHT}, () => Array(WIDTH).fill(0));
let score = 0, lines = 0, level = 1, fallTime = 500, lastFall = Date.now();
let gameOver = false, paused = false;
let currentPiece = null, nextPiece = null, pieceX, pieceY;

function newPiece() {
    const idx = Math.floor(Math.random() * SHAPES.length);
    return { shape: SHAPES[idx], color: COLORS[idx] };
}

function spawnPiece() {
    currentPiece = nextPiece || newPiece();
    nextPiece = newPiece();
    pieceX = Math.floor(WIDTH/2 - currentPiece.shape[0].length/2);
    pieceY = 0;
    if (collision(currentPiece, pieceX, pieceY)) {
        gameOver = true;
    }
}

function collision(piece, x, y) {
    for (let row=0; row<piece.shape.length; row++) {
        for (let col=0; col<piece.shape[row].length; col++) {
            if (piece.shape[row][col]) {
                const bx = x + col;
                const by = y + row;
                if (bx < 0 || bx >= WIDTH || by >= HEIGHT) return true;
                if (by >= 0 && board[by][bx]) return true;
            }
        }
    }
    return false;
}

function lockPiece() {
    for (let row=0; row<currentPiece.shape.length; row++) {
        for (let col=0; col<currentPiece.shape[row].length; col++) {
            if (currentPiece.shape[row][col]) {
                const by = pieceY + row;
                const bx = pieceX + col;
                if (by >= 0) board[by][bx] = 1;
            }
        }
    }
    clearLines();
    spawnPiece();
}

function clearLines() {
    let cleared = 0;
    for (let y=HEIGHT-1; y>=0; y--) {
        if (board[y].every(v => v !== 0)) {
            board.splice(y, 1);
            board.unshift(Array(WIDTH).fill(0));
            cleared++;
            y++; // recheck
        }
    }
    if (cleared) {
        lines += cleared;
        score += cleared * 100;
        level = Math.floor(lines / 10) + 1;
        fallTime = Math.max(100, 500 - (level-1) * 30);
    }
}

function movePiece(dx, dy) {
    const nx = pieceX + dx;
    const ny = pieceY + dy;
    if (!collision(currentPiece, nx, ny)) {
        pieceX = nx;
        pieceY = ny;
        return true;
    }
    if (dy === 1) lockPiece();
    return false;
}

function rotatePiece() {
    const shape = currentPiece.shape;
    const rotated = shape[0].map((_, idx) => shape.map(row => row[idx]).reverse());
    const newPiece = { shape: rotated, color: currentPiece.color };
    if (!collision(newPiece, pieceX, pieceY)) {
        currentPiece = newPiece;
    } else {
        // wallkick
        for (const dx of [-1, 1]) {
            if (!collision(newPiece, pieceX+dx, pieceY)) {
                pieceX += dx;
                currentPiece = newPiece;
                break;
            }
        }
    }
}

function drop() {
    while (!collision(currentPiece, pieceX, pieceY+1)) pieceY++;
    lockPiece();
}

// UI
const box = blessed.box({
    top: 'center',
    left: 'center',
    width: 40,
    height: 24,
    border: { type: 'line' },
    style: { border: { fg: 'cyan' } }
});

const content = blessed.box({
    parent: box,
    top: 1,
    left: 1,
    width: 38,
    height: 22,
    content: ''
});
screen.append(box);

function render() {
    let out = '';
    // рамка поля
    out += '┌' + '─'.repeat(WIDTH) + '┐\n';
    for (let y=0; y<HEIGHT; y++) {
        out += '│';
        for (let x=0; x<WIDTH; x++) {
            let ch = ' ';
            let color = 'white';
            if (board[y][x]) {
                ch = '█'; color = COLORS[(x+y) % 6];
            }
            // current
            for (let row=0; row<currentPiece.shape.length; row++) {
                for (let col=0; col<currentPiece.shape[row].length; col++) {
                    if (currentPiece.shape[row][col]) {
                        const by = pieceY + row;
                        const bx = pieceX + col;
                        if (by === y && bx === x) {
                            ch = '█'; color = currentPiece.color;
                        }
                    }
                }
            }
            // цвет
            if (ch !== ' ') {
                out += `{${color}-fg}${ch}{/}`;
            } else {
                out += ' ';
            }
        }
        out += '│\n';
    }
    out += '└' + '─'.repeat(WIDTH) + '┘\n';
    out += `Score: ${score}  Level: ${level}  Lines: ${lines}\n`;
    out += `Next: `;
    // next piece preview
    for (let row=0; row<nextPiece.shape.length; row++) {
        for (let col=0; col<nextPiece.shape[row].length; col++) {
            if (nextPiece.shape[row][col]) {
                out += `{${nextPiece.color}-fg}█{/}`;
            } else out += ' ';
        }
        out += ' ';
    }
    if (paused) out += '\nPAUSED';
    if (gameOver) out += '\nGAME OVER';
    content.setContent(out);
    screen.render();
}

// key handling
screen.key(['q', 'C-c'], () => process.exit(0));
screen.key(['p'], () => { paused = !paused; });
screen.key(['left'], () => { if (!paused) movePiece(-1,0); });
screen.key(['right'], () => { if (!paused) movePiece(1,0); });
screen.key(['down'], () => { if (!paused) movePiece(0,1); });
screen.key(['up'], () => { if (!paused) rotatePiece(); });
screen.key(['space'], () => { if (!paused) drop(); });

// инициализация
spawnPiece();

// game loop
setInterval(() => {
    if (!paused && !gameOver) {
        const now = Date.now();
        if (now - lastFall > fallTime) {
            movePiece(0, 1);
            lastFall = now;
        }
    }
    render();
}, 30);

screen.render();
