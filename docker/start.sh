#!/bin/sh

crond -b -l 8 && /docker-entrypoint.sh "$@"
