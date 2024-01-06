import { Game, Square } from "four-in-a-row";
import { memory } from "four-in-a-row/four_in_a_row_bg";

const SQUARE_SIZE = 70; // px
const PIECE_RADIUS = SQUARE_SIZE / 2.3;

const GRID_COLOR = "#bbbbbb";
const HIGHLIGHT_COLOR = "#eaeaea";
const P1_COLOR = "#6699ff"
const P2_COLOR = "#ff4d4d"

let game = Game.new();
const BOARD_WIDTH = game.board_width();
const BOARD_HEIGHT = game.board_height();

const boardCanvas = document.getElementById("board-canvas");
boardCanvas.width = (SQUARE_SIZE + 1) * BOARD_WIDTH + 1;
boardCanvas.height = (SQUARE_SIZE + 1) * BOARD_HEIGHT + 1;
const boardCtx = boardCanvas.getContext('2d');

const gameStatePre = document.getElementById("game-state-pre");

const elemLeft = boardCanvas.offsetLeft + boardCanvas.clientLeft
const elemTop = boardCanvas.offsetTop + boardCanvas.clientTop

function getBoardColumn(mouseEvent) {
    return Math.floor((mouseEvent.pageX - elemLeft) / SQUARE_SIZE) + 1;
}

boardCanvas.addEventListener('click', function (event) {
    if (game.is_human_move()) {
        var x = getBoardColumn(event);
        game.make_move(x)
        draw()
    }
}, false);

let highlightedColumn = -1;
boardCanvas.addEventListener('mousemove', function (event) {
    var x = getBoardColumn(event) - 1;

    if (x < 0 || x >= BOARD_WIDTH) {
        return
    }

    highlightedColumn = x;
    draw()
}, false);

function drawEmptyBoard() {
    boardCtx.beginPath();
    boardCtx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= BOARD_WIDTH; i++) {
        boardCtx.moveTo(i * SQUARE_SIZE, 0)
        boardCtx.lineTo(i * SQUARE_SIZE, SQUARE_SIZE * BOARD_HEIGHT)
    }

    // Horizontal lines.
    for (let i = 0; i <= BOARD_HEIGHT; i++) {
        boardCtx.moveTo(0, i * SQUARE_SIZE)
        boardCtx.lineTo(BOARD_WIDTH * SQUARE_SIZE, i * SQUARE_SIZE)
    }
    boardCtx.stroke();
}

function drawPieces() {
    let squaresPtr = game.squares();
    let squares = new Uint8Array(memory.buffer, squaresPtr, BOARD_WIDTH * BOARD_HEIGHT);

    for (let i = 0; i < BOARD_WIDTH; i++) {
        for (let j = 0; j < BOARD_HEIGHT; j++) {
            let index = j * BOARD_WIDTH + i

            if (squares[index] === Square.P1) {
                drawSinglePiece(i, j, P1_COLOR)
            } else if (squares[index] === Square.P2) {
                drawSinglePiece(i, j, P2_COLOR)
            }
        }
    }
}

function drawHightlightedColumn() {
    boardCtx.beginPath();
    boardCtx.fillStyle = HIGHLIGHT_COLOR;
    boardCtx.rect(highlightedColumn * SQUARE_SIZE, 0, SQUARE_SIZE, SQUARE_SIZE * BOARD_HEIGHT);
    boardCtx.fill();
}

function drawSinglePiece(i, j, color) {
    boardCtx.beginPath();
    boardCtx.fillStyle = color;
    const xOffset = (i + 0.5) * SQUARE_SIZE
    const yOffset = (j + 0.5) * SQUARE_SIZE
    boardCtx.moveTo(xOffset, yOffset)
    boardCtx.arc(xOffset,
        yOffset,
        PIECE_RADIUS,
        0,
        2 * Math.PI);
    boardCtx.fill();
}

function draw() {
    boardCtx.clearRect(0, 0, boardCanvas.width, boardCanvas.height);
    drawHightlightedColumn()
    drawEmptyBoard()
    drawPieces()
    updateGameStateText()
}

function updateGameStateText() {
    switch (game.game_state()) {
        case 0:
            return
        case 1:
        case 2:
            gameStatePre.textContent = `Game Over!\nPlayer ${game.current_player()} won`
            return
        case 3:
            gameStatePre.textContent = `Game Over!\nDraw`
            return
        default:
            console.log(`Got illegal game state ${game.game_state()}`);
    }
}

draw()

setInterval(() => {
    if (!game.is_human_move()) {
        game.make_computer_move()
        draw()
    }
}, 1000)


// setInterval(function () {
//     if (!game.is_human_move()) {
//         sleep(200)
//         game.make_computer_move()
//         draw()
//     }
// }, 2000000);
