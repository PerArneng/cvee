mod cli;
mod skills;
mod logging;

use std::path::PathBuf;
use std::str::FromStr;
use log;
use skills::optimise;

use crate::skills::types::{OptimiseResult, OptimizerType, Skill, SkillOptimizer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    logging::init()?;

    log::info!("starting up");

    let matches = cli::build_cli().get_matches();

    if let Some(matches) = matches.subcommand_matches("skills") {
        if let Some(matches) = matches.subcommand_matches("optimise") {
            let skill_file = matches.get_one::<PathBuf>("input-skill-json").unwrap();
            let job_ad_file = matches.get_one::<PathBuf>("job-txt").unwrap();
            let output_file = matches.get_one::<PathBuf>("output-skill-json").unwrap();
            let optimizer_type_string = matches.get_one::<String>("optimizer-type").unwrap();

            let optimizer_type = OptimizerType::from_str(&optimizer_type_string)?;

            let result = optimise::optimise(skill_file, job_ad_file, output_file, optimizer_type).await;
            if let Err(e) = result {
                log::error!("error: {}", e);
            }
        }
    }

    Ok(())
}