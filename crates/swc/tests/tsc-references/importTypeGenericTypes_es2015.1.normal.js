// @declaration: true
// @lib: es6
// @filename: foo.ts
module.exports = Point;
export { };
// @filename: foo2.ts
class Bar {
    constructor(input){}
}
export { Bar };
// @filename: usage.ts
export const x = {
    x: 0,
    y: 0,
    data: {
        x: 12
    }
};
export let y = {
    a: "",
    b: 0,
    data: {
        x: 12
    }
};
export class Bar2 {
    constructor(input){}
}
export let shim = {
    Bar: Bar2
};
