global.get_move_from_js = function get_move_from_js(i) {
    console.log('get move from js invoked')
    return Promise.resolve()
}

global.js_time = function js_time() {
    console.log('js_time')
    return Math.round(Date.now() / 1000)
}

global.zebra = {
    display_board(arr) {
        console.log('display board', arr)
    }
}

import("./pkg").then(async wasm => {

    wasm.greet()


    self.addEventListener("message", ev => {
        console.log(ev)
    });
});
