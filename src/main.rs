mod controller;
mod paths;
mod args;
mod opts;
mod urls;
mod feed;
mod feed_item;
mod builder;
mod template;


use crate::controller::Controller;

fn main() {
    let controller = match Controller::init() {
        Err(e)=> panic!("{:?}", e),
        Ok(c) => c,
    };
    let results = match controller.process_feeds() {
        Err(e)=> panic!("{:?}", e),
        _ => println!("ok")
    };

    // println!("{:?}", res)
    // TODO: add logger
    //
    // Parse urls file
    // parse tags/filters/queries
    // merge results with those form db
    // generate actual feeds
    // use the feeds to generate templates
    // 
    // 
    // 
    // 
}
