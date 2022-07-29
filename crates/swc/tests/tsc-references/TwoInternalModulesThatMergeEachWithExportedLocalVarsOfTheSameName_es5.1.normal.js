//@filename: part1.ts
export var A;
(function(A) {
    var Utils;
    (function(Utils) {
        function mirror(p) {
            return {
                x: p.y,
                y: p.x
            };
        }
        Utils.mirror = mirror;
    })(Utils = A.Utils || (A.Utils = {}));
    var Origin = A.Origin = {
        x: 0,
        y: 0
    };
})(A || (A = {}));
//@filename: part2.ts
import _class_call_check from "@swc/helpers/src/_class_call_check.mjs";
export var A;
(function(A) {
    var Origin = A.Origin = {
        x: 0,
        y: 0
    };
    var Utils;
    (function(Utils) {
        var Plane = function Plane(tl, br) {
            "use strict";
            _class_call_check(this, Plane);
            this.tl = tl;
            this.br = br;
        };
        Utils.Plane = Plane;
    })(Utils = A.Utils || (A.Utils = {}));
})(A || (A = {}));
