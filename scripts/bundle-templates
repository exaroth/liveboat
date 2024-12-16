#! /bin/bash

rm -Rf tmp && mkdir -p ./tmp/templates

declare -a dirs
i=1

for d in ./templates/*
do
    if  test -d "$d"; then
        dirs[i++]="${d%/}"
    fi;
done

for((i=1;i<=${#dirs[@]};i++))
do
    declare -a dd="${dirs[i]}/dist"
    declare -a target="${dirs[i]##*/}"
    if [ -d $dd ]; then 
        cp -Rf $dd "./tmp/templates/$target"
    fi
done

cd tmp && tar -czf ./templates.tar.gz ./templates
