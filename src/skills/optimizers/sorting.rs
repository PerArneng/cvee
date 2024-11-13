use std::future::Future;
use std::pin::Pin;
use crate::skills::types::{OptimiseResult, Skill, SkillOptimizer};

pub struct SortingOptimizer;

impl SkillOptimizer for SortingOptimizer {
    fn optimise<'a>(
        &'a self,
        skills: &'a [Skill],
        job_description: &'a str,
    ) -> Pin<Box<dyn Future<Output = OptimiseResult> + Send + 'a>> {
        Box::pin(async move {
            let mut sorted_skills:Vec<Skill> = skills.to_vec();
            sorted_skills.sort_by(|a:&Skill, b:&Skill| a.name.cmp(&b.name));
            Ok(sorted_skills)
        })
    }
}