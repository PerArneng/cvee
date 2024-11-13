use std::future::Future;
use std::pin::Pin;
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

pub trait SkillOptimizer {
    fn optimise<'a>(
        &'a self,
        skills: &'a [Skill],
        job_description: &'a str,
    ) -> Pin<Box<dyn Future<Output = OptimiseResult> + Send + 'a>>;
}