use log::info;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;
use std::io::ErrorKind;
use std::path::Path;
use std::fs;

use anyhow::Result;
use handlebars::Handlebars;
use url::Url;

use crate::builders::aux::*;
use crate::builders::utils::{generate_opml, generate_rss_channel};
use crate::errors::ConfigurationError;
use crate::feed::{Feed, FeedList};
use crate::template::{Context, TemplateConfig, FeedContext};


/// Available modes for rendering templates in the builder.
#[derive(PartialEq)]
enum BuilderMode {
	Spa,
	Multi,
	MultiE,
}


/// This represents default builder module
/// used for processing single page Liveboat templates.
pub struct GenericBuilder<'a, C>
where
	C: serde::Serialize + Context,
{
	template_path: &'a Path,
	template_config: &'a TemplateConfig,
	tmp_dir: &'a Path,
	context: &'a mut C,
	mode: BuilderMode,
	debug: bool,
}

impl<'a, C> Builder for GenericBuilder<'a, C>
where
	C: serde::Serialize + Context,
{
	/// Generate any auxiliary data required for page generation,
	/// such as json feeds.
	fn generate_aux_data(&self) -> Result<()> {
		if self.template_config.include_json_feeds {
			self.save_json_feeds()?;
		}
		if self.template_config.include_rss {
			self.save_rss_channel()?;
		}
		if self.template_config.include_opml {
			self.save_query_feed_channels()?;
			self.save_opml()?;
		}
		self.save_build_time()?;
		Ok(())
	}


	/// Render template using context provided.
	fn render_templates(&self) -> Result<()> {
		self.render_index()?;
		if self.mode == BuilderMode::Multi || self.mode == BuilderMode::MultiE {
			self.render_feed_templates()?;
		}
		if self.mode == BuilderMode::MultiE {
			self.render_article_templates()?;
		}
		Ok(())
	}
}

impl<'a, C> GenericBuilder<'a, C>
where
	C: serde::Serialize + Context,
{
	pub fn init(
		tmp_dir: &'a Path,
		template_path: &'a Path,
		template_config: &'a TemplateConfig,
		context: &'a mut C,
		debug: bool,
	) -> Result<GenericBuilder<'a, C>> {
		if !template_path.try_exists()? {
			return Err(IOError::new(
					ErrorKind::NotFound,
					format!(
						"Template not found at path {}",
						template_path.to_str().unwrap()
					),
			)
				.into());
		}
		let mode_b = template_config.builder.to_lowercase();
		let m = mode_b.as_str();
		let mode: BuilderMode;
		if m == "spa" {
			mode = BuilderMode::Spa
		} else if m == "multipage" {
			mode = BuilderMode::Multi
		} else if m == "multipage-ext" {
			mode = BuilderMode::MultiE
		} else {
			return Err(ConfigurationError::InvalidBuildMode.into());
		}

		Ok(GenericBuilder {
			template_path,
			template_config,
			tmp_dir,
			context,
			mode,
			debug,
		})
	}

	/// Render index template.
	fn render_index(&self) -> Result<()> {
		let tpl_file = self.template_path.join(format!("{}.hbs", INDEX_TEMPLATE_FILENAME));
		if !tpl_file.try_exists()? {
			return Err(IOError::new(
					ErrorKind::NotFound,
					format!(
						"Index template does not exist for path {}",
						self.template_path.display()
					),
			)
				.into());
				};
		info!("Rendering index template @ {}", &tpl_file.display());
		let raw = fs::read_to_string(tpl_file)?;
		let mut handlebars = Handlebars::new();
		_ = handlebars.register_template_string(INDEX_TEMPLATE_FILENAME, raw);
		let out = handlebars.render(INDEX_TEMPLATE_FILENAME, &self.context)?;

		let out_path = self.tmp_dir.join(format!("{}.html", INDEX_TEMPLATE_FILENAME));
		info!("Saving index page @ {}", &out_path.display());

		let mut f = File::create(out_path)?;
		f.write_all(out.as_bytes())?;

		Ok(())
	}

	/// Render template for each feed.
	fn render_feed_templates(&self) -> Result<()> {
		let tpl_file = self.template_path.join(format!("{}.hbs", FEED_TEMPLATE_FILENAME));
		if !tpl_file.try_exists()? {
			return Err(IOError::new(
					ErrorKind::NotFound,
					format!(
						"Feed template does not exist for path {}",
						self.template_path.display()
					),
			)
				.into());
				};
	
		let feed_p_dir = self.tmp_dir.join(FEED_PAGES_DIRNAME);
		_ = fs::create_dir(&feed_p_dir)?;

		let raw = fs::read_to_string(tpl_file)?;
		let mut handlebars = Handlebars::new();
        _ = handlebars.register_template_string(FEED_TEMPLATE_FILENAME, raw);
		let mut idx: usize = 0;
        let mut feed_ctx: FeedContext;
		for f in self.context.feeds() {
            feed_ctx = FeedContext::init(
                f,
                self.context.options(),
                self.context.template_settings(),
                self.context.template_version(),
                self.context.build_time(),
                );
			idx += 1;

            let out = handlebars.render(FEED_TEMPLATE_FILENAME, &feed_ctx)?;
            // TODO use proper filename
            let out_path = feed_p_dir.join(format!("abc-{}.html", idx));
            info!("Saving feed page @ {}", &out_path.display());

            let mut f = File::create(out_path)?;
            f.write_all(out.as_bytes())?;
		}
		Ok(())
	}

	/// Render template for each feed.
	fn render_article_templates(&self) -> Result<()> {
		Ok(())
	}

	/// Save build time as text file.
	fn save_build_time(&self) -> Result<()> {
		let path = self.tmp_dir.join(BUILD_TIME_FILENAME);
		let mut file = File::create(path)?;
		file.write_all(format!("{}", self.context.build_time()).as_bytes())?;
		Ok(())
	}

	/// Save atom feed for all the feeds
	fn save_rss_channel(&self) -> Result<()> {
		let path = self.tmp_dir.join(RSS_FILE_FILENAME);
		let mut file = File::create(path)?;
		file.write_all(
			generate_rss_channel(
				self.context.options(),
				self.context.feeds(),
				true,
			)
			.as_bytes(),
		)?;
		Ok(())
	}

	/// Save atom feed for all the feeds
	fn save_opml(&self) -> Result<()> {
		let url = Url::parse(self.context.options().site_url.as_str());
		if url.is_err() {
			return Err(ConfigurationError::InvalidSiteUrl.into());
		}
		let path = self.tmp_dir.join(OPML_FILENAME);
		let mut file = File::create(path)?;
		file.write_all(
			generate_opml(
				self.context.options(),
				self.context.feeds(),
				&url.unwrap(),
			)
			.as_bytes(),
		)?;
		Ok(())
	}


	/// Save rss feeds containing query feed data used in the
	/// self referential data used in OPML channel.
	fn save_query_feed_channels(&self) -> Result<()> {
		let base_path = self.tmp_dir.join(SELF_REFERENTIAL_RSS_DIRNAME);
		_ = fs::create_dir(&base_path)?;
		let q_feeds: Vec<&Feed> = self
			.context
			.feeds()
			.into_iter()
			.filter(|f| f.is_query_feed() == true)
			.collect();
		for f in q_feeds {
			let path = base_path.join(format!("{}.xml", f.id()));
			info!("Saving channel data for query feed: {}", path.display());
			let mut file = File::create(path)?;
			file.write_all(
				generate_rss_channel(
					self.context.options(),
					&Vec::from([f.clone()]),
					false,
				)
				.as_bytes(),
			)?;
		}
		Ok(())
	}

	/// Generate json files for each feed.
	fn save_json_feeds(&self) -> Result<()> {
		_ = fs::create_dir(self.tmp_dir.join(FEEDS_DIRNAME))?;
		let mut f_list = FeedList::new();
		for f in self.context.feeds() {
			if !f.is_empty() && !f.is_hidden() {
				f_list.add_feed(&f);
				self.save_json_feed(f)?;
			}
		}
		self.save_json_feedlist(&f_list, String::from("feeds"))?;
		Ok(())
	}

	/// Save list of all feeds.
	fn save_json_feedlist(
		&self,
		feedlist: &FeedList,
		name: String,
	) -> Result<()> {
		if self.debug {
			self.save_feed_data(
				&name,
				serde_json::to_string_pretty(&feedlist)?.as_bytes(),
			)?;
		} else {
			self.save_feed_data(
				&name,
				serde_json::to_string(&feedlist)?.as_bytes(),
			)?;
		}
		Ok(())
	}

	/// Save single feed items.
	fn save_json_feed(&self, feed: &Feed) -> Result<()> {
		if feed.is_empty() || feed.is_hidden() {
			info!("Skipping saving feed: {:?}", feed);
			return Ok(());
		}
		let mut truncated = feed.clone();
		truncated.truncate_items();
		if self.debug {
			self.save_feed_data(
				truncated.id(),
				serde_json::to_string_pretty(&truncated)?.as_bytes(),
			)?;
			self.save_feed_data(
				&format!("{}_archive", feed.id()),
				serde_json::to_string_pretty(&feed)?.as_bytes(),
			)?;
		} else {
			self.save_feed_data(
				truncated.id(),
				serde_json::to_string(&truncated)?.as_bytes(),
			)?;
			self.save_feed_data(
				&format!("{}_archive", feed.id()),
				serde_json::to_string(&feed)?.as_bytes(),
			)?;
		}
		Ok(())
	}

	/// Save single feed data in tmp dir.
	fn save_feed_data(&self, name: &String, data: &[u8]) -> Result<()> {
		let feeds_dir = self.tmp_dir.join(FEEDS_DIRNAME);
		let path = feeds_dir.join(format!("{}.json", name));
		info!("Saving feed at path {}", path.display());
		let mut file = File::create(path)?;
		file.write_all(data)?;
		Ok(())
	}
}
