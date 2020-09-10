const worker = new Worker("./worker.js");

worker.addEventListener("message", ev => {
    console.log(ev)
});
