use std::path::PathBuf;
use log;
use crate::skills::types::{OptimiseResult, Skill, SkillOptimizer};
use super::io;
use super::optimizers::sorting::SortingOptimizer;

pub async fn optimise(skill_file: &PathBuf, job_file: &PathBuf, output_file: &PathBuf)
                    -> Result<(), Box<dyn std::error::Error>> {

    if !skill_file.exists() {
        return Err(format!("input skill JSON file does not exist: {}", skill_file.display()).into());
    }

    if !job_file.exists() {
        return Err(format!("job description text file does not exist: {}", job_file.display()).into());
    }

    if let Some(parent) = output_file.parent() {
        if !parent.exists() {
            return Err(format!("output skill JSON parent directory does not exist: {}", parent.display()).into());
        }
    } else {
        return Err("output skill JSON has no parent directory".into());
    }

    log::info!("optimising skills based on job description");
    log::info!("loading skills from {}", skill_file.display());
    log::info!("loading job description from {}", job_file.display());


    let skills_json = io::deserialize_skills(&std::fs::read_to_string(skill_file)?)?;
    let job_description = std::fs::read_to_string(job_file)?;

    log::info!("optimising skills");
    let optimised_skills = optimize_skills(&skills_json, &job_description).await;


    log::info!("saving optimised skills to {}", output_file.display());

    Ok(())
}

pub async fn optimize_skills(skills: &[Skill], job_description: &str) -> OptimiseResult {
    let optimizer: &dyn SkillOptimizer = &SortingOptimizer;
    optimizer.optimise(skills, job_description).await
}


