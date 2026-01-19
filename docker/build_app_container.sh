#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
docker build -f "${SCRIPT_DIR}"/app.Dockerfile --tag dof:latest "${SCRIPT_DIR}/.."