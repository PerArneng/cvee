

use async_openai::types::{
    ChatCompletionMessageToolCall, ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage, ChatCompletionRequestToolMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionToolArgs, ChatCompletionToolType,
    FunctionObjectArgs,
};
use async_openai::{types::CreateChatCompletionRequestArgs, Client};


use crate::skills::types::{OptimiseFuture, OptimiseResult, Skill, SkillOptimizer};

pub struct OpenAISkillOptimizer;

impl SkillOptimizer for OpenAISkillOptimizer {
    fn optimise<'a>(
        &'a self,
        skills: &'a [Skill],
        job_description: &'a str,
    ) -> OptimiseFuture<'a> {
        Box::pin(async move {

            Ok(vec![Skill {
                id: "1".to_string(),
                name: "Skill 1".to_string(),
                level: 1,
                roles: vec!["Role 1".to_string()],
            }])
        })
    }
}