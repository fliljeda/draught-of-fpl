#!/bin/bash

#Checking prerequisites
if [ ! $(type -t jq) ]; then
    echo "jq not installed"
    exit
fi

if [ ! $(type -t curl) ]; then
    echo "curl not installed"
    exit
fi


path_curl() {
    path=$(echo $1 | sed -e 's/^https:\/\/draft.premierleague.com\///g')
    dirpath=$(echo $path | sed -e 's/\(.*\)\/.*/\1/g')
    mkdir -p $dirpath
    local curl_res=$(set -x; curl -s "$1")
    pc_json=$(echo $curl_res | jq -r -M ".")
    if [ ! -e "$path" ]; then
        echo "$pc_json" > $path
    fi
}


if [ -z $1 ]; then
    echo "Need league code as argument"
    exit
else
    _league="$1"
fi

_api_prefix="https://draft.premierleague.com/api"

#Fetch general things
path_curl "${_api_prefix}/bootstrap-static"

#Get gameweek range
path_curl "${_api_prefix}/game"
_current_gw=$(echo $pc_json | jq ".current_event")

#Get teams 
path_curl "${_api_prefix}/league/${_league}/details"
_teams=$(echo $pc_json | jq ".league_entries[].entry_id")



#Fetch team specific
for team_id in $_teams; do
    path_curl "${_api_prefix}/entry/${team_id}/public"
done
if [ ! "$_current_gw" == "null" ]; then
    for gw in $(seq $_current_gw); do
        #Fetch GW specific
        path_curl "${_api_prefix}/event/${gw}/live"

        for team_id in $_teams; do
            #Fetch GW + Team specific
            path_curl "${_api_prefix}/entry/${team_id}/event/${gw}"
        done
    done
else
    echo "Current GW is null, preseason!"
    exit
fi
