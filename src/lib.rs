use anyhow::Context;
use chrono::Utc;
use log::{debug, error};
use serde_json::{self, json, Value};
use std::error::Error;
use std::fmt::Display;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Run a single lighthouse for given target.
pub fn run_lighthouse(
    app_dir: &str,
    domain: &str,
    target: &str,
    id: usize,
) -> Result<String, Box<dyn Error>> {
    let time_stamp = Utc::now().format("%Y-%m-%d_%H:%M:%S");
    let filepath = format!(
        "{app_dir}/{domain}/{time}.json",
        app_dir = &app_dir,
        domain = domain,
        time = time_stamp
    );
    let filepath = Path::new(&filepath);
    let output_path = filepath.to_str().unwrap().to_string();
    let mut cmd = Command::new("lighthouse");

    cmd.arg(&target)
        .arg("--output=json")
        .arg(format!("--output-path={}", &output_path))
        .arg("--chrome-flags=\"--headless\"");

    debug!(
        "{}",
        format!(
            "Running lighthouse itr={} for target={}, writing to file={}",
            id,
            target,
            filepath.to_str().unwrap(),
        )
    );

    let cmd_result = cmd.output().with_context(|| {
        "failed to execute lighthouse, make sure it's installed and accessible in PATH"
    })?;

    if !cmd_result.status.success() {
        io::stderr().write_all(&cmd_result.stderr)?;
        error!("failed to run lighthouse, make sure it's installed or that used option flags haven't been renamed");

        return Err(LighthouseGroupieError::InternalError.into());
    }

    Ok(output_path)
}

/// Aggregates all files for given target and creates a single output file that include selected target attributes.
pub fn create_result_aggregate(
    domain: &str,
    result_files: Vec<String>,
    timings_only: bool,
) -> Result<Value, Box<dyn Error>> {
    let mut fcp = vec![];
    let mut fmp = vec![];
    let mut tbt = vec![];
    let mut tti = vec![];
    let mut srt = vec![];
    let mut but = vec![];
    let mut rtt = vec![];
    let mut tbw = vec![];
    let mut tbc = vec![];
    let mut seo = vec![];
    let mut per = vec![];
    let mut bes = vec![];

    result_files
        .iter()
        .try_for_each(|filepath| -> Result<(), Box<dyn Error>> {
            debug!("collecting results for run={}", filepath);

            let result_file = std::fs::File::open(filepath)?;
            let json_data: Value = serde_json::from_reader(result_file)?;
            let fcp_v = json_data["audits"]["first-contentful-paint"]["numericValue"].as_f64();
            let fmp_v = json_data["audits"]["first-meaningful-paint"]["numericValue"].as_f64();
            let tbt_v = json_data["audits"]["total-blocking-time"]["numericValue"].as_f64();
            let tti_v = json_data["audits"]["interactive"]["numericValue"].as_f64();
            let srt_v = json_data["audits"]["server-response-time"]["numericValue"].as_f64();
            let but_v = json_data["audits"]["bootup-time"]["numericValue"].as_f64();
            let rtt_v = json_data["audits"]["network-rtt"]["numericValue"].as_f64();
            let tbw_v = json_data["audits"]["total-byte-weight"]["numericValue"].as_f64();
            let tbc_v = json_data["audits"]["total-byte-weight"]["details"]["items"]
                .as_array()
                .expect("failed to parse byte weight file count")
                .len();
            let seo_v = json_data["categories"]["seo"]["score"].as_f64();
            let per_v = json_data["categories"]["performance"]["score"].as_f64();
            let bes_v = json_data["categories"]["best-practices"]["score"].as_f64();

            fcp.push(fcp_v);
            fmp.push(fmp_v);
            tbt.push(tbt_v);
            tti.push(tti_v);
            srt.push(srt_v);
            but.push(but_v);
            rtt.push(rtt_v);
            tbw.push(tbw_v);
            tbc.push(tbc_v);
            seo.push(seo_v);
            per.push(per_v);
            bes.push(bes_v);

            Ok(())
        })?;

    let time_stamp = Utc::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    let json_result = match timings_only {
        false => json!({
          "domain": domain,
          "timeStamp": time_stamp,
          "firstContentfulPaint": fcp,
          "firstMeaningfulPaint": fmp,
          "totalBlockingTime": tbt,
          "timeToInteractive": tti,
          "serverResponseTime": srt,
          "bootupTime": but,
          "roundTripTime": rtt,
          "totalByteWeight": tbw,
          "totalFileCount": tbc,
          "performance": per,
          "seo": seo,
          "bestPractices": bes,
        }),
        true => json!({
          "timeStamp": time_stamp,
          "firstContentfulPaint": fcp,
          "firstMeaningfulPaint": fmp,
          "totalBlockingTime": tbt,
          "timeToInteractive": tti,
          "serverResponseTime": srt,
          "bootupTime": but,
          "roundTripTime": rtt,
          "totalByteWeight": tbw,
        }),
    };

    Ok(json_result)
}

#[derive(Debug, Clone)]
enum LighthouseGroupieError {
    InternalError,
}

impl Display for LighthouseGroupieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LighthouseGroupieError::InternalError => {
                write!(f, "Lighthouse failed internally for some reason...")
            }
        }
    }
}

impl Error for LighthouseGroupieError {}
