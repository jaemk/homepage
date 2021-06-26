#!/bin/bash

set -e

# update version
tag="$(git rev-parse HEAD | head -c 7 | awk '{ printf "%s", $0 }')"

echo "building images... latest, $tag "

tagged_image="docker.jaemk.me/homepage:$tag"
latest_image="docker.jaemk.me/homepage:latest"

docker build -t $tagged_image .
docker build -t $latest_image .

ports="-p 4000:4000"

# set envs from csv env var
if [[ -z "$ENVS" ]]; then
    envs="$envs"
else
    for e_str in $(echo $ENVS | tr "," "\n")
    do
        envs="-e $e_str $envs"
    done
fi

# set key-value pairs if there's an .env.local
if [[ -z "$ENVFILE" ]]; then
    if [ -f .env.local ]; then
        envfile="--env-file .env.local"
    fi
else
    envfile="--env-file $ENVFILE"
fi


if [ "$1" = "run" ]; then
    echo "running..."
    set -x
    docker run --rm -it --init $ports $envs $envfile $latest_image
elif [ "$1" = "shell" ]; then
    echo "running..."
    set -x
    docker run --rm -it --init $ports $envs $envfile $latest_image /bin/bash
elif [ "$1" = "push" ]; then
    echo "pushing images..."
    set -x
    docker push $tagged_image
    docker push $latest_image
fi
