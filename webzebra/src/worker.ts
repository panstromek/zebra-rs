import init, {InteractionRequest, ZebraGame} from '../crate/pkg'
import {Message} from "./message";


let game: ZebraGame | undefined = undefined
let skills = [6, 6, 6, 0, 0, 0]

self.addEventListener("message", ev => {
    const messageType = ev.data[0];
    const messageData = ev.data[1];
    if (messageType === Message.GetMove) {
        if (game) {
            play_game(game, ev.data[1])
        }
    } else if (messageType === Message.GetPass) {
        if (game) {
            play_game(game, ev.data[1])
        }
    } else if (messageType === Message.NewGame) {
        if (game) {
            game.free()
        }
        game = ZebraGame.new()
        game.set_skills(...skills)
        play_game(game)
    } else if (messageType === Message.SetSkill) {
        skills = messageData
        if (game)
            game.set_skills(...messageData)
    } else if (messageType === Message.Undo) {
        if (game) {
            game.undo()
        }
    } else {
        console.log('Unknown message')
    }
});

function play_game(game: ZebraGame, move?: number) {
    // self.zebra.display_board(game.get_board())
    let request = game.play_until_next_interaction(move);
    if (request == InteractionRequest.End) {
        // just don't do anything
        // self.zebra.display_board(game.get_board())
    } else if (request == InteractionRequest.Pass) {
        self.postMessage([Message.GetPass])
    } else if (request == InteractionRequest.Move) {
        if (game.side_to_move() === -1) {
            self.postMessage([Message.GetPass])
        } else {
            self.postMessage([Message.GetMove])
        }
    }
}

self.js_time = function js_time() {
    return Math.round(Date.now() / 1000)
}

self.zebra = {
    display_board(arr) {
        // TODO this call is really expensive for some reason, investigate that.
        self.postMessage([Message.DisplayBoard, [...arr]])
    }
}


// FIXME is it possible to get rid of this nonsense cascade?
// I'm fighting some transpile process or something with this

init('../crate/pkg/webzebra_bg.wasm')
    .then(() => import('../crate/pkg'))
    .then(res => {
        res.initialize()

    })
