
use crate::skills::types::{OptimiseFuture, OptimiseResult, Skill, SkillOptimizer};

pub struct SortingOptimizer;

impl SkillOptimizer for SortingOptimizer {
    fn optimise<'a>(
        &'a self,
        skills: &'a [Skill],
        job_description: &'a str,
    ) -> OptimiseFuture<'a> {
        Box::pin(async move {
            let mut sorted_skills:Vec<Skill> = skills.to_vec();
            sorted_skills.sort_by(|a:&Skill, b:&Skill| a.name.cmp(&b.name));
            Ok(sorted_skills)
        })
    }
}