import { Board, Square } from "four-in-a-row";
import { memory } from "four-in-a-row/four_in_a_row_bg";

const SQUARE_SIZE = 50; // px
const PIECE_RADIUS = SQUARE_SIZE / 2.3;

const GRID_COLOR = "#bbbbbb";
const P1_COLOR = "#6699ff"
const P2_COLOR = "#ff4d4d"

let board = Board.new();
const BOARD_WIDTH = board.width();
const BOARD_HEIGHT = board.height();

const canvas = document.getElementById("board-canvas");
canvas.width = (SQUARE_SIZE + 1) * BOARD_WIDTH + 1;
canvas.height = (SQUARE_SIZE + 1) * BOARD_HEIGHT + 1;
const ctx = canvas.getContext('2d');

function drawEmptyBoard() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    board.set(1, 2, Square.P1)
    board.set(4, 1, Square.P2)

    // Vertical lines.
    for (let i = 0; i <= BOARD_WIDTH; i++) {
        ctx.moveTo(i * SQUARE_SIZE, 0)
        ctx.lineTo(i * SQUARE_SIZE, SQUARE_SIZE * BOARD_HEIGHT)
    }

    // Horizontal lines.
    for (let i = 0; i <= BOARD_HEIGHT; i++) {
        ctx.moveTo(0, i * SQUARE_SIZE)
        ctx.lineTo(BOARD_WIDTH * SQUARE_SIZE, i * SQUARE_SIZE)
    }
    ctx.stroke();
}

function drawPieces() {
    let squaresPtr = board.squares();
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

function drawSinglePiece(i, j, color) {
    ctx.beginPath();
    ctx.fillStyle = color;
    const xOffset = (i + 0.5) * SQUARE_SIZE
    const yOffset = (j + 0.5) * SQUARE_SIZE
    ctx.moveTo(xOffset, yOffset)
    ctx.arc(xOffset,
        yOffset,
        PIECE_RADIUS,
        0,
        2 * Math.PI);
    ctx.fill();
}

drawEmptyBoard()
drawPieces()

// const renderLoop = () => {
//     universe.tick();

//     drawGrid();
//     drawCells();

//     requestAnimationFrame(renderLoop);
// };

// drawGrid();
// drawCells();
// requestAnimationFrame(renderLoop);
