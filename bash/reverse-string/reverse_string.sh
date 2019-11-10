#!/usr/bin/env bash

main () {
    out=""
    for (( i=${#1}-1; i>=0; i-- )); do
        out+=${1:$i:1}
    done
    echo "$out"
}

main "$@"
