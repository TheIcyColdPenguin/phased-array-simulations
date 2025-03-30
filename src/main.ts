import "./style.css";
import init, { initialise, create_app } from "innards";

init().then(() => {
    initialise();

    const canvas = document.querySelector("canvas");
    if (!canvas) {
        alert("canvas not found");
        return;
    }
    canvas.width = Math.min(0.75 * window.innerWidth, window.innerHeight);
    canvas.height = Math.min(0.75 * window.innerWidth, window.innerHeight);

    const context = canvas.getContext("webgl2");
    if (!context) {
        alert("Couldn't get context");
        return;
    }

    const app = create_app(context, canvas.width, canvas.height);

    let phase = 0;
    let frequency = 3e9;
    let num_elements = 25;
    let spacing = 0.25;
    let zoom = 0.5;

    requestAnimationFrame(function animate(time) {
        app.draw(time / 1000, num_elements, spacing, frequency, phase, zoom);
        requestAnimationFrame(animate);
    });

    canvas.addEventListener("mousemove", (e) => {
        const mouseX = (e.clientX - canvas.offsetLeft) / canvas.clientWidth;
        const mouseY = (canvas.clientHeight - (e.clientY - canvas.offsetTop)) / canvas.clientHeight;

        const steering = Math.atan2(mouseY - 0.5, mouseX - 0.1);
        phase = 2 * Math.PI * spacing * Math.sin(steering);
    });

    document.getElementById("zoom")!.addEventListener("input", (e) => {
        zoom = Number((e.target as HTMLInputElement).value);
    });
    document.getElementById("frequency")!.addEventListener("input", (e) => {
        frequency = Number((e.target as HTMLInputElement).value) * 10e9;
    });
    document.getElementById("antennaspacing")!.addEventListener("input", (e) => {
        spacing = Number((e.target as HTMLInputElement).value);
    });
    document.getElementById("antennacount")!.addEventListener("input", (e) => {
        num_elements = Number((e.target as HTMLInputElement).value);
    });
});
