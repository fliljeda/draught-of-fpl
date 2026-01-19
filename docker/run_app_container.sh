#!/usr/bin/env bash

function cleanup {
    docker container stop dof >/dev/null 2>&1 || true
    docker container rm dof >/dev/null 2>&1 || true
}
trap cleanup INT EXIT TERM

docker run --init --rm -p 8000:80 --name dof dof
