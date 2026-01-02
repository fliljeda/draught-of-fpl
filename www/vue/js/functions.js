
function httpGetAsync(theUrl, callback) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function () {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200) {
            callback(xmlHttp.responseText);
        } else if (xmlHttp.readyState == 4) {
            callback(null);
        }
    }
    xmlHttp.open("GET", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
}

function httpGetBlocking(theUrl, callback) {
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function () {
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200) {
            callback(xmlHttp.responseText);
        } else if (xmlHttp.readyState == 4) {
            callback(null);
        };
    }
    xmlHttp.open("GET", theUrl, false); // true for asynchronous 
    xmlHttp.send(null);
}
