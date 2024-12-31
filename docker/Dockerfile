FROM nginx:alpine

WORKDIR /liveboat

ADD config.toml urls /liveboat/
ADD liveboat.conf /etc/nginx/conf.d/default.conf
ADD rebuild.sh /etc/periodic/15min

RUN  apk update && apk add newsboat wget

# Fetch stable version of Liveboat
RUN wget -O /usr/bin/liveboat https://github.com/exaroth/liveboat/releases/download/stable/liveboat-linux-musl && \
    chmod +x /usr/bin/liveboat

# Fetch templates
RUN LIVEBOAT_CONFIG_DIR=/liveboat /usr/bin/liveboat -x update
RUN /etc/periodic/15min/rebuild.sh

EXPOSE 8080
