<template>
  <div>
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
    <div>
      <button @click="newGame">New Game</button>
      <br>
      <h4>Skills</h4>
      <div>
        <input placeholder="black_skill" type="text">
        <input placeholder="black_exact_skill" type="text">
        <input placeholder="black_wld_skill" type="text">
        <br>
        <input placeholder="white_skill" type="text">
        <input placeholder="white_exact_skill" type="text">
        <input placeholder="white_wld_skill" type="text">
        <br>
        <button @click="setSkills">Set Skills</button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {ref, defineComponent, reactive} from 'vue'

import Worker from '../worker.ts?worker=true'

const worker = new Worker()

//todo
// document.getElementById('set-skills').addEventListener('click', set_skills)
//
// function set_skills() {
//   let numbers = [
//     Number(document.getElementById('black_skill').value),
//     Number(document.getElementById('black_exact_skill').value),
//     Number(document.getElementById('black_wld_skill').value),
//     Number(document.getElementById('white_skill').value),
//     Number(document.getElementById('white_exact_skill').value),
//     Number(document.getElementById('white_wld_skill').value)
//   ];
//   if (numbers.some(num => isNaN(num) && !Number.isInteger(num))) {
//     alert('Some values are not integers')
//     return
//   }
//   worker.postMessage(['set-skills', numbers])
// }



export default defineComponent({
  name: 'HelloWorld',
  props: {
    msg: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      board: Array(128).fill(0)
          .map(Math.random)
          .map(a => a * 2)
          .map(Math.round),
      waitingForMove: false,
      waitingForPass : false,
    }
  },
  created() {
    worker.addEventListener("message", this.workerListener = ev => {
      const [type, data] = ev.data;
      console.log(ev)
      switch (type) {
        case 'display_board': {
          this.board = data
          break
        }
        case 'get_move_from_js' : {
          console.log('waiting for move')
          this.waitingForMove = true
          break
        }
        case 'get_pass_from_js' : {
          console.log('waiting for pass')
          this.waitingForPass = true
          break
        }
      }
    })
  },
  beforeUnmount() {
    worker.removeEventListener('message', this.workerListener)
  },
  methods: {
    setSkills() {
      //todo
    },
    newGame() {
      worker.postMessage(['new-game'])
    },
    clickBoard(e: MouseEvent) {
      const boardSize = 600
      const fieldSize = boardSize / 8

      if (waitingForPass) {
        worker.postMessage(['get_pass_from_js', -1])
        this.waitingForPass = false
      } else if (waitingForMove) {
        let x = e.clientX
        let y = e.clientY
        let j = Math.floor(x / fieldSize) + 1
        let i = Math.floor(y / fieldSize) + 1
        let move = (10 * i + j)
        console.log(x, y, i, j)
        worker.postMessage(['get_move_from_js', move])
        this.waitingForMove = false
      }
    }
  },
  computed: {
    circles() {
      const arr = []
      for (let i = 1; i <= 8; i++) {
        for (let j = 1; j <= 8; j++) {
          let color;
          switch (this.board[(10 * i + j)]) {
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
