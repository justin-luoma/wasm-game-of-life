import {get_patterns_as_string, Grid} from "wasm-game-of-life";
import {memory} from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#b3b3b3";
const ALIVE_COLOR = "#000000";


const width = 200;
const height = 200;

const grid = Grid.new(width, height);

const canvas = document.getElementById("game-of-life-canvas");
const cursorSpawn = document.getElementById("cursorSpawn");

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

grid.spawn_glider_gun(75, 15);
grid.spawn_pulsar(10, 35);
grid.spawn_pentadecanthlon(25, 55);
grid.spawn_acorn(25, 175);
grid.spawn_r_pentomino(180, 25);
grid.spawn_glider_loop(100, 100);
grid.spawn_pulsar(100, 100);
grid.spawn_infinite_growth_1(175, 175);

const ctx = canvas.getContext("2d");

let animationId = null;

const renderLoop = () => {

    drawGrid();
    drawCells();

    // for (let i = 0; i < 9; i++) {
    grid.step_forward();
    // }

    animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animationId === null;
};


const spawn = () => {
    const x = document.getElementById("xSpawn").value
    const y = document.getElementById("ySpawn").value
    const pattern = document.getElementById("spawnSelection").value;
    grid.spawn_pattern(pattern, x, y);
};

function randomize() {
    grid.randomize();
}

function randomizeCenter() {
    grid.randomize_center();
}

function reset() {
    grid.reset();
}

const pauseBtn = document.getElementById("pause");

function pause() {
    pauseBtn.innerText = "Resume";
    cancelAnimationFrame(animationId);
    animationId = null;
}

function play() {
    pauseBtn.innerText = "Pause";
    renderLoop();
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = grid.get_cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    // Alive cells.
    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (!cells[idx]) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    // Dead cells.
    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx]) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};


canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const x = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
    const y = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);

    console.log("click", x, y);

    if (cursorSpawn.value === "99") {
        grid.revive_cell(x, y);
    } else {
        const pattern = cursorSpawn.value;
        grid.spawn_pattern(pattern, x, y);
    }

    drawGrid();
    drawCells();
});

function reviveCell() {
    const coordString = document.getElementById("coordsRevive").value;
    let coords = coordString.split("\n");
    coords.forEach(coord => {
        if (coord.split(",").length === 2) {
            const [x, y] = coord.split(",");
            grid.revive_cell(x, y);
        } else {
            const [x, y] = coord.split(".");
            grid.revive_cell(x, y);
        }
    });
}

const setup = () => {
    const randomizeBtn = document.getElementById("randomize");
    randomizeBtn.addEventListener("click", randomize);

    const randomizeCenterBtn = document.getElementById("randomizeCenter");
    randomizeCenterBtn.addEventListener("click", randomizeCenter);

    const resetBtn = document.getElementById("reset");
    resetBtn.addEventListener("click", reset);

    const pauseBtn = document.getElementById("pause");
    pauseBtn.addEventListener("click", () => {
        if (isPaused()) {
            play();
        } else {
            pause();
        }
    });

    const spawnBtn = document.getElementById("spawn");
    spawnBtn.addEventListener("click", spawn);
    const spawnSelection = document.getElementById("spawnSelection");
    const patterns = get_patterns_as_string()
    patterns.split(",").forEach((pattern, i) => {
        const opt = document.createElement("option");
        opt.value = i;
        opt.innerHTML = pattern;
        spawnSelection.appendChild(opt);
    });

    const dot = document.createElement("option");
    dot.value = 99;
    dot.innerText = "Dot";
    cursorSpawn.appendChild(dot);
    patterns.split(",").forEach((pattern, i) => {
        const opt = document.createElement("option");
        opt.value = i;
        opt.innerHTML = pattern;
        cursorSpawn.appendChild(opt);
    });

    const reviveBtn = document.getElementById("revive");
    reviveBtn.addEventListener("click", reviveCell);

    const dimensions = document.getElementById("dimensions");
    dimensions.innerText = `${width} x ${height}`
};

setup();
play();

// requestAnimationFrame(renderLoop);

// tick();


