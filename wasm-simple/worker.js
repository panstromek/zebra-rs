const identity = _ => _;
let resolver = identity
let rejecter = identity


self.addEventListener("message", ev => {
    console.log('message recieved in worker', ev)
    if (ev.data[0] === 'get_move_from_js') {
        resolver(ev.data[1])
    } else if (ev.data[0] === 'get_pass_from_js') {
        resolver(ev.data[1])
    } else {
        console.error('rejecting promise because message from from main doesn\'t have known name.')
        rejecter()
    }
});

global.get_move_from_js = function (side_to_move) {
    console.log('get move from js invoked -> side_to_move', side_to_move)
    return new Promise(((resolve, reject) => {
        resolver = resolve
        rejecter = reject
        if (side_to_move === -1) {
            postMessage(['get_pass_from_js'])
        } else {
            postMessage(['get_move_from_js'])
        }
    })).then(val => {
        resolver = identity
        rejecter = identity
        return val
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
