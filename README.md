<h1 align="center">
<img align="center" width="160" height="160" src="logo.png" alt="Liveboat"><br/>
Liveboat
</h1>
<h1>See <a href="https://konrad.website/liveboat-github-runner" target="_blank">Demo</a></h1>

Liveboat generates static pages based on the Newsboat RSS Reader configuration, content which can be easily hosted online and reached even when away from the terminal.

## Running via Github actions
The most straightforward way to generate Liveboat feed page is via Github Actions - the site will be uploaded to Github Pages on your account, available immediately and set up with automatic updates.
For details follow instructions at [https://github.com/exaroth/liveboat-github-runner](https://github.com/exaroth/liveboat-github-runner)

## Running locally

### Installation

For Arch users:

``` sh
pacman -S liveboat
```

For Ubuntu/Deb:

``` sh
snap install liveboat
```
Other Linux distros:

via `wget`

``` sh
wget -O /bin/liveboat https://github.com/exaroth/liveboat/releases/download/stable/liveboat-musl
```

via `curl`
``` sh
curl -o /bin/liveboat https://github.com/exaroth/liveboat/releases/download/stable/liveboat-musl
```

OSX:

``` sh
brew install liveboat
```

Pre-built binaries are available at the [Releases](https://github.com/exaroth/liveboat/releases/tag/stable) page.

### Compiling from source

After cloning repository run `make install && make build` to build the binary. Rustc/Cargo required.

## Usage

If you're not Newsboat user yet see [Newsboat documentation](https://newsboat.org/releases/2.10.2/docs/newsboat.html) for set up and configuration details.
<br/>
<br/>
After installing Liveboat execute `liveboat -x init` to set up configuration and download the build templates.

### Running on every Newsboat update

Add following lines to your Newsboat config file (typically stored at `~/.newsboat/config`)

```
notify-always yes
notify-format "/path/to/liveboat/build_directory"
notify-program "liveboat"
```
This will trigger page rebuild every time Newsboat reloads feeds list.

###  Setting up scheduled rebuilds

If you don't want to run Liveboat on every Newsboat rebuild you can set up a cron job to run it:

`crontab -e`

```
*/30 * * * *  newsboat -x reload && liveboat
```

To manually rebuild Newsboat feeds and generate the page every 30 minutes

### Executing manually

Run `liveboat --help` for list of all available arguments

> [!IMPORTANT]
> During every update Liveboat will only regenerate template files and feed list, if you want to add additional files or directories such as git repository feel free to do so as these won't be overwritten when rebuilding feed pages.

### Options file

Configuration file can be found at `~/.config/liveboat/config.toml` and stores options related to page generation.

- `title` - Main title for the feed page.
- `site_path` - This defines base path under which feed page will be hosted, unless deployed at the root domain this variable should be updated, eg. if hosted on the Github Pages (as a repository) this will need to be changed to `/<repo_name>/`.
- `show_read_articles` - Whether or not to include articles marked as read by Newsboat.
- `time_threshold` - Amount of time in the past (in days) for which Liveboat should look for when retrieving articles. 
- `template_name` - Name of the template to use when generating the feed page, templates are stored at `~/.config/liveboat/templates`, if you want to use template located elsewhere use `--template-path` argument when invoking Liveboat.
- `build_dir` - Default path to directory where Liveboat will output feed page files, can be overwritten via `--build-dir` argument.
- `newsboat_urls_file` - Path to Newsboat urls file.
- `newsboat_cache_file` - Path to file containing Newsboat cache db.

### Updating liveboat

Execute `liveboat -x update` to check for new versions of Liveboat and update if one exists, including templates.


### Git integration

Liveboat does not include any git integration if you want to automatically push newly generated page to Github or anywhere else you can wrap the execution in shell script. For example:
``` sh
#!/bin/sh
build_dir="$HOME/liveboat_build";
timestamp=$(date +%s);
liveboat $build_dir;
cd $build_dir && git add -A . && git commit -a -m "Liveboat build @ $timestamp" && git push;
cd -
```

## Template development

See [https://github.com/exaroth/liveboat/templates/README.md](https://github.com/exaroth/liveboat/tree/develop/templates) for details about developing your own template or modifying existing one.

### Compatibility

Newsboat is compatible with Newsboat urls filtering and aggregation syntax, generated pages will contain same attributes as those displayed in the terminal. It supports query filter syntax as well, with following exceptions:
- `description` `<rss_feed_attribute>`
- `feed_date` `<rss_feed_attribute>`
- `feed_index` `<rss_feed_attribute>`
- `article_index` `<rss_article_attribute>`

If your urls file contains any of the above filters these will be ignored when generating the page. 

## Acknowledgements
- Team behind Newsboat/Newsbeuter RSS readers for making amazing app :)
 
## License
Liveboat is provided under MIT License, see `LICENSE` file for details


