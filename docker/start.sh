#!/bin/sh

crond -b -l 8 && /etc/periodic/15min/rebuild.sh && /docker-entrypoint.sh "$@"
