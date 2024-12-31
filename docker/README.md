# Liveboat Docker image

## Liveboat is available as [ghcr hosted image] you can use to easily run Liveboat

__NOTE__ at minimum you should provide list of urls to generate page with, see [url reference](https://github.com/exaroth/liveboat-github-runner?tab=readme-ov-file#liveboat-url-file-breakdown) for details

``` sh
docker run -p 8080:8080 -v <path_to_url_file>:/liveboat/urls  ghcr.io/exaroth/liveboat:latest
```
---
If you'd like to use custom page settings:

``` sh
docker run -p 8080:8080 -v <path_to_url_file>:/liveboat/urls -v <path_to_config_file>:/liveboat/config.toml ghcr.io/exaroth/liveboat:latest
```

### docker directory structure

```
root
│
├──liveboat
│    ├── build <---- build directory
│    ├── cache.db <---- newsboat cache
│    ├── config.toml <---- liveboat configuration
│    ├── templates  <---- template directory
│    │   └── default
│    └── urls <---- urls file
│
├──start.sh <- main entrypoint
└──entrypoint.sh <- nginx entrypoint
```

## Building docker image manually

This directory contains all the files required for building and running
Liveboat docker image - the image by default will run nginx server hosting
Liveboat website on port 8080, it will also run scheduled feed rebuild task
every 15 minutes.

> [!IMPORTANT]
> Before building image update `urls` file adding RSS urls you want to follow beforehand
> as well as `config.toml` file to change default settings
> (See [Options File Reference](https://github.com/exaroth/liveboat?tab=readme-ov-file#options-file))
> for details.
