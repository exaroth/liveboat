#!/bin/sh

# This script will trigger page rebuild for Liveboat
 
echo "Rebuilding Liveboat page, please wait..."
/usr/bin/newsboat -x reload -u /liveboat/urls -c /liveboat/cache.db
LIVEBOAT_CONFIG_DIR=/liveboat /usr/bin/liveboat -x build --build-dir /liveboat/build
