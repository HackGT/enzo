use crate::utils::{
    error::EnzoError,
    query::{AnswerKind, Question},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Section(String);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Instruction {
    Ask {
        question: String,
        answer: String,
        default: Option<String>,
        hints: Option<Vec<String>>,
    },
    Run(Vec<String>),
    Pipe {
        from: String,
        to: String,
    },
}

pub fn execute(instructions: &Vec<Instruction>) -> Result<(), EnzoError> {
    for instruction in instructions {
        execute_instruction(instruction)?;
    }
    Ok(())
}

// TODO remove pub
pub fn execute_instruction(instruction: &Instruction) -> Result<(), EnzoError> {
    match instruction {
        Instruction::Ask { question, .. } => {
            let question = Question {
                question: &question,
                default: None,
                hints: None,
                prefill: None,
            };
            let mut answer = AnswerKind::Single(String::new());
            question.ask(&mut answer);
            println!("The answer is {:#?}", answer);
        }
        _ => unimplemented!(),
    }
    Ok(())
}
