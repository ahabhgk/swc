//@module: commonjs
//@target: ES3
// @filename: m1.ts
var R;
export default R = {
    "__esmodule": true,
    "__proto__": {}
};
// @filename: m2.ts
import R from "./m1";
var __esmodule = R.__esmodule, __proto__ = R.__proto__;
