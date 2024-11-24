mod controller;
mod config;
mod args;
mod opts;
mod urls;
mod feed;
mod feed_item;


use crate::controller::Controller;

fn main() {
    let controller = match Controller::init() {
        Err(e)=> panic!("{:?}", e),
        Ok(c) => c,
    };
    let results = match controller.process() {
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
