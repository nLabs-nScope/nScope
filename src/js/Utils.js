export function getId(id) {
    return document.getElementById(id);
}

export function isEmpty(obj) {
    return obj && Object.keys(obj).length === 0 && Object.getPrototypeOf(obj) === Object.prototype;
}

export function idFromCh(chString) {
    return parseInt(chString.slice(-1));
}

export function clamp(x, lower, upper) {
    return Math.min(Math.max(x, lower), upper);
}