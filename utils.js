
let key = 0;

export function getkey() {
    return keys;
}

export function printlog(c) {
    if(c === 10) {
        println();
    }
    // var output = document.getElementById("output").innerText;
    var output = "";
    // // console.log(c);
    if (c === 32) {
        output += '\u00A0';     // shitty js
    } else {
        output += String.fromCharCode(c);
    }

    globalThis.outputbuffer += output;

    // document.getElementById("output").innerText = output;
    // console.log(c + "  " + String.fromCharCode(c));
    // console.log(String.fromCharCode(c));
}

export function println() {
    document.getElementById("output").innerText += globalThis.outputbuffer;

    globalThis.outputbuffer = "";
}

export function printdebug() {
    document.getElementById("disassembly").innerText = globalThis.debugbuffer + document.getElementById("disassembly").innerText;
    globalThis.debugbuffer = "";
}

export function printdisassembly(c) {
    if(c === 10) {
        printdebug();
    }

    var output = "";
    // // console.log(c);
    if (c === 32) {
        output += '\u00A0';     // shitty js
    } else {
        output += String.fromCharCode(c);
    }

    globalThis.debugbuffer += output;
}