#!/usr/bin/env bash

error () {
    echo "$1"
    exit 1
}

main () {
    if (( $# != 2 )); then
        error "Usage: hamming.sh <string1> <string2>"
    fi
    
    if [[ -z ${1} && -n ${2} ]]; then
        error "left strand must not be empty"
    fi

    if [[ -n ${1} && -z ${2} ]]; then
        error "right strand must not be empty"
    fi
 
    if (( ${#1} != ${#2} )); then
        error "left and right strands must be of equal length"
    fi
   
    local -i dist=0
    for ((i=0; i<=${#1}; i++)); do
        if [[ ${1:$i:1} != ${2:$i:1} ]]; then
            dist+=1
        fi
    done
    echo "$dist"
}

main "$@"
