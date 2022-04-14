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
    console.log('checking stop token') // todo remove in prod?
    let xhr = new XMLHttpRequest();
    let res = false
    xhr.open("GET", stopToken, false);
    xhr.onload = function (e) {
        if (xhr.readyState === 4 && xhr.status !== 200) {
            res = true
        }
    };
    xhr.onerror = function (e) {
        console.error(xhr.statusText);
    };
    try {
        xhr.send(null);
    } catch (e) {
        return true
    }
    return res;
}