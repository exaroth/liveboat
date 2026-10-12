<a href="https://github.com/exaroth/liveboat" id="githubfork"><img loading="lazy" decoding="async" width="149" height="149" src="https://github.blog/wp-content/uploads/2008/12/forkme_right_orange_ff7600.png" class="attachment-full size-full" alt="Fork me on GitHub"></a>

<h2 align="center">
<img align="center" width="70" height="70" src="https://github.com/exaroth/liveboat/releases/download/development/liveboat-ico-page.png" alt="" loading="lazy" decoding="async" ><br/>
<br/>
</h2>
# <span style="color: #ff201e">Liveboat</span> is a static page generator which turns RSS reader feed data into beautiful websites.

<h2 align="center">
<img width="2059" height="1298" alt="" align="center" src="https://github.com/exaroth/liveboat/releases/download/development/demo.webp" loading="lazy" decoding="async"/>
</h2>

- Generate static pages for your RSS/Atom subscriptions allowing you to access all the news from the browser
- Aggregate all the subscriptions in one place so you can use single feed source in any of your RSS clients. OPML file is also available.
- Easily deployable to Github Pages - See [liveboat-github-runner](https://github.com/exaroth/liveboat-github-runner) template for details. There's also [Docker image](https://github.com/exaroth/liveboat/tree/develop/docker) included for easy self hosting.
- Liveboat exposes simple JSON API you can use to integrate RSS subscriptions into your apps [Using Liveboat's JSON API](#using-liveboat-json-api)
- Compatible with Newsboat url file format including query feeds
- Templating support - See [Template development guide](https://github.com/exaroth/liveboat/tree/develop/templates) for details

## Github Pages setup

- Visit [Liveboat Github Runner](https://github.com/exaroth/liveboat-github-runner) repository and click `Use this template` button in top right corner. Select name and privacy settings
- After the repository has been created use `git clone` to download it.
- Update configuration and urls file inside cloned repository.
    + Edit `config/liveboat-config.toml` file, update `title` and  `site_path` fields - site path needs to be set to `/<repo_name>/` where `repo_name` corresponds to repository name created in Step 1.
    + Replace feed urls in `config/urls` with those you want to follow - simply add 1 atom/rss link per line.
- Commit all the changes and `git push` them to remote.
- Go to `Settings->Actions->General` page within the repo created in Step 1. In `Workflow Permissions` section set `Read and write permissions` and click `Save`.


<img width="80%"  alt="" style="display: block; margin: auto;" src="https://raw.githubusercontent.com/exaroth/liveboat-github-runner/master/assets/screen1.png" loading="lazy" decoding="async"/>


- In Project Settings go to `Pages` tab and under `Build and deployment`, set `Source` to `Deploy from branch`, set `Branch name` to `master` and select `/docs` as the folder to deploy Pages from. Click `Save`.


<img width="80%" alt="" align="center" style="display: block; margin: auto;" src="https://raw.githubusercontent.com/exaroth/liveboat-github-runner/master/assets/screen2.png" loading="lazy" decoding="async"/>


- Execute `git tag build && git push --tags`
- That's it. Your Liveboat page is live and available at `https:://<username>.github.io/<repo_name>`.

## Local Installation

- Cargo

```
cargo install liveboat
```

- Snapcraft

```sh
snap install liveboat
```

- AUR

```sh
yay -S liveboat
```

- wget (Linux)

```sh
sh -c 'wget -O /usr/local/bin/liveboat https://github.com/exaroth/liveboat/releases/download/stable/liveboat-linux-musl && chmod +x /usr/local/bin/liveboat'
```

- Homebrew (MacOS)

```sh
brew tap exaroth/liveboat
brew install liveboat
```

<br/>

<span id="text-centered">MIT Licence - 2025-2026 Konrad Wasowicz `<exaroth@gmail.com>`</span>
