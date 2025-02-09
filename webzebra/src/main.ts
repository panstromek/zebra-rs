import {reactive, watch} from '@vue/reactivity'
import './index.css'
import {EvaluatedMove, Message, MessageType, SkillSetting} from "./message";
import {createStopToken, stop} from "./stopToken";
import {boardData, scoreFromCircles} from "./game";

interface ZWorker extends Worker {
    postMessage(message: Message): void
}

const worker = new Worker(new URL('./worker.js', import.meta.url), {type: 'module'}) as ZWorker

const data = reactive({
    board: Array(128).fill(1) as number[],
    evals: [] as EvaluatedMove[],
    clickedMove: undefined as number | undefined,
})
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
        data.clickedMove = move
        worker.postMessage([MessageType.GetMove, move])
        waitingForMove = false
    }
}

worker.addEventListener('message', ev => {
    const [type, dataFromWorker] = (ev as MessageEvent).data as Message;
    switch (type) {
        case MessageType.DisplayBoard: {
            data.board = dataFromWorker
            data.clickedMove = undefined
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
            data.evals = JSON.parse(dataFromWorker).evals
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
    const board = data.board;
    const evaluatedMoves = data.evals;
    const clickedMove = data.clickedMove;
    const svgData_ = boardData(board, clickedMove, evaluatedMoves);

    const score = scoreFromCircles(svgData_.circles)
    const circles = svgData_.circles;
    const circlesHtml = circles.map(circle => {
        return `<circle r="${circle.r}" cx="${circle.cx}" cy="${circle.cy}" style="fill: ${circle.color}"></circle>`
    }).join('')

    document.getElementById('circles')!.innerHTML = circlesHtml
    document.getElementById('score-black')!.innerText = '' + score.black
    document.getElementById('score-white')!.innerText = '' + score.white

    const evals = svgData_.evals;
    const evalsHtml = evals.map(eval_ => {
        return `<text x="${eval_.x}" y="${eval_.y}" style="fill: ${eval_.color}; font-size: 50px">${eval_.text}</text>`
    }).join('')

    document.getElementById('evals')!.innerHTML = evalsHtml
}

watch(data, render)
