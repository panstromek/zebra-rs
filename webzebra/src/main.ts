import './index.css'
import {EvaluatedMove, Message, MessageType, SkillSetting} from "./message";
import {createStopToken, stop} from "./stopToken";
import {Circle} from "./game";

interface ZWorker extends Worker {
    postMessage(message: Message): void
}

const worker = new Worker(new URL('./worker.js', import.meta.url), {type: 'module'}) as ZWorker

let board = Array(128).fill(1) as number[]
let evals = [] as EvaluatedMove[]
let clickedMove = undefined as number | undefined

let waitingForMove = false;
let waitingForPass = false;
let workerIsRunning = false;
let stopToken = undefined as string | undefined;

const stopWorkerIfNeeded = () => {
    if (workerIsRunning) {
        if (stopToken) {
            stop(stopToken)
        } else {
            console.error('cannot stop worker, missing stop token')
        }

        stopToken = createStopToken()
        worker.postMessage([MessageType.StopToken, stopToken])
    }
};

function undo() {
    stopWorkerIfNeeded()
    worker.postMessage([MessageType.Undo])
}

function setSkills() {
    stopWorkerIfNeeded()

    const black_skill = 0,
        black_exact_skill = 0,
        black_wld_skill = 0,
        white_skill = 0,
        white_exact_skill = 0,
        white_wld_skill = 0;

    let numbers: SkillSetting = [
        black_skill,
        black_exact_skill,
        black_wld_skill,
        white_skill,
        white_exact_skill,
        white_wld_skill
    ];
    if (numbers.some(num => isNaN(num) && !Number.isInteger(num))) {
        alert('Some values are not integers')
        return
    }
    worker.postMessage([MessageType.SetSkill, numbers])
}

function newGame() {
    stopWorkerIfNeeded()
    setSkills()
    worker.postMessage([MessageType.NewGame])
}

function clickBoard(e: MouseEvent) {
    stopWorkerIfNeeded()
    const board = document.getElementById('board') as unknown as SVGElement;
    const boardSize = board.clientWidth
    const fieldSize = boardSize / 8

    if (waitingForPass) {
        worker.postMessage([MessageType.GetPass, -1])
        waitingForPass = false
    } else {
        let x = e.offsetX
        let y = e.offsetY
        let j = Math.floor(x / fieldSize) + 1
        let i = Math.floor(y / fieldSize) + 1
        let move = (10 * i + j)
        clickedMove = move
        worker.postMessage([MessageType.GetMove, move])
        waitingForMove = false
        render()
    }
}

worker.addEventListener('message', ev => {
    const [type, dataFromWorker] = (ev as MessageEvent).data as Message;
    switch (type) {
        case MessageType.DisplayBoard: {
            board = dataFromWorker
            clickedMove = undefined
            render()
            break
        }
        case MessageType.GetMove : {
            waitingForMove = true
            break
        }
        case MessageType.GetPass : {
            waitingForPass = true
            break
        }
        case MessageType.Evals: {
            evals = JSON.parse(dataFromWorker).evals
            render()
            break;
        }
        case MessageType.Initialized: {
            const newGameButton = document.getElementById('new_game');
            newGameButton?.removeAttribute('disabled')
            newGameButton?.addEventListener('click', (e) => {
                newGame()
            })
            break;
        }
        case MessageType.WorkerIsRunning : {
            workerIsRunning = dataFromWorker
            break
        }
    }
})

stopToken = createStopToken()
worker.postMessage([MessageType.StopToken, stopToken])

document.getElementById('board')?.addEventListener('click', (e) => {
    e.preventDefault()
    e.stopPropagation()
    clickBoard(e)
})

function render() {
    let evals_ = []
    const fieldSize = 100
    const circles = [] as Circle[]
    for (let i = 1; i <= 8; i++) {
        for (let j = 1; j <= 8; j++) {
            let color;
            let move = 10 * i + j;

            const piece = board[move];

            switch (piece) {
                case 0  :
                    color = 'black'
                    break;

                case 2 :
                    color = 'white'
                    break;

                default : {
                    if (clickedMove === move) {
                        // todo take into account side to move
                        color = 'rgba(127, 127, 127, 0.5)'
                        break
                    }
                    const eval_ = evals.find((eval_: EvaluatedMove) => eval_.move === move)
                    if (!eval_) {
                        continue
                    }

                    if (eval_.best) {
                        color = '#00FFFF'
                    } else {
                        color = '#FFFF00'
                    }
                    const text = eval_.eval_s
                    evals_.push({
                        x: (j - 1) * fieldSize + 0.2 * fieldSize,
                        y: (i - 1) * fieldSize + 0.65 * fieldSize,
                        color,
                        text
                    })
                    continue
                }
            }

            const pieceSize = 80
            circles.push({
                cx: (j - 1) * fieldSize + 0.5 * fieldSize,
                cy: (i - 1) * fieldSize + 0.5 * fieldSize,
                r: pieceSize / 2,
                color
            })
        }
    }

    // todo this is slow, use something better when we use some more efficient board representation
    const score_white = circles.reduce((count: number, circle: Circle) => circle.color === 'white' ? count + 1 : count, 0)
    const score_black = circles.reduce((count: number, circle: Circle) => circle.color === 'black' ? count + 1 : count, 0)

    const circlesHtml = circles.map(circle => {
        return `<circle r="${circle.r}" cx="${circle.cx}" cy="${circle.cy}" style="fill: ${circle.color}"></circle>`
    }).join('')

    document.getElementById('circles')!.innerHTML = circlesHtml
    document.getElementById('score-black')!.innerText = '' + score_black
    document.getElementById('score-white')!.innerText = '' + score_white

    const evalsHtml = evals_.map(eval_ => {
        return `<text x="${eval_.x}" y="${eval_.y}" style="fill: ${eval_.color}; font-size: 50px">${eval_.text}</text>`
    }).join('')

    document.getElementById('evals')!.innerHTML = evalsHtml
}

