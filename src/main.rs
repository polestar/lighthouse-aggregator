use anyhow::Result;
use core::panic;
use dirs::home_dir;
use indicatif::{ProgressBar, ProgressStyle};
use lighthouse_groupie::{create_result_aggregate, run_lighthouse};
use log::{debug, error};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::{fs, process};
use structopt::StructOpt;
use url::Url;

/// Perform one or more lighthouse runs against target site and outputs the aggregated result.
/// Individual lighthouse runs and aggregate results are stored in `~/.lighthouse-groupie`.
///
/// example: lighthouse-groupie https://www.google.se
///
#[derive(StructOpt, Debug)]
struct Cli {
    /// The target site to test.
    target: String,

    /// Additional headers that will be added to each request.
    #[structopt(short, long)]
    headers: Option<String>,

    /// The number of runs that should be performed against the target.
    #[structopt(short, long, default_value = "30")]
    count: usize,

    /// Aggregate output path.
    #[structopt(short, long)]
    output: Option<String>,

    /// Ignore scoring and other assets that usually stay static between runs.
    #[structopt(short, long)]
    timings_only: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::from_args();
    let app_dir = format!(
        "{}/{}",
        home_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .expect("failed to read home dir"),
        ".lighthouse-groupie"
    );
    let target = match Url::parse(&args.target) {
        Ok(d) => d,
        Err(_) => panic!("cant't parse {} as a proper URL", args.target),
    };
    let domain = target.domain().unwrap();
    let target = args.target.clone();

    debug!("using app_dir={}", &app_dir);
    debug!("args={:?}", args);
    fs::create_dir_all(format!("{}/{}", &app_dir, domain))
        .expect("Failed to create application directory");

    let mut result_files = vec![];

    let loader = ProgressBar::new(args.count as u64);
    loader.set_style(
        ProgressStyle::default_bar()
            .template("{spinner} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}")
            .progress_chars("##-"),
    );

    loader.inc(0);
    loader.enable_steady_tick(60);

    for i in 0..args.count {
        let filepath = match run_lighthouse(&app_dir, domain, &target, i) {
            Ok(o) => o,
            Err(err) => {
                error!("{}", err);
                process::exit(1);
            }
        };

        result_files.push(filepath);

        loader.inc(1);
    }

    loader.finish();

    let res = create_result_aggregate(domain, result_files, args.timings_only)
        .expect("failed to parse and collect result aggregate");

    if let Some(output_path) = args.output {
        let target = Path::new(&output_path);
        let mut fd = File::create(&target)?;

        serde_json::to_writer(&fd, &res)?;
        fd.write_all("\n".as_bytes())?;

        debug!("wrote aggregate to {:?}", target);
    } else {
        serde_json::to_writer_pretty(io::stdout(), &res)?;
        println!();
    }

    Ok(())
}
