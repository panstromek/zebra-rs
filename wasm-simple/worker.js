const identity = _ => _;
let resolver = identity
let rejecter = identity


self.addEventListener("message", ev => {
    if (ev.data[1] === 'get_move_from_js') {
        resolver(ev.data[2])
    } else {
        rejecter()
    }
});

global.get_move_from_js = function (side_to_move) {
    console.log('get move from js invoked -> side_to_move', side_to_move)
    return new Promise(((resolve, reject) => {
        resolver = resolve
        rejecter = reject
        postMessage(['get_move_from_js'])
    })).then(() => {
        resolver = identity
        rejecter = identity
    })

}

global.js_time = function js_time() {
    console.log('js_time requested')
    return Math.round(Date.now() / 1000)
}

global.zebra = {
    display_board(arr) {
        postMessage(['display_board', arr])
    }
}

import("./pkg").then(async wasm => {
    wasm.greet()
});
