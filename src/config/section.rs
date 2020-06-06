use crate::utils::{
    error::EnzoError,
    query::{AnswerKind, Question},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::Command};

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
    let mut answers = HashMap::new();
    for instruction in instructions {
        execute_instruction(instruction, &mut answers)?;
    }
    Ok(())
}

// TODO remove pub
pub fn execute_instruction(
    instruction: &Instruction,
    answers: &mut HashMap<String, AnswerKind>,
) -> Result<(), EnzoError> {
    match instruction {
        Instruction::Ask {
            question, answer, ..
        } => {
            let question = Question {
                question: &question,
                default: None,
                hints: None,
                prefill: None,
            };
            let mut answer_kind = AnswerKind::Single(String::new());
            question.ask(&mut answer_kind);
            answers.insert(answer.to_string(), answer_kind);
        }
        Instruction::Run(commands) => {
            // TODO clean this up
            // TODO make it more robust with better error handling
            for command in commands {
                let mut it = command.split(" ");
                let cmd = it.next().unwrap();
                let mut args = vec![];
                for arg in it {
                    if arg.starts_with("$") {
                        if let Some(val) = answers.get(arg) {
                            match val {
                                AnswerKind::Single(s) => args.push(s.as_str()),
                                _ => args.push(arg),
                            }
                        } else {
                            args.push(arg);
                        }
                    } else {
                        args.push(arg);
                    }
                }
                Command::new(cmd).args(&args).status()?;
            }
        }
        _ => {}
    }
    Ok(())
}
