🎮 Neon Tetris – Неоновая классика в терминале
Ослепительная реализация культовой игры в стиле неон с цветными фигурами и динамическими эффектами.
Поддерживает 7 языков программирования – запустите на любом!

✨ Особенности
🌈 Неоновая графика – яркие цвета, мерцание и свечение фигур.

🎵 Атмосферный саундтрек (опционально) – звуковые эффекты при вращении, падении и удалении линий.

⌨️ Интуитивное управление – стрелки для перемещения и вращения, пробел для мгновенного падения.

📊 Подсчёт очков и уровня – ускорение с каждым уровнем.

🔮 Предпросмотр следующей фигуры – планируйте свои ходы.

💾 Сохранение рекорда – лучший результат сохраняется локально.

⚡ Кроссплатформенность – работает в терминалах Linux, macOS и Windows (с поддержкой ANSI-цветов).

🎮 Управление
Клавиша	Действие
← / →	Движение влево / вправо
↑	Поворот фигуры по часовой стрелке
↓	Ускоренное падение
Пробел	Мгновенное падение (drop)
P	Пауза
Q	Выход
📦 Поддерживаемые языки
Язык	Файл	Библиотеки терминала
Python	neon_tetris.py	curses
Go	neon_tetris.go	tcell
Rust	neon_tetris.rs	crossterm, tui
JavaScript	neon_tetris.js	blessed
C#	neon_tetris.cs	System.Console
Java	NeonTetris.java	lanterna
C++	neon_tetris.cpp	ncurses
🚀 Быстрый старт
1. Склонируйте репозиторий
bash
git clone https://github.com/yourname/neon-tetris.git
cd neon-tetris
2. Установите зависимости и запустите
Python (требуется curses, встроена в Unix, на Windows установите windows-curses)

bash
pip install windows-curses  # только для Windows
python neon_tetris.py
Go

bash
go mod init neon_tetris
go get github.com/gdamore/tcell/v2
go run neon_tetris.go
Rust

bash
cargo new neon_tetris --bin
# добавьте зависимости в Cargo.toml
cargo run
JavaScript (Node.js)

bash
npm install blessed
node neon_tetris.js
C#

bash
dotnet new console -n neon_tetris
dotnet run
Java (сборка с Maven/Gradle)

bash
javac -cp .:lanterna.jar NeonTetris.java
java -cp .:lanterna.jar NeonTetris
C++ (сборка с ncurses)

bash
g++ -std=c++17 neon_tetris.cpp -lncurses -pthread -o neon_tetris
./neon_tetris
📸 Скриншот (терминал)
text
┌──────────────────────────────────┐
│  NEON TETRIS  🟢🟡🔵🟣          │
│  Score: 420   Level: 3           │
│  Lines: 12                       │
│  Next: [🟢]                      │
│                                   │
│       [][][]   []                 │
│       []       []                 │
│                []                 │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│                                   │
│  Use arrows, P-pause, Q-quit    │
└──────────────────────────────────┘
🛠️ Дополнительные опции (в большинстве реализаций)
--no-color – отключить цвет.

--speed <мс> – начальная скорость падения в миллисекундах.

--help – справка.

📄 Лицензия
MIT – свободно используйте, модифицируйте и распространяйте.

🤝 Вклад
Приветствуются pull request'ы! Если хотите добавить новый язык или улучшить существующий – создавайте issue.

🧠 Авторы
Проект создан в образовательных целях для демонстрации игрового цикла на разных языках.
