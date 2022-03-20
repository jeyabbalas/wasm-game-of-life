import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
//const universe = Universe.random();
//const universe = Universe.glider();
//const universe = Universe.middleweight_spaceship();

const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext("2d");

let animationId = null;
const playPauseButton = document.getElementById("play-pause");

const renderLoop = () => {
    //debugger;

    drawGrid();
    drawCells();

    universe.tick();

    animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // vertical lines
    for(let i=0; i<=width; i++) {
        ctx.moveTo(i*(CELL_SIZE+1)+1, 0);
        ctx.lineTo(i*(CELL_SIZE+1)+1, (CELL_SIZE+1)*height+1);
    }

    // horizontal lines
    for(let j=0; j<=height; j++) {
        ctx.moveTo(0, j*(CELL_SIZE+1)+1);
        ctx.lineTo((CELL_SIZE+1)*width+1, j*(CELL_SIZE+1)+1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row*width + column;
};

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8); // bit index in byte array
    const mask = 1 << (n % 8); // all zeros except one at the index of interest
    return (arr[byte] & mask) === mask; // did that index have true?
};

const drawCells = () => {
    // direct WASM linear memory access!
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, (width*height)/8);

    ctx.beginPath();
    for(let row=0; row<height; row++) {
        for(let col=0; col<width; col++) {
            const idx = getIndex(row, col);
            ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;
            ctx.fillRect(col*(CELL_SIZE+1)+1, row*(CELL_SIZE+1)+1, CELL_SIZE, CELL_SIZE);
        }
    }

    ctx.stroke();
};

const isPaused = () => {
    return animationId === null;
};

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶️";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if(isPaused()) {
        play();
    } else {
        pause();
    }
});


drawGrid();
drawCells();
play(); // requestAnimationFrame(renderLoop);
