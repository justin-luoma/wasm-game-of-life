import {Grid, get_patterns_as_string} from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const grid = Grid.new(100, 100);
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
    const x = document.getElementById("x").value
    const y = document.getElementById("y").value
    const pattern = document.getElementById("spawnSelection").value;
    console.log(pattern);
    grid.spawn_pattern(pattern, x, y);
    // grid.spawn_glider(2 ,2);
    // setTimeout(() => {
    //     spawn();
    // }, 15000);
};

const render = () => {
    grid.step_forward();
    setTimeout(() => {
        render();
    }, 250);
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

const setup = () => {
    const randomizeBtn = document.getElementById("randomize");
    randomizeBtn.addEventListener("click", randomize);

    const randomizeCenterBtn = document.getElementById("randomizeCenter");
    randomizeCenterBtn.addEventListener("click", randomizeCenter);

    const resetBtn = document.getElementById("reset");
    resetBtn.addEventListener("click", reset);

    const spawnBtn = document.getElementById("spawn");
    spawnBtn.addEventListener("click", spawn);
    const spawnSelection = document.getElementById("spawnSelection");
    get_patterns_as_string().split(",").forEach((pattern, i) => {
        const opt = document.createElement("option");
        opt.value = i;
        opt.innerHTML = pattern;
        spawnSelection.appendChild(opt);
    })
};

setup();

requestAnimationFrame(renderLoop);

render();

console.log(get_patterns_as_string());

// setTimeout(() => spawn(), 1000);


