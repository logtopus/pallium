import { Pallium } from "pallium";

const pallium = Pallium.new();

var srcCount = pallium.source_count();
var sourceChoice = document.getElementById('SourceChoice');

for (var i = 0; i < srcCount; i++) {
    var src = pallium.source(i);
    var opt = document.createElement('option');
    opt.innerHTML = src;
    opt.value = src;
    sourceChoice.appendChild(opt);
}

var content = document.getElementById('Content');
for (var i = 0; i < 200; i++) {
    var tr = document.createElement('tr');

    var td = document.createElement('td');
    td.textContent = "2019-03-10 15:42:09";
    tr.appendChild(td);

    var td = document.createElement('td');
    if (i % 3 == 0) {
        td.className = "error";
        td.textContent = "ERROR";
    }
    else if (i % 3 == 1) {
        td.className = "warning";
        td.textContent = "WARN";
    }
    else {
        td.className = "debug";
        td.textContent = "DEBUG";
    }
    tr.appendChild(td);

    var td = document.createElement('td');
    td.textContent = "This is a warning message " + i;
    tr.appendChild(td);
    content.appendChild(tr);
}
