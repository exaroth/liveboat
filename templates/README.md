# Liveboat Templates Reference

> [!IMPORTANT]
> At the moment Liveboat supports only SPA (Single Page Ppplication) templates with single HTML page being rendered.

This file contains details about Liveboat template structure as well as instructions about developing and updating new templates.

## Template structure

```
templates
└── <template-name>
   ├── dist            <---- Contains files which will be added to release
   │   ├── include     <---- All static files for the template
   │   ├── index.hbs   <---- Handlebars HTML file
   │   └── config.toml <---- Template configuration file
   └── src             <---- Source files for template
       ├── include
       ├── index.hbs
       └── config.toml
```

Above describes structure for the template dir, when building the template Liveboat will flatten the tree outputting all files within `include` directory on the same level as compiled `index.html` with `feeds` directory included, so that this structure:

```
├── include
│   └── assets
│       ├── index.css
│       └── index.js
├── index.hbs
└── config.toml
```

Will become:

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

Feeds directory contains `JSON` files (one per feed) with all the articles which matched the criteria. Query feeds are treated same way as ordinary URL based feeds (with the exception of `is_query` flag). On top of that list of feeds and query feeds will be saved in `feeds.json` and `query_feeds.json`. See `Serializer` reference at [https://github.com/exaroth/liveboat/src/feed_item.rs](https://github.com/exaroth/liveboat/blob/develop/src/feed_item.rs) for details about available  fields in each feed article rendered in `JSON` file.

> [!NOTE]
> `JSON` files are minified by default unless template is built with `--debug` options. <br/>
 
> [!NOTE]
> Feeds array is sorted so that the order corresponds to the order as defined in Newsboat urls file, article items within JSON files are sorted from latest to oldest.

> [!NOTE]
> Each JSON file holding feed data contains only limited number of articles - the formula is `min(<num_total_articles>, max(<num_articles_from_last_7_days>, 50))` - this is done in order to limit size of json files to be loaded by default. Each feed file does have corresponding archive version with suffix `_archive` which contains full non-truncated list of articles, note however that even this archive version is limited by particular Liveboat and Newsboat settings - `time_threshold` setting in case of Liveboat and `keep-articles-days` for Newsboat.

## Template context

Each `index.hbs` is rendered with context containing following variables:

- `feeds` - array of all feeds which contained at least one article, see serializer reference @ [https://github.com/exaroth/liveboat/src/feed.rs](https://github.com/exaroth/liveboat/blob/develop/src/feed.rs) for details. This also includes all articles associated with each feed.
- `options` - Page generation options as defined at [https://github.com/exaroth/liveboat/src/opts.rs](https://github.com/exaroth/liveboat/blob/develop/src/opts.rs)
- `build_time` - Timestamp containing build generation time
- `template_settings` - A dictionary containing any arbitrary data to be passed to in the context as defined in `config.toml` file for given template.
- `template_version` - String denoting current version of the template

## Template `config.toml` file
Each template must contain `config.toml` file containing following fields

- `builder` - this denotes type of builder module to be used for building template, as for now the only available version is `spa`
- `version` - current version of the template in a format `<major>.<minor>.<fix>`
- `template_settings` - Arbitrary string based map containing custom data influencing rendering of the template, so that user can customize it without having to update template code. This data will be attached to template context during rendering.


> [!TIP]
> If you'd like to change layout of any existing template to match your needs it is advisable to not overwrite files at `~/.config/liveboat/templates` but instead use a copy of it and change `template_name` in options or use explicit `--template-path` argument when building as default templates might be overwritten during update. For working with source files use `setup-default-template-dev` in `Makefile` as reference on setting up development environment.
