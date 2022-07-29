// @module: commonjs
// @Filename: foo_0.ts
import _class_call_check from "@swc/helpers/src/_class_call_check.mjs";
export var C1 = function C1() {
    "use strict";
    _class_call_check(this, C1);
    this.m1 = 42;
};
C1.s1 = true;
// @Filename: foo_1.ts
var foo = require("./foo_0");
if (foo.C1.s1) {
// Should cause runtime import
}
export { };
