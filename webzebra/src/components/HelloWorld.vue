<template>
  <div class="flex flex-wrap">
    <div>
      <svg width="600" height="600" viewBox="0 0 800 800" style="background-color: green"
           @click="clickBoard">
        <g>
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
    <div class="max-w-sm text-left p-1.5">
      <button
          @click="newGame"
          class="focus:outline-none border border-solid border-black bg-green-900 text-white p-0.5 m-0.5">
        New Game
      </button>
      <br>
      <h4>Skills</h4>
      <div>
        <input v-model="black_skill" placeholder="black_skill" type="number">
        <input v-model="black_exact_skill" placeholder="black_exact_skill" type="number">
        <input v-model="black_wld_skill" placeholder="black_wld_skill" type="number">
        <br>
        <input v-model="white_skill" placeholder="white_skill" type="number">
        <input v-model="white_exact_skill" placeholder="white_exact_skill" type="number">
        <input v-model="white_wld_skill" placeholder="white_wld_skill" type="number">
        <br>
        <button @click="setSkills">Set Skills</button>
      </div>
      <div v-if="waitingForMove">Waiting for move</div>
      <div v-if="waitingForPass">Waiting for pass (click anywhere on the board)</div>
      <div class="mt-8">
        Score: <br>
        Black: {{ score.black }} <br>
        White: {{ score.white }}
      </div>
      <button @click="undo">Undo</button>
      <div>
        <input type="checkbox" id="practice_mode_checkbox" class="mr-2" v-model="practiceMode">
        <label for="practice_mode_checkbox">Practice mode</label>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'

import Worker from '../worker.ts?worker=true'
import {Message} from "../message";

export default defineComponent({
  name: 'HelloWorld',
  props: {
    msg: {
      type: String,
      required: true,
    }
  },
  data() {
    return {
      board: Array(128).fill(0)
          .map(Math.random)
          .map(a => a * 2)
          .map(Math.round),
      waitingForMove: false,
      waitingForPass: false,
      black_skill: 6,
      black_exact_skill: 6,
      black_wld_skill: 6,
      white_skill: 6,
      white_exact_skill: 6,
      white_wld_skill: 6,
      practiceMode: false,
    }
  },
  created() {
    const worker = new Worker()
    this.worker = worker
    worker.addEventListener('message', this.workerListener = ev => {
      const [type, data] = ev.data;
      switch (type) {
        case Message.DisplayBoard: {
          this.board = data
          break
        }
        case Message.GetMove : {
          this.waitingForMove = true
          break
        }
        case Message.GetPass : {
          this.waitingForPass = true
          break
        }
      }
    })
  },
  beforeUnmount() {
    this.worker.removeEventListener('message', this.workerListener)
  },
  methods: {
    undo() {
      this.worker.postMessage([Message.Undo])
    },
    setSkills() {
      let numbers = [
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
      this.worker.postMessage([Message.SetSkill, numbers])
    },
    newGame() {
      this.worker.postMessage([Message.NewGame])
    },
    clickBoard(e: MouseEvent) {
      const boardSize = 600
      const fieldSize = boardSize / 8

      if (this.waitingForPass) {
        this.worker.postMessage([Message.GetPass, -1])
        this.waitingForPass = false
      } else if (this.waitingForMove) {
        let x = e.clientX
        let y = e.clientY
        let j = Math.floor(x / fieldSize) + 1
        let i = Math.floor(y / fieldSize) + 1
        let move = (10 * i + j)
        this.worker.postMessage([Message.GetMove, move])
        this.waitingForMove = false
      }
    }
  },
  computed: {
    score() {
      return {
        // todo this is slow, use something better when we use some more efficient board representation
        white: this.circles.reduce((count, circle) => circle.color === 'white' ? count + 1 : count, 0),
        black: this.circles.reduce((count, circle) => circle.color === 'black' ? count + 1 : count, 0)
      }
    },
    circles() {
      let board = this.board;

      const arr = []
      for (let i = 1; i <= 8; i++) {
        for (let j = 1; j <= 8; j++) {
          let color;
          switch (board[(10 * i + j)]) {
            case 0  :
              color = 'black'
              break;

            case 2 :
              color = 'white'
              break;

            default :
              continue

          }

          const fieldSize = 100
          const pieceSize = 80
          arr.push({
            cx: (j - 1) * fieldSize + 0.5 * fieldSize,
            cy: (i - 1) * fieldSize + 0.5 * fieldSize,
            r: pieceSize / 2,
            color
          })
        }
      }
      return arr
    }
  }
})
</script>

<style scoped>
a {
  color: #42b983;
}
</style>
