
let key = 0;

export function getkey() {
    return keys;
}

export function printlog(c) {
    var output = document.getElementById("output").innerText;
    // console.log(c);
    if (c === 32) {
        output += '\u00A0';     // shitty js
    } else {
        output += String.fromCharCode(c);
    }

    document.getElementById("output").innerText = output;
    // console.log(c + "  " + String.fromCharCode(c));
    // console.log(String.fromCharCode(c));
}

export function printdisassembly(c) {
    var output = document.getElementById("disassembly").innerText;
    // console.log(c);
    if (c === 32) {
        output += '\u00A0';     // shitty js
    } else {
        output += String.fromCharCode(c);
    }

    document.getElementById("disassembly").innerText = output;
}