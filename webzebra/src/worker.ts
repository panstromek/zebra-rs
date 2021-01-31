import init, {InitOutput, InteractionRequest, set_skills, ZebraGame} from '../crate/pkg'


let game: ZebraGame | undefined = undefined

self.addEventListener("message", ev => {

    console.log('message recieved in worker')
    const messageType = ev.data[0];
    const messageData = ev.data[1];
    if (messageType === 'get_move_from_js') {
        if (game) {
            play_game(game, ev.data[1])
        }
    } else if (messageType === 'get_pass_from_js') {
        if (game) {
            play_game(game, ev.data[1])
        }
    } else if (messageType === 'new-game') {
        if (game) {
            game.free()
        }
        game = ZebraGame.new()
        play_game(game)
    } else if (messageType === 'set-skills') {
        set_skills(...messageData)
    } else {}
});

function play_game(game: ZebraGame, move?: number) {
    // self.zebra.display_board(game.get_board())
    let request = game.play_until_next_interaction(move);
    if (request == InteractionRequest.End) {
        // just don't do anything
        // self.zebra.display_board(game.get_board())
    } else if (request == InteractionRequest.Pass) {
        self.postMessage(['get_pass_from_js'])
    } else if (request == InteractionRequest.Move) {
        get_move_from_js(game.side_to_move())
    }
}

function get_move_from_js(side_to_move: number) {
    console.log('get move from js invoked -> side_to_move', side_to_move)
    if (side_to_move === -1) {
        self.postMessage(['get_pass_from_js'])
    } else {
        self.postMessage(['get_move_from_js'])
    }
    return
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
