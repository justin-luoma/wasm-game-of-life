import {Grid} from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const grid = Grid.new(75, 75);
grid.spawn_glider_gun(30, 15);
grid.spawn_pulsar(10, 35);
grid.spawn_pentadecanthlon(25, 55);

const renderLoop = () => {
    pre.textContent = grid.render();


    requestAnimationFrame(renderLoop);
};

const spawn = () => {
    grid.spawn_glider(2 ,2);
    setTimeout(() => {
        spawn();
    }, 15000);
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

function reset() {
    grid.reset();
}

const setup = () => {
    const randomizeBtn = document.getElementById("randomize");
    randomizeBtn.addEventListener("click", randomize);

    const resetBtn = document.getElementById("reset");
    resetBtn.addEventListener("click", reset);
};

setup();

requestAnimationFrame(renderLoop);

render();

// setTimeout(() => spawn(), 1000);


