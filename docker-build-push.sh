#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "please specify tag"
    exit 1
fi


echo "building images... latest, $1 "

docker build -t jaemk/homepage:$1 .
docker build -t jaemk/homepage:latest .

if [ "$2" = "push" ]; then
    echo "pushing images..."
    set -x
    docker push jaemk/homepage:$1
    docker push jaemk/homepage:latest
fi
