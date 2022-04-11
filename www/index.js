import {get_patterns_as_string, Grid} from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");

const widthX = 200;
const widthY = 200;

const grid = Grid.new(widthX, widthY);

let paused;

grid.spawn_glider_gun(30, 15);
grid.spawn_pulsar(10, 35);
grid.spawn_pentadecanthlon(25, 55);
grid.spawn_beacon(55, 3);
grid.spawn_toad(65, 15);

const renderLoop = () => {
    pre.textContent = grid.render();


    requestAnimationFrame(renderLoop);
};

const spawn = () => {
    const x = document.getElementById("xSpawn").value
    const y = document.getElementById("ySpawn").value
    const pattern = document.getElementById("spawnSelection").value;
    grid.spawn_pattern(pattern, x, y);
};

const tick = () => {
    if (!paused) {
        grid.step_forward();
        setTimeout(() => {
            tick();
        }, 150);
    }
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

function pause() {
    const pauseBtn = document.getElementById("pause");
    if (paused) {
        paused = false;
        tick();
        pauseBtn.innerText = "Pause";
    } else {
        paused = true;
        pauseBtn.innerText = "Resume";
    }
}

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
    pauseBtn.addEventListener("click", pause);

    const spawnBtn = document.getElementById("spawn");
    spawnBtn.addEventListener("click", spawn);
    const spawnSelection = document.getElementById("spawnSelection");
    get_patterns_as_string().split(",").forEach((pattern, i) => {
        const opt = document.createElement("option");
        opt.value = i;
        opt.innerHTML = pattern;
        spawnSelection.appendChild(opt);
    });

    const reviveBtn = document.getElementById("revive");
    reviveBtn.addEventListener("click", reviveCell);

    const dimensions = document.getElementById("dimensions");
    dimensions.innerText = `${widthX} x ${widthY}`
};

setup();

requestAnimationFrame(renderLoop);

tick();

console.log(get_patterns_as_string());

// setTimeout(() => spawn(), 1000);


