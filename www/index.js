import {Grid} from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const grid = Grid.new(75, 50);
grid.init();

const renderLoop = () => {
    pre.textContent = grid.render();
    grid.step_forward();



    requestAnimationFrame(renderLoop);
};

const spawn = () => {
    setTimeout(() => {
        grid.spawn_new()
        spawn()
    }, 1000);
};

requestAnimationFrame(renderLoop);

setTimeout(() => spawn(), 1000);
