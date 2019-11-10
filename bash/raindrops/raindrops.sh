#!/usr/bin/env bash

main () {
    out=""
    (( $1 % 3 == 0 )) && out+="Pling"
    (( $1 % 5 == 0 )) && out+="Plang"
    (( $1 % 7 == 0 )) && out+="Plong"
    echo "${out:-$1}"
}

main "$@"
