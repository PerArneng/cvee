use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

/// Represents a skill with an ID, name, level, and associated roles.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub level: u8,
    pub roles: Vec<String>,
}
pub type OptimiseResult = Result<Vec<Skill>, Box<dyn std::error::Error>>;
pub type OptimiseFuture<'a> = Pin<Box<dyn Future<Output = OptimiseResult> + Send + 'a>>;

pub trait SkillOptimizer {
    fn optimise<'a>(
        &'a self,
        skills: &'a [Skill],
        job_description: &'a str,
    ) -> OptimiseFuture<'a>;
}

#[derive(Debug)]
pub enum OptimizerType {
    OpenAI,
    Sorting,
}

// Implement Display for OptimizerType to convert enum to string
impl fmt::Display for OptimizerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            OptimizerType::OpenAI => "OpenAI",
            OptimizerType::Sorting => "Sorting",
        };
        write!(f, "{}", s)
    }
}

// Implement FromStr for OptimizerType with a generic &'static str error
impl FromStr for OptimizerType {
    type Err = String; // Use &'static str as a simple error type

    fn from_str(string_type: &str) -> Result<Self, Self::Err> {
        match string_type.to_lowercase().as_str() {
            "openai" => Ok(OptimizerType::OpenAI),
            "sorting" => Ok(OptimizerType::Sorting),
            _ => Err(format!("Unknown optimizer type: {}", string_type)), // Return a static string error
        }
    }
}