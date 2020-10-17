var HOST = "localhost"
var PORT = "8000"
var PAGE_TITLE = "FPL Draft"

var dof_url = "https://" + HOST + ":" + PORT +  "/table"
document.title = PAGE_TITLE

fetchAndUpdate()
setInterval(fetchAndUpdate, 30000);

function fetchAndUpdate() {
    httpGetAsync(dof_url, function (resp) {
        setUpdatedScores(JSON.parse(resp));
    });
}


function httpGetAsync(theUrl, callback)
{
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() { 
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            callback(xmlHttp.responseText);
    }
    xmlHttp.open("GET", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
}

function changeGw(gw){
    document.title = "Draft Gameweek " + gw
}

function setUpdatedScores(table){

    var teamContainer = document.getElementById("team-container")
    teamContainer.innerHTML = ""
    var template = document.getElementById("team-template")
    var topPoints = table["entries"][0]["total_points"]
    for(let x of table["entries"]){
        teamVals = x
        var clone = template.content.cloneNode(true).children[0]
        var teamName = clone.getElementsByClassName("team-name")[0]
        var pointsTot = clone.getElementsByClassName("team-points-total")[0]
        var pointsGw = clone.getElementsByClassName("team-points-gw")[0]
        var pointsDiff = clone.getElementsByClassName("team-points-diff")[0]


        teamName.innerHTML = teamVals["team_name"]

        var total_points = teamVals["total_points"]
        var total_projected_points = teamVals["total_projected_points"]
        var gw_points = teamVals["gw_points"]
        var gw_projected_points = teamVals["gw_projected_points"]

        var proj_total_str = (total_points != total_projected_points ? (" (" +  total_projected_points + ")") : "")
        var proj_gw_str = (gw_points != gw_projected_points ? (" (" +  gw_projected_points + ")") : "")

        pointsTot.innerHTML = "Total:   " + total_points + proj_total_str
        pointsGw.innerHTML = "Gw:   " + gw_points + proj_gw_str

        var diffPoints = (teamVals["total_points"] - topPoints)
        pointsDiff.innerHTML = "Diff:   " + diffPoints

        var proj_button = clone.getElementsByClassName("team-proj-button")[0]
        var proj_expand = clone.getElementsByClassName("team-proj-expand")[0]
        var proj_content = clone.getElementsByClassName("team-proj-content")[0]
        proj_button.href="#collapse_" + teamVals["team_code"]
        proj_expand.id="collapse_" + teamVals["team_code"]

        proj_content.innerHTML = '';
        var list_of_explanations = teamVals["projected_points_explanation"]
        if (list_of_explanations.length > 0) {
            for(let expl of list_of_explanations) {
                var div = document.createElement("div");
                var n = expl["name"]
                var b = expl["bonus_points"]
                var s = expl["subbed_points"]
                if(s) {
                    div.innerHTML = n + " " + s + "p " + (b ? "(" + b + " bonus)" : "") + " sub"
                } else if (b) {
                    div.innerHTML = n + " " + b + "p bonus"
                }
                proj_content.appendChild(div)
            }
        } else {
            proj_button.style.visibility = "hidden";
        }




        teamContainer.append(clone)
    }
}
