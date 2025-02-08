import {computed, reactive, watch} from '@vue/reactivity'
import './index.css'
// @ts-ignore
import ZebraWorker from './worker.ts?worker=true'
import {EvaluatedMove, Message, MessageType, SkillSetting} from "./message";
import {createStopToken, stop} from "./stopToken";
import {boardData, scoreFromCircles} from "./game";
// we have to reference the wasm url here, otherwise wasm doesn't get bundled for some reason.
// If we reference it only from worker, it's missing in production build.
// FIXME report this as a bug in Vite
import wasm_path from '../crate/pkg/webzebra_bg.wasm?url'
// log, because bundler would eliminate the import if we didn't use it
console.log('wasm path: ' + wasm_path)

interface ZWorker extends Worker {
    postMessage(message: Message): void
}

const worker = new ZebraWorker() as ZWorker

const data = reactive({
    board: Array(128).fill(1) as number[],
    waitingForMove: false,
    waitingForPass: false,
    black_skill: 0,
    black_exact_skill: 0,
    black_wld_skill: 0,
    white_skill: 0,
    white_exact_skill: 0,
    white_wld_skill: 0,
    practiceMode: true,
    evals: [] as EvaluatedMove[],
    initialized: false,
    stopToken: undefined as string | undefined,
    workerIsRunning: false,
    clickedMove: undefined as number | undefined,
})

const stopWorkerIfNeeded = () => {
    if (data.workerIsRunning) {
        if (data.stopToken) {
            stop(data.stopToken)
        } else {
            console.error('cannot stop worker, missing stop token')
        }

        data.stopToken = createStopToken()
        worker.postMessage([MessageType.StopToken, data.stopToken])
    }
};

function undo() {
    stopWorkerIfNeeded()
    worker.postMessage([MessageType.Undo])
}

function setSkills() {
    stopWorkerIfNeeded()

    let numbers: SkillSetting = [
        Number(data.black_skill),
        Number(data.black_exact_skill),
        Number(data.black_wld_skill),
        Number(data.white_skill),
        Number(data.white_exact_skill),
        Number(data.white_wld_skill)
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

    if (data.waitingForPass) {
        worker.postMessage([MessageType.GetPass, -1])
        data.waitingForPass = false
    } else {
        let x = e.offsetX
        let y = e.offsetY
        let j = Math.floor(x / fieldSize) + 1
        let i = Math.floor(y / fieldSize) + 1
        let move = (10 * i + j)
        data.clickedMove = move
        worker.postMessage([MessageType.GetMove, move])
        data.waitingForMove = false
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
            data.waitingForMove = true
            break
        }
        case MessageType.GetPass : {
            data.waitingForPass = true
            break
        }
        case MessageType.Evals: {
            data.evals = JSON.parse(dataFromWorker).evals
            break;
        }
        case MessageType.Initialized: {
            data.initialized = true
            break;
        }
        case MessageType.WorkerIsRunning : {
            data.workerIsRunning = dataFromWorker
            break
        }
    }
})

data.stopToken = createStopToken()
worker.postMessage([MessageType.StopToken, data.stopToken])
// @click.prevent.stop="clickBoard"
document.getElementById('board')?.addEventListener('click', (e) => {
    e.preventDefault()
    e.stopPropagation()
    clickBoard(e)
})
document.getElementById('new_game')?.addEventListener('click', (e) => {
    newGame()
})

const svgData = computed(() => {
    const board = data.board;
    const evaluatedMoves = data.evals;
    const clickedMove = data.clickedMove;
    return boardData(board, clickedMove, evaluatedMoves);
})

watch(svgData, (svgData_: typeof svgData.value) => {
    const circles = svgData_.circles;
    const circlesHtml = circles.map(circle => {
        return `<circle r="${circle.r}" cx="${circle.cx}" cy="${circle.cy}" style="fill: ${circle.color}"></circle>`
    }).join('')

    document.getElementById('circles')!.innerHTML = circlesHtml
})

watch([svgData, () => data.practiceMode], ([svgData_, practiceMode]: [typeof svgData.value, boolean])=> {
    console.log('practice mode changed', svgData_, practiceMode)
    if (!practiceMode) {
        return ''
    }
    const evals = svgData_.evals;
    const evalsHtml = evals.map(eval_ => {
        return `<text x="${eval_.x}" y="${eval_.y}" style="fill: ${eval_.color}; font-size: 50px">${eval_.text}</text>`
    }).join('')

    document.getElementById('evals')!.innerHTML = evalsHtml
})

watch(svgData, (svgData_: typeof svgData.value) => {
    const score = scoreFromCircles(svgData_.circles)
    document.getElementById('score-black')!.innerText = '' + score.black
    document.getElementById('score-white')!.innerText = '' + score.white
})
