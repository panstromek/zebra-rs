/*
* This is a simple hacky utility that allows you to stop synchronous code in a worker
* from another thread without Atomics and other unstable APIs. The code in the worker just
* has to repeatedly call a check function until it returns true. No async `onmessage` handler needed.
*
* Usage:
*  1. use createStopToken to create stop token
*  2. give the stop token to the worker that you want to stop at some point
*  3. repeatedly call checkStopToken in the worker thread until it returns true
*  4. When you want to stop the worker, call stop function with the same token
*       This destroys the token and cause the checkStopToken function in the worker to return true
*
* */


export function createStopToken(): string {
    return URL.createObjectURL(new Blob());
}

export function stop(stopToken: string): void {
    return URL.revokeObjectURL(stopToken)
}

export function checkStopToken(stopToken: string): boolean {
    let xhr = new XMLHttpRequest();
    xhr.open("GET", stopToken, false);
    try {
        xhr.send(null);
    } catch (e) {
        return true
    }
    return false;
}