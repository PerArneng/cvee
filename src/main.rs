mod cli;
mod skills;
mod logging;

use std::path::PathBuf;
use log;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    logging::init()?;

    log::info!("starting up");

    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("skills") {
        if let Some(matches) = matches.subcommand_matches("optimise") {
            let skill_file = matches.get_one::<PathBuf>("input-skill-json").unwrap();
            let job_ad_file = matches.get_one::<PathBuf>("job-txt").unwrap();
            let output_file = matches.get_one::<PathBuf>("output-skill-json").unwrap();

            let result = skills::skills_optimise(skill_file, job_ad_file, output_file);
            if let Err(e) = result {
                log::error!("error: {}", e);
            }
        }
    }

    Ok(())
}