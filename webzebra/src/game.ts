export type Circle = {
    cx: number,
    cy: number,
    r: number,
    color: string
}

export const scoreFromCircles = (circles: Circle[]) => {
    return {
        // todo this is slow, use something better when we use some more efficient board representation
        white: circles.reduce((count: number, circle: Circle) => circle.color === 'white' ? count + 1 : count, 0),
        black: circles.reduce((count: number, circle: Circle) => circle.color === 'black' ? count + 1 : count, 0)
    }
}
