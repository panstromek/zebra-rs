
export const enum MessageType {
    GetMove,
    GetPass,
    DisplayBoard,
    NewGame,
    SetSkill,
    Undo,
    Evals,
    Initialized,
    StopToken,
    WorkerIsRunning
}
export type SkillSetting = [number, number, number, number, number, number]

export type Message =
    | [MessageType.GetMove, number?]
    | [MessageType.GetPass, -1?]
    | [MessageType.DisplayBoard, number[]]
    | [MessageType.NewGame]
    | [MessageType.SetSkill, SkillSetting]
    | [MessageType.Undo]
    | [MessageType.Evals, string]
    | [MessageType.Initialized]
    | [MessageType.StopToken, string]
    | [MessageType.WorkerIsRunning, boolean]

export interface EvaluatedMove {
    move: number
    best: boolean
    eval_s: string
    eval_l: string
}