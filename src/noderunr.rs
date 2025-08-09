#![allow(unused)]

/* Initailize (external) libraries. */
use clap::{Arg, Command};
use clap::Parser;
use human_panic::setup_panic;
use log::{info, warn};
use serde_json::json;

/* Import modules. */
mod api;
mod cmd;
mod comm;
mod crypto;
mod node;
mod privacy;
mod ui;
mod utils;

// #[derive(Parser)]
// struct Cli {
//     seed_phrase: String,

//     #[clap(parse(from_os_str))]
//     config_path: std::path::PathBuf,
// }
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[clap(short, long, value_parser, default_value = "Satoshi")]
   name: String,

   /// Number of times to greet
   #[clap(short, long, value_parser, default_value_t = 1)]
   count: u8,
}

/**
 * Main
 *
 * Entry point for this application.
 */
fn main() {
    /* Setup (human) panic. */
    setup_panic!();

    /* Display welcome banner. */
    ui::welcome::banner_alt();

    // let args = Cli::parse();
    // println!("  (Private) seed phrase is : {}", args.seed_phrase);
    // println!("     Configuration path is : {}\n", args.config_path.display());

    // let args = Args::parse();

    /* Set version. */
    let version = noderunr::get_version();

    /* Handle application arguments. */
    let matches = Command::new("NodΞRunr")
        .version(version)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("id")
            .required(false)
            // .takes_value(true)
            .index(1)
            .help("connect to an existing session"))
        .arg(Arg::new("seed")
            .required(false)
            // .takes_value(true)
            .index(2)
            .help("12 or 24 word seed phrase"))
        .arg(Arg::new("url")
            .required(false)
            // .takes_value(true)
            .index(3)
            .help("url of node viewer"))
        .get_matches();

    /* Handle session id. */
    // let sessionid = matches
    //     .value_of("id")
    //     .unwrap_or("");
    // if !sessionid.is_empty() {
    //     println!("  Session ID is [ {} ]\n", sessionid);
    // }
    // if let Some(sessionid) = matches.get_one::<String>("id") {
    //     let sessionid_string: String = sessionid.to_string();
    //     println!("  Session ID is [ {} ]\n", sessionid_string);
    // }

    /* Handle (master) seed. */
    // let seed = matches
    //     .value_of("seed")
    //     .unwrap_or("");
    // if !seed.is_empty() {
    //     println!("  Master seed is [ {} ]\n", seed);
    // }
    // if let Some(seed) = matches.get_one::<String>("seed") {
    //     let seed_string: String = seed.to_string();
    //     println!("  Master seed is [ {} ]\n", seed_string);
    // }

    /* Handle URL. */
    // let url = matches
    //     .value_of("url")
    //     .unwrap_or("");
    // if !url.is_empty() {
    //     println!("  URL is [ {} ]\n", url);
    // }
    // if let Some(url) = matches.get_one::<String>("url") {
    //     let url_string: String = url.to_string();
    //     println!("  URL is [ {} ]\n", url_string);
    // }

    // for _ in 0..args.count {
    //     println!("  Hi there {}!\n", args.name)
    // }

    // let cmd = clap::Command::new("noderunr")
    // .bin_name("noderunr")
    // .subcommand_required(true)
    // .subcommand(
    //     clap::command!("example").arg(
    //         clap::arg!(--"manifest-path" <PATH>)
    //             .value_parser(clap::value_parser!(std::path::PathBuf)),
    //     ),
    // );
    // let matches = cmd.get_matches();
    // let matches = match matches.subcommand() {
    //     Some(("example", matches)) => matches,
    //     _ => unreachable!("clap should ensure we don't get here"),
    // };
    // let manifest_path = matches.get_one::<std::path::PathBuf>("manifest-path");
    // println!("{:?}", manifest_path);

    // let result = std::fs::read_to_string(&args.config_path);
    // match result {
    //     Ok(content) => {
    //         for line in content.lines() {
    //             if line.contains(&args.pattern) {
    //                 println!("  Look! We found a line ---> {}", line);
    //             }
    //         }

    //         println!(); // empty line / spacer
    //     }

    //     Err(error) => {
    //         println!("Oops! Could not read the file you specified.\n\n[ {} ]", error);
    //     }
    // }

    // let my_sqr = crypto::math::sqr(4.0);
    // println!("  Square is {}\n", my_sqr);

    // env_logger::init();
    // info!("starting up");
    // warn!("oops, nothing implemented!\n");

    // let node = FederationNode {
    //     id: String::from("190171ee-ac37-4e05-988b-a7e683c1e5d3"),
    //     owner: String::from("Shomari"),
    //     title: String::from("Awesome Node # 1337"),
    //     createdAt: String::from("Tuesday"),
    // }
    // println!("  Node ID is: {}\n", node.get_id);

    node::session::new();

    // panic!("Oops! What happened??");

    // comm::make_request();

    // utils::remote::get_ip();

    // commander::sys::ping()

    // utils::remote::start_download();
}