// @module: es2020
// @target: es2020
// @filename: 0.ts
export class B {
    print() {
        return "I am B";
    }
}
// @filename: 2.ts
import _async_to_generator from "@swc/helpers/src/_async_to_generator.mjs";
function foo() {
    return _foo.apply(this, arguments);
}
function _foo() {
    _foo = _async_to_generator(function*() {
        class C extends (yield import("./0")).B {
        }
        var c = new C();
        c.print();
    });
    return _foo.apply(this, arguments);
}
foo();
