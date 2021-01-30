import init, {InitOutput, set_skills, ZebraGame} from '../crate/pkg'


const identity = (a: any) => a;
let resolver = identity
let rejecter = identity


let game: ZebraGame | undefined = undefined

self.addEventListener("message", ev => {

    console.log('message recieved in worker', ev)
    const messageType = ev.data[0];
    const messageData = ev.data[1];
    if (messageType === 'get_move_from_js') {
        resolver(ev.data[1])
    } else if (messageType === 'get_pass_from_js') {
        resolver(ev.data[1])
    } else if (messageType === 'new-game') {
        rejecter(undefined);
        rejecter = identity
        if(game) {
            game.free()
        }
        game = ZebraGame.new()
    } else if (messageType === 'set-skills') {
        set_skills(...messageData)
    } else {
        console.error('rejecting promise because message from from main doesn\'t have known name.')
        rejecter(undefined)
    }
});

self.get_move_from_js = function (side_to_move : number) {
    console.log('get move from js invoked -> side_to_move', side_to_move)
    return new Promise(((resolve, reject) => {
        resolver = resolve
        rejecter = reject
        if (side_to_move === -1) {
            self.postMessage(['get_pass_from_js'])
        } else {
            self.postMessage(['get_move_from_js'])
        }
    })).then(val => {
        resolver = identity
        rejecter = identity
        return val
    })
}

self.js_time = function js_time() {
    console.log('js_time requested')
    return Math.round(Date.now() / 1000)
}

self.zebra = {
    display_board(arr) {
        self.postMessage(['display_board', arr])
    }
}



// FIXME is it possible to get rid of this nonsense cascade?
// I'm fighting some transpile process or something with this

init('../crate/pkg/webzebra_bg.wasm')
    .then(() => import('../crate/pkg'))
    .then(res => {
        res.initialize()

    })
