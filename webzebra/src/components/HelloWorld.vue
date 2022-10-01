<template>
  <div class="flex flex-wrap justify-center">
    <div style="max-width: 750px; width: 100%">
      <svg width="100%"
           preserveAspectRatio="xMidYMid meet"
           ref="board"
           viewBox="0 0 800 800"
           style="background-color: #3a7f46; border: 1vw brown solid;"
           @click.prevent.stop="clickBoard">
        <g>
          <template v-if="practiceMode">
            <text v-for="eval_ in svg_data.evals"
                  :x="eval_.x"
                  :y="eval_.y"
                  :style="{'fill': eval_.color, 'font-size' : '50px'}">
              {{eval_.text}}
            </text>
          </template>
          <circle
              v-for="circle in circles"
              :r="circle.r"
              :cx="circle.cx"
              :cy="circle.cy"
              :style="{'fill': circle.color}"></circle>
          <line
              v-for="i in 7"
              x1="0"
              :y1="i * 100"
              x2="800"
              :y2="i*100"
              stroke="black"
          ></line>
          <line
              v-for="i in 7"
              y1="0"
              :x1="i * 100"
              y2="800"
              :x2="i*100"
              stroke="black"
          ></line>
        </g>
      </svg>
    </div>
    <div class="max-w-sm text-left p-1.5 w-full mt-3">
      <div class="flex justify-between items-center">
        <div class="flex items-center text-black text-2xl">
          <div style="width: 1em; height: 1em; border-radius: 0.5em; background: black"></div>
          <div class="px-2">{{ score.black }}</div>
          <div style="width: 1em; height: 1em; border-radius: 0.5em; background: white"></div>
          <div class="px-2 text-white">{{ score.white }}</div>
        </div>
        <button
            :disabled="!initialized"
            @click="newGame"
            :class="initialized ? 'border-black' : 'text-gray-700 border-gray-700'"
            class="focus:outline-none border-2 rounded border-solid text-white p-2 m-0.5">
          New Game
        </button>
      </div>
<!--      <div>-->
<!--        Worker running: {{workerIsRunning}}-->
<!--      </div>-->
<!--      <br>-->
<!--      <h4>Skills</h4>-->
<!--      <div>-->
<!--        <input v-model="black_skill" placeholder="black_skill" type="number">-->
<!--        <input v-model="black_exact_skill" placeholder="black_exact_skill" type="number">-->
<!--        <input v-model="black_wld_skill" placeholder="black_wld_skill" type="number">-->
<!--        <br>-->
<!--        <input v-model="white_skill" placeholder="white_skill" type="number">-->
<!--        <input v-model="white_exact_skill" placeholder="white_exact_skill" type="number">-->
<!--        <input v-model="white_wld_skill" placeholder="white_wld_skill" type="number">-->
<!--        <br>-->
<!--        <button @click="setSkills">Set Skills</button>-->
<!--      </div>-->
<!--      <div v-if="waitingForMove">Waiting for move</div>-->
<!--      <div v-if="waitingForPass">Waiting for pass (click anywhere on the board)</div>-->
<!--      <button @click="undo">Undo</button>-->
<!--      <div>-->
<!--        <input type="checkbox" id="practice_mode_checkbox" class="mr-2" v-model="practiceMode">-->
<!--        <label for="practice_mode_checkbox">Practice mode</label>-->
<!--      </div>-->
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'

import ZebraWorker from '../worker.ts?worker=true'
import {EvaluatedMove, Message, MessageType, SkillSetting} from "../message";
import {createStopToken, stop} from "../stopToken";

interface ZWorker extends Worker {
  postMessage(message: Message): void
}

type NonReactiveData = {
  worker: ZWorker,
  workerListener: (this: Worker, ev: MessageEvent) => any
}
type Circle = {
  cx: number,
  cy: number,
  r: number,
  color: string
}
type Eval = {
  x: number,
  y: number,
  color: string,
  text: string
}

function boardData(board: number[], clickedMove: number | undefined, evaluatedMoves: EvaluatedMove[]) {
  let evals = []
  const fieldSize = 100
  const arr = [] as Circle[]
  for (let i = 1; i <= 8; i++) {
    for (let j = 1; j <= 8; j++) {
      let color;
      let move = 10 * i + j;

      switch (board[move]) {
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
          const eval_ = evaluatedMoves.find((eval_: EvaluatedMove) => eval_.move === move)
          if (!eval_) {
            continue
          }

          if (eval_.best) {
            color = '#00FFFF'
          } else {
            color = '#FFFF00'
          }
          const text = eval_.eval_s
          evals.push({
            x: (j - 1) * fieldSize + 0.2 * fieldSize,
            y: (i - 1) * fieldSize + 0.65 * fieldSize,
            color,
            text
          })
          continue
        }
      }

      const pieceSize = 80
      arr.push({
        cx: (j - 1) * fieldSize + 0.5 * fieldSize,
        cy: (i - 1) * fieldSize + 0.5 * fieldSize,
        r: pieceSize / 2,
        color
      })
    }
  }
  return {
    circles: arr,
    evals
  }
}

export default defineComponent({
  name: 'HelloWorld',
  data() {
    const board = Array(128).fill(1) as number[];
    // board[54] = 0;
    // board[45] = 0;
    // board[55] = 2;
    // board[44] = 2;
    return {
      board: board,
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
    }
  },
  created() {
    const worker = new ZebraWorker() as ZWorker
    this.worker = worker
    worker.addEventListener('message', this.workerListener = ev => {
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

    this.stopToken = createStopToken()
    worker.postMessage([MessageType.StopToken, this.stopToken])
  },
  beforeUnmount() {
    this.worker.removeEventListener('message', this.workerListener)
  },
  methods: {
    undo() {
      this.stopWorkerIfNeeded()
      this.worker.postMessage([MessageType.Undo])
    },
    setSkills() {
      this.stopWorkerIfNeeded()

      let numbers: SkillSetting = [
        Number(this.black_skill),
        Number(this.black_exact_skill),
        Number(this.black_wld_skill),
        Number(this.white_skill),
        Number(this.white_exact_skill),
        Number(this.white_wld_skill)
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
      this.worker.postMessage([MessageType.NewGame])
    },
    stopWorkerIfNeeded() {
      if (this.workerIsRunning) {
        if (this.stopToken) {
          stop(this.stopToken)
        } else {
          console.error('cannot stop worker, missing stop token')
        }

        this.stopToken = createStopToken()
        this.worker.postMessage([MessageType.StopToken, this.stopToken])
      }
    },
    clickBoard(e: MouseEvent) {
      this.stopWorkerIfNeeded()
      const board = this.$refs.board as SVGElement;
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
      return {
        // todo this is slow, use something better when we use some more efficient board representation
        white: this.circles.reduce((count: number, circle: Circle) => circle.color === 'white' ? count + 1 : count, 0),
        black: this.circles.reduce((count: number, circle: Circle) => circle.color === 'black' ? count + 1 : count, 0)
      }
    },
    circles(): Circle[] {
      return this.svg_data.circles
    },
    svg_data(): { circles: Circle[], evals: Eval[] } {
      const board = this.board;
      const evaluatedMoves = this.evals;
      const clickedMove = this.clickedMove;
      return boardData(board, clickedMove, evaluatedMoves);
    }
  }
})
</script>

<style>
a {
  color: #42b983;
}

/* Avoid text selection on the board */
svg text {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

svg text::selection {
  background: none;
}
</style>
