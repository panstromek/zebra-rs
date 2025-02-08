import {createApp, reactive} from 'vue'
import './index.css'
import {defineComponent} from 'vue'
import ZebraWorker from './worker.ts?worker=true'
import {EvaluatedMove, Message, MessageType, SkillSetting} from "./message";
import {createStopToken, stop} from "./stopToken";
import {boardData, Circle, Eval, scoreFromCircles} from "./game";
// we have to reference the wasm url here, otherwise wasm doesn't get bundled for some reason.
// If we reference it only from worker, it's missing in production build.
// FIXME report this as a bug in Vite
import wasm_path from '../crate/pkg/webzebra_bg.wasm?url'
// log, because bundler would eliminate the import if we didn't use it
console.log('wasm path: ' + wasm_path)

interface ZWorker extends Worker {
    postMessage(message: Message): void
}

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

    // workardound for analysis not working properly
    // initialized in created hook
    worker: undefined as any as ZWorker,
    workerListener: undefined as any as (this: Worker, ev: WorkerEventMap[keyof WorkerEventMap]) => any
})

const App = defineComponent({
    name: 'HelloWorld',
    data() {
        return data
    },
    created() {
        const worker = new ZebraWorker() as ZWorker
        data.worker = worker
        worker.addEventListener('message', data.workerListener = ev => {
            const [type, data] = (ev as MessageEvent).data as Message;
            switch (type) {
                case MessageType.DisplayBoard: {
                    this.board = data
                    this.clickedMove = undefined
                    break
                }
                case MessageType.GetMove : {
                    this.waitingForMove = true
                    break
                }
                case MessageType.GetPass : {
                    this.waitingForPass = true
                    break
                }
                case MessageType.Evals: {
                    this.evals = JSON.parse(data).evals
                    break;
                }
                case MessageType.Initialized: {
                    this.initialized = true
                    break;
                }
                case MessageType.WorkerIsRunning : {
                    this.workerIsRunning = data
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
            this.clickBoard(e)
        })
        document.getElementById('new_game')?.addEventListener('click', (e) => {
            this.newGame()
        })
    },
    beforeUnmount() {
        data.worker.removeEventListener('message', data.workerListener)
    },
    methods: {
        undo() {
            this.stopWorkerIfNeeded()
            data.worker.postMessage([MessageType.Undo])
        },
        setSkills() {
            this.stopWorkerIfNeeded()

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
            this.worker.postMessage([MessageType.SetSkill, numbers])
        },
        newGame() {
            this.stopWorkerIfNeeded()
            this.setSkills()
            data.worker.postMessage([MessageType.NewGame])
        },
        stopWorkerIfNeeded() {
            if (data.workerIsRunning) {
                if (data.stopToken) {
                    stop(data.stopToken)
                } else {
                    console.error('cannot stop worker, missing stop token')
                }

                data.stopToken = createStopToken()
                this.worker.postMessage([MessageType.StopToken, data.stopToken])
            }
        },
        clickBoard(e: MouseEvent) {
            this.stopWorkerIfNeeded()
            const board = document.getElementById('board') as unknown as SVGElement;
            const boardSize = board.clientWidth
            const fieldSize = boardSize / 8

            if (this.waitingForPass) {
                this.worker.postMessage([MessageType.GetPass, -1])
                this.waitingForPass = false
            } else {
                let x = e.offsetX
                let y = e.offsetY
                let j = Math.floor(x / fieldSize) + 1
                let i = Math.floor(y / fieldSize) + 1
                let move = (10 * i + j)
                this.clickedMove = move
                this.worker.postMessage([MessageType.GetMove, move])
                this.waitingForMove = false
            }
        }
    },
    computed: {
        score(): { white: number, black: number } {
            const circles = this.circles;
            return scoreFromCircles(circles);
        },
        circles(): Circle[] {
            return this.svg_data.circles
        },
        svg_data(): { circles: Circle[], evals: Eval[] } {
            const board = this.board;
            const evaluatedMoves = this.evals;
            const clickedMove = this.clickedMove;
            return boardData(board, clickedMove, evaluatedMoves);
        },
        circlesHtml(): string {
            const circles = this.circles;
            return circles.map(circle => {
                return `<circle r="${circle.r}" cx="${circle.cx}" cy="${circle.cy}" style="fill: ${circle.color}"></circle>`
            }).join('')
        },
        evalsHtml(): string {
            if (!this.practiceMode) {
                return ''
            }
            const evals = this.svg_data.evals;
            return evals.map(eval_ => {
                return `<text x="${eval_.x}" y="${eval_.y}" style="fill: ${eval_.color}; font-size: 50px">${eval_.text}</text>`
            }).join('')
        }
    },
    watch: {
        circlesHtml() {
            document.getElementById('circles')!.innerHTML = this.circlesHtml
        },
        evalsHtml() {
            document.getElementById('evals')!.innerHTML = this.evalsHtml
        },
        score(score) {
            document.getElementById('score-black')!.innerText = score.black
            document.getElementById('score-white')!.innerText = score.white
        }
    }
})

createApp(App).mount('#app')
