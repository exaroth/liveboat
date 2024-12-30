<h1 align="center">
<img align="center" width="130" height="130" src="logo.png" alt="Liveboat"><br/>
Liveboat
</h1>
<h1>See <a href="https://konrad.website/liveboat-github-runner" target="_blank">Demo</a></h1>

![stable](https://github.com/exaroth/liveboat/actions/workflows/test.yml/badge.svg?branch=main)
[![License](https://img.shields.io/github/license/exaroth/liveboat)](https://github.com/exaroth/liveboat/blob/develop/LICENSE)
[![rustc 1.84.0](https://img.shields.io/badge/rust-1.84%2B-orange.svg)](https://img.shields.io/badge/rust-1.84%2B-orange.svg)

## What Liveboat is about
- Generate static pages for your RSS/Atom subscriptions allowing you to access all the news you follow from the browser
- Aggregate all the subscriptions in one place so you can use single feed source in any of your RSS clients. Liveboat also provides OPML file for all your subscriptions.
- Easily deployable to Github Pages - See [liveboat-github-runner](https://github.com/exaroth/liveboat-github-runner) template for details
- Liveboat exposes simple JSON API you can use to integrate RSS subscriptions into your apps [Using Liveboat's JSON API](#using-liveboat-json-api)
- Compatible with Newsboat url file format including query feeds
- Templating support - See [Template development guide](https://github.com/exaroth/liveboat/tree/develop/templates) for details

## Running via Github actions
The most straightforward way to generate Liveboat feed page is via Github Actions - the site will be uploaded to Github Pages on your account, available immediately and set up with automatic updates.
For details follow instructions at [https://github.com/exaroth/liveboat-github-runner](https://github.com/exaroth/liveboat-github-runner)

## Running locally

### Installation

For Arch users:

``` sh
yay -S liveboat
```

Other Linux distros:

via `cargo`
```
cargo install liveboat
```

via `wget`

``` sh
sh -c 'wget -O /usr/local/bin/liveboat https://github.com/exaroth/liveboat/releases/download/stable/liveboat-linux-musl && chmod +x /usr/local/bin/liveboat'
```

OSX:

``` sh
brew tap exaroth/liveboat
brew install liveboat
```

Pre-built binaries are available at the [Releases](https://github.com/exaroth/liveboat/releases/tag/stable) page.

### Compiling from source

After cloning repository run `make setup && make build` to build the binary. Rustc/Cargo required.

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
> [!IMPORTANT]
> If you want to pass any named arguments to liveboat, wrap the execution into a shell script as Newsboat will fail to run the command if any were passed. Also consider passing full path to executable in `notify program` parameter if you installed liveboat using cargo or if the binary is in non standard path eg. outside of  '/bin', '/usr/bin' or '/usr/local/bin'.


###  Setting up scheduled rebuilds

If you don't want to run Liveboat on every Newsboat rebuild you can set up a cron job to run it:

`crontab -e`

```
*/30 * * * *  newsboat -x reload && liveboat
```

To manually rebuild Newsboat feeds and generate the page every 30 minutes

### Executing manually


```
Usage: liveboat [OPTIONS] [BUILD_TARGET]...

Arguments:
  [BUILD_TARGET]...  Optional path to build directory

Options:
      --cache-file <CACHE_FILE>        Path to newsboat db cache
      --url-file <URL_FILE>            Path to newsboat urls file
      --build-dir <BUILD_DIR>          Path to build directory
      --template-path <TEMPLATE_PATH>  Path to directory containing Liveboat template
      --config-file <CONFIG_FILE>      path to liveboat config file
      --debug                          Print verbose code execution info
      --use-nightly                    If set will use nightly channel for updates
  -x <COMMAND>                         Command to execute [available options: build, init, update] [default: build]
  -h, --help                           Print help
  -V, --version                        Print version
```

> [!IMPORTANT]
> During every update Liveboat will only regenerate template files and feed list, if you want to add additional files or directories such as git repository feel free to do so as these won't be overwritten when rebuilding feed pages.

### Options file

Configuration file can be found at `~/.config/liveboat/config.toml` and stores options related to page generation.

- `title` - Main title for the feed page.
- `site_path` - This defines base path under which feed page will be hosted, unless deployed at the root domain this variable should be updated, eg. if hosted on the Github Pages (as a repository) this will need to be changed to `/<repo_name>/`.
- `show_read_articles` - Whether or not to include articles marked as read by Newsboat.
- `time_threshold` - Amount of time in the past (in days) for which Liveboat should look for when retrieving articles. 
- `template_name` - Name of the template to use when generating the feed page, templates are stored at `~/.config/liveboat/templates`, if you want to use template located elsewhere use `--template-path` argument when invoking Liveboat.
- `include_article_content_in_rss_feeds` - Set this option to true to include article content in aggregated rss xml file, it might increase file size significantly
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
### Using Liveboat JSON API

Liveboat exposes simple idempodent API consisting of 3 endpoints

- `GET <address>/feeds/feeds.json`:  Retrieve list of all RSS feeds available, use it to retrieve ids of the feeds which can be used in 2 following calls to fetch article items.
- `GET <address>/feeds/<feed_id>.json` - Retrieve feed details along with compacted list of the most recent articles using formula `min(<num_total_articles>, max(<num_articles_from_last_7_days>, 50))`
- `GET <address>/feeds/<feed_id>_archive.json` - This call will fetch feed data alongside all the article items associated with that feed.

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


