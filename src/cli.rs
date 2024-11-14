use std::path::PathBuf;
use clap::{value_parser, Arg, Command};

pub fn build_cli() -> Command{
    Command::new("cvee")
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
                        ).arg(
                            Arg::new("optimizer-type")
                                .short('t')
                                .long("optimizer-type")
                                .value_name("OPTIMIZER_TYPE")
                                .help("The type of optimizer to use [openai, sorting]")
                                .default_value("openai")
                                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                                .required(false),
                        ),
                ),
        )
}