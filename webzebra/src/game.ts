import {EvaluatedMove} from "./message";

export type Circle = {
    cx: number,
    cy: number,
    r: number,
    color: string
}
export type Eval = {
    x: number,
    y: number,
    color: string,
    text: string
}

export function boardData(board: number[], clickedMove: number | undefined, evaluatedMoves: EvaluatedMove[]) {
    let evals_ = []
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
        evals: evals_
    }
}

export const scoreFromCircles = (circles: Circle[]) => {
    return {
        // todo this is slow, use something better when we use some more efficient board representation
        white: circles.reduce((count: number, circle: Circle) => circle.color === 'white' ? count + 1 : count, 0),
        black: circles.reduce((count: number, circle: Circle) => circle.color === 'black' ? count + 1 : count, 0)
    }
}
