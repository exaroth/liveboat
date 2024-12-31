# Liveboat Docker image

This directory contains all the files required for building and running
Liveboat docker image - the image by default will run nginx server hosting
Liveboat website on port 8080, it will also run scheduled feed rebuild task
every 15 minutes. 

> [!IMPORTANT]
> Before building image update `urls` file adding RSS urls you want to follow beforehand
> as well as `config.toml` file to change default settings
> (See [Options File Reference](https://github.com/exaroth/liveboat?tab=readme-ov-file#options-file))
> for details
