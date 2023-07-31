export default function(s,r){try{if("string"==typeof s&&s.length>0)return function(e){if((e=String(e)).length>100)throw Error("Value exceeds the maximum length of 100 characters.");const s=/^(-?(?:\d+)?\.?\d+) *(milliseconds?|msecs?|ms|seconds?|secs?|s|minutes?|mins?|m|hours?|hrs?|h|days?|d|weeks?|w|years?|yrs?|y)?$/i.exec(e);if(!s)return NaN;const r=parseFloat(s[1]),a=(s[2]||"ms").toLowerCase();switch(a){case"years":case"year":case"yrs":case"yr":case"y":return 315576e5*r;case"weeks":case"week":case"w":return 6048e5*r;case"days":case"day":case"d":return 864e5*r;case"hours":case"hour":case"hrs":case"hr":case"h":return 36e5*r;case"minutes":case"minute":case"mins":case"min":case"m":return 6e4*r;case"seconds":case"second":case"secs":case"sec":case"s":return 1e3*r;case"milliseconds":case"millisecond":case"msecs":case"msec":case"ms":return r;default:throw Error(`The unit ${a} was matched, but no matching case exists.`)}}(s);if("number"==typeof s&&isFinite(s))return r?.long?function(s){const r=Math.abs(s);return r>=864e5?e(s,r,864e5,"day"):r>=36e5?e(s,r,36e5,"hour"):r>=6e4?e(s,r,6e4,"minute"):r>=1e3?e(s,r,1e3,"second"):`${s} ms`}(s):function(e){const s=Math.abs(e);return s>=864e5?`${Math.round(e/864e5)}d`:s>=36e5?`${Math.round(e/36e5)}h`:s>=6e4?`${Math.round(e/6e4)}m`:s>=1e3?`${Math.round(e/1e3)}s`:`${e}ms`}(s);throw Error("Value is not a string or number.")}catch(r){const e="object"==typeof r&&null!==r&&"message"in r?`${r.message}. value=${JSON.stringify(s)}`:"An unknown error has occured.";throw Error(e)}}function e(e,s,r,a){return`${Math.round(e/r)} ${a}${s>=1.5*r?"s":""}`}
