# Liveboat Templates Reference

This document contains details about Liveboat template structure and instruction on how to start building static pages with Liveboat.

> [!TIP]
> If you'd like to change layout of any of default supplied templates to match your needs it is advisable to not overwrite files at `~/.config/liveboat/templates` but instead use a copy and change `template_name` in Liveboat options or use explicit `--template-path` argument when building as supplied templates might be overwritten during update.

Each template is composed of at least 2 required components:
- Configuration file (`config.toml`)
- Set of Handlebars templates used for rendering page (`index.hbs`, `feed.hbs`, `article.hbs` depending on build mode)

For quick start see `plain` and `plain-ext` templates in [default template directory](https://github.com/exaroth/liveboat/blob/develop/templates), these templates are easy way to start building custom pages with Liveboat.

## Configuration file

Each template needs to include configuration file - `config.toml` which contains all the settings used for generating HTML and associated data.

Setting are:

- `name` - Name of the template
- `version` - version of the template (in a form of `MAJOR.MINOR.PATCH`)
- `builder` - this denotes build mode to be used when generating pages, available build modes:
    + `spa` - Used for single page applications, only index page will be rendered based on `index.hbs` template.
    + `multipage` - If set liveboat will generate both index page as well as separate page for each feed using `feed.hbs` template.
    + `multipage-ext` - This mode is similar to `multipage` setting but will also generate separate HTML page for each article in each feed using `article.hbs` template.
- `include_json_feeds` - If set to `true` liveboat will compile JSON files containing feed data, see 'JSON Feeds' section below for details.
- `include_rss` - If set to `true` liveboat will attach `rss.xml` file with all compiled feed articles in build directory (useful when using 3rd party RSS readers with liveboat)
- `include_opml` - Set it to include OPML data in generated project (used by some RSS readers).
- `template_settings` - all fields in this section will be passed in template context when rendering (in `template_settings` key) - these can be used to modify look and behaviour of the page by the user. IMPORTANT: All values should be strings.

## Include directory

If you want to include any auxiliary files with the template (images/scripts/styles etc.) put them inside `<template_name>/include` dir - contents of this directory will be copied verbatim when building page.

## Build modes

Below is the overview of each build mode as set in template configuration file and details of using Handlebars templates for each one.

### SPA

This mode is enabled by setting `builder` key to `spa` in template configuration file. When enabled liveboat will only generate single page based on `index.hbs` included in template.

Each `index.hbs` is rendered with context containing following variables:

- `feeds` - array of all feeds which contained at least one article, see serializer reference @ [feed.rs](https://github.com/exaroth/liveboat/blob/develop/src/feed.rs) for details. This also includes all articles associated with each feed.
- `options` - Page generation options as defined at [opts.rs](https://github.com/exaroth/liveboat/blob/develop/src/opts.rs)
- `build_time` - Timestamp containing build generation time
- `template_settings` - A dictionary containing any arbitrary data to be passed to in the context as defined in `config.toml` file for given template.
- `template_version` - String denoting current version of the template

### MULTIPAGE

This mode is enabled by setting `builder` to `multipage`, in this mode liveboat will generate both index as well as separate HTML page for each feed using `feed.hbs` template. Both `index.hbs` and `feed.hbs` templates must be present in template directory. TODO update after adding opt.

When rendering `feed.hbs` following variables are available:

- `feed` - feed data as described in [feed.rs](https://.com/exaroth/liveboat/blob/develop/src/feed.rs) serializer.
- `items` - articles associated with given feed - see [feed_item.rs](https://github.com/exaroth/liveboat/blob/develop/src/feed_item.rs) serializer reference for details.
- `options`, `build_time`, `template_settings`, `template_version` - same as in `spa` mode.

Context for `index.hbs` is same as in `spa` mode.

### MULTIPAGE EXTENDED

This mode is enabled by setting `builder` to `multipage-ext` - it's similar to `multipage` but will also generate separate HTML page for each article.

This mode requires including `article.hbs` file in template directory, as well as `index.hbs` and `feed.hbs`.

`article.hbs` is rendered with following context:

- `article` - current article being displayed, see [feed_item.rs](https://github.com/exaroth/liveboat/blob/develop/src/feed_item.rs) serializer reference for details.
- `feed` - identical to `feed` in `feed.hbs` template context.
- `options`, `build_time`, `template_settings`, `template_version`


### JSON Feeds

If `include_json_feeds` is set in template configuration file Liveboat will generate JSON files containing feed data, this is useful when building JS based Single Page Applications(SPA) or can be used as single idempodent API by 3rd party apps.

Liveboat will save JSON feed data in `<build_dir>/feeds` directory (one per feed) with all the articles which matched the criteria. Query feeds are treated same way as ordinary URL based feeds (with the exception of `is_query` flag). On top of that list of feeds and query feeds will be saved in `feeds.json`. See `Serializer` reference at [https://github.com/exaroth/liveboat/src/feed_item.rs](https://github.com/exaroth/liveboat/blob/develop/src/feed_item.rs) for details about available fields in each feed article rendered in `JSON` file.

> [!NOTE]
> `JSON` files are minified by default unless template is built with `--debug` options. <br/>

> [!NOTE]
> Feeds array is sorted so that the order corresponds to the order as defined in Newsboat urls file, article items within JSON files are sorted from latest to oldest.

> [!NOTE]
> Each JSON file holding feed data contains only limited number of articles - the formula is `min(<num_total_articles>, max(<num_articles_from_last_2_days>, 50))` - this is done in order to limit size of json files to be loaded by default. Each feed file does have corresponding archive version with suffix `_archive` which contains full non-truncated list of articles, note however that even this archive version is limited by particular Liveboat and Newsboat settings - `time_threshold` setting in case of Liveboat and `keep-articles-days` for Newsboat.

## Submitting templates to repository

If you would like to submit your own template, which will be available as default for every user supply it as described below.

### Template structure

This assumes template running in `spa` mode with `include_json_feeds` option set.

```
templates
└── <template-name>
   ├── dist            <---- Contains files which will be added to release
   │   ├── include     <---- All static files for the template
   │   ├── index.hbs   <---- Handlebars HTML file
   │   └── config.toml <---- Template configuration file
   └── src             <---- Source files for template (OPTIONAL)
       ├── include
       ├── index.hbs
       └── config.toml
```

Template supplied using this directory structure will be shipped as:

```
├── include
│   └── assets
│       ├── index.css
│       └── index.js
├── index.hbs
└── config.toml
```

When building the template Liveboat will flatten the tree outputting all files within `include` directory on the same level as compiled `index.html` with `feeds` directory included, so that above structure will be built as:

```
├── assets
│   ├── index.css
│   ├── index.js
├── feeds
│   ├── <feed_id_1>.json
│   ├── <feed_id_1>_archive.json
│   ├── ...
│   └── feeds.json
└── index.html
```
