const worker = new Worker("./worker.js");

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById('board-canvas');
const ctx = canvas.getContext('2d')

function drawBoard(board = []) {
    ctx.fillStyle = 'green'
    const boardSize = Math.min(canvas.width, canvas.height)
    ctx.fillRect(0, 0, canvas.width, canvas.height)
    const fieldSize = boardSize / 8
    ctx.fillStyle = 'white'
    for (let i = 1; i < 8; i++) {
        ctx.fillRect(i * fieldSize, 0, 1, canvas.height)
        ctx.fillRect(0, i * fieldSize, canvas.width, 1)
    }
    const pieceSize = fieldSize * 0.8;

    for (let i = 1; i <= 8; i++) {
        for (let j = 1; j <= 8; j++) {
            switch (board[(10 * i + j)]) {
                case 0  : {
                    ctx.fillStyle = 'black'
                    break;
                }
                case 2 : {
                    ctx.fillStyle = 'white'
                    break;
                }
                default : {
                    continue
                }
            }

            ctx.beginPath()
            ctx.arc(
                (j - 1) * fieldSize + 0.5 * fieldSize,
                (i - 1) * fieldSize + 0.5 * fieldSize,
                pieceSize / 2,
                0, 2 * Math.PI, false);
            ctx.fill()
        }
    }
}

worker.addEventListener("message", ev => {
    const [type, data] = ev.data;
    switch (type) {
        case 'display_board': {
            drawBoard(data)
        }
    }
    console.log(ev)
});
