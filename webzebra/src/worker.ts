import init, {InteractionRequest, ZebraGame} from '../crate/pkg'
import {Message, MessageType, SkillSetting} from "./message";
import {checkStopToken} from "./stopToken";


let game: ZebraGame | undefined = undefined
let skills: SkillSetting = [6, 6, 6, 0, 0, 0]
let stopToken: string | undefined
let lastMessageTime = Date.now()

self.addEventListener("message", ev => {
    lastMessageTime = Date.now()
    self.postMessage([MessageType.WorkerIsRunning, true])

    const msg = ev.data as Message;
    const messageType = msg[0];
    if (msg[0] === MessageType.StopToken) {
        stopToken = msg[1]
    } else if (messageType === MessageType.GetMove) {
        if (game) {
            play_game(game, msg[1])
        }
    } else if (messageType === MessageType.GetPass) {
        if (game) {
            play_game(game, msg[1])
        }
    } else if (messageType === MessageType.NewGame) {
        if (game) {
            game.free()
        }
        game = ZebraGame.new()
        game.set_skills(...skills)
        play_game(game)
    } else if (messageType === MessageType.SetSkill) {
        skills = msg[1]
        if (game)
            game.set_skills(...skills)
    } else if (messageType === MessageType.Undo) {
        if (game) {
            game.undo()
        }
    } else {
        console.log('Unknown message')
    }

    self.postMessage([MessageType.WorkerIsRunning, false])
});

function play_game(game: ZebraGame, move?: number) {
    if (stopToken === undefined) {
        console.error('missing stop token, can\'t continue working.')
        return
    }
    // self.zebra.display_board(game.get_board())
    let request = game.play_until_next_interaction(move);
    if (request == InteractionRequest.End) {
        // just don't do anything
        // self.zebra.display_board(game.get_board())
    } else if (request == InteractionRequest.Pass) {
        self.postMessage([MessageType.GetPass])
    } else if (request == InteractionRequest.Move) {
        if (game.side_to_move() === -1) {
            self.postMessage([MessageType.GetPass])
        } else {
            self.postMessage([MessageType.GetMove])
        }
    }
}

(self as any).js_time = function js_time() {
    return Math.round(Date.now() / 1000)
};

(self as any).zebra = {
    display_board(arr: number[]) {
        // TODO this call is really expensive for some reason, investigate that.
        self.postMessage([MessageType.DisplayBoard, [...arr]])
    }
};

(self as any).send_evals  = function(evals: string) {
    self.postMessage([MessageType.Evals, evals])
};

let lastStopTokenCheck = undefined as number | undefined;

(self as any).should_stop = function() : boolean {
    if (stopToken === undefined) {
        return true
    }
    // Stop token check is expensive, so let's assume that
    // no one will interact with zebra too quickly
    // and skip the check at the begining
    if ((Date.now() - lastMessageTime) < 300) {
        return false
    }
    // Stop token check is expensive, don't check if we checked recently
    if (lastStopTokenCheck !== undefined && (Date.now() - lastStopTokenCheck) < 80) {
        return false
    }

    let shouldStop = checkStopToken(stopToken);
    lastStopTokenCheck = Date.now()
    if (shouldStop) {
        stopToken = undefined
        lastStopTokenCheck = undefined
        return true
    }
    return false
}

// FIXME is it possible to get rid of this nonsense cascade?
// I'm fighting some transpile process or something with this

init()
    .then(() => import('../crate/pkg'))
    .then(res => {
        res.initialize()

    })
    .then(() => {
        self.postMessage([MessageType.Initialized])
    })
