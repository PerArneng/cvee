use std::path::PathBuf;
use clap::{value_parser, Arg, Command};

fn main() {
    let matches = Command::new("cvee")
        .version("0.1.0")
        .about("A CV command line tool written in Rust")
        .subcommand(
            Command::new("skills")
                .about("Manage skills")
                .subcommand(
                    Command::new("optimise")
                        .about("Optimise skills based on job description")
                        .arg(
                            Arg::new("input-skill-json")
                                .short('i')
                                .long("input-skill-json")
                                .value_name("INPUT_SKILL_JSON")
                                .help("Input skill JSON file")
                                .value_parser(value_parser!(PathBuf))
                                .required(true),
                        )
                        .arg(
                            Arg::new("job-txt")
                                .short('j')
                                .long("job-txt")
                                .value_name("JOB_TXT")
                                .help("Job description text file")
                                .value_parser(value_parser!(PathBuf))
                                .required(true),
                        )
                        .arg(
                            Arg::new("output-skill-json")
                                .short('o')
                                .long("output-skill-json")
                                .value_name("OUTPUT_SKILL_JSON")
                                .help("Output skill JSON file")
                                .value_parser(value_parser!(PathBuf))
                                .required(true),
                        ),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("skills") {
        if let Some(matches) = matches.subcommand_matches("optimise") {
            let input_skill_json = matches.get_one::<PathBuf>("input-skill-json").unwrap();
            let job_txt = matches.get_one::<PathBuf>("job-txt").unwrap();
            let output_skill_json = matches.get_one::<PathBuf>("output-skill-json").unwrap();

            println!("Input skill JSON: {}", input_skill_json.display());
            println!("Job description text file: {}", job_txt.display());
            println!("Output skill JSON: {}", output_skill_json.display());
        }
    }
}