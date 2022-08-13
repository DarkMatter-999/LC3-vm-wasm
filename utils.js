
let key = 0;
export function listenKB(e) {
    var keynum;
    if(window.event) { // IE                  
        keynum = e.keyCode;
    } else if(e.which){ // Netscape/Firefox/Opera                 
        keynum = e.which;
    }

    console.log("Listen " + keynum);
    key = keynum;
    return keynum;
}

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