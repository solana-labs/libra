#!/usr/bin/env bash

cargo install $1

RET=$?

if [[ RET -eq 0 ]] || [[ RET -eq 101 ]]; then
    exit 0
fi

exit ${RET}