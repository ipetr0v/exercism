#!/bin/bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
echo $SCRIPTPATH
for D in `ls $SCRIPTPATH`; do
    if [[ -d $D ]]; then 
        echo Cleaning $D
        (cd $D; stack clean)
    fi
done
