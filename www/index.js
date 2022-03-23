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

let steps = document.getElementById("steps");

let animationId = null;
const playPauseButton = document.getElementById("play-pause");

const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        const fps = (1/delta) * 1000; // fps = fpms * millisecs in a second

        this.frames.push(fps);
        if(this.frames.length > 100) { // 100 capacity
            this.frames.shift();
        }

        // statistics
        let max = -Infinity;
        let min = Infinity;
        let sum = 0;
        for(let i=0; i<this.frames.length; i++) {
            sum += this.frames[i];
            max = Math.max(this.frames[i], max);
            min = Math.min(this.frames[i], min);
        }
        let mean = sum / this.frames.length;

        this.fps.textContent = `
        Frames per second: 
                               latest: ${Math.round(fps)}
        Avg. over last 100 iterations: ${Math.round(mean)}
         Min over last 100 iterations: ${Math.round(min)}
         Max over last 100 iterations: ${Math.round(max)}
        `.trim();
        this.lastFrameTimeStamp = performance.now();
    }
};

const renderLoop = () => {
    //debugger;
    fps.render();

    drawGrid();
    drawCells();

    for(let i=0; i<steps.value; i++) {
        universe.tick();
    }

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


canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left)*scaleX;
    const canvasTop = (event.clientY - boundingRect.top)*scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE+1)), height-1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE+1)), width-1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});


drawGrid();
drawCells();
play(); // requestAnimationFrame(renderLoop);
