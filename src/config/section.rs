use crate::utils::{
    error::{EnzoError, EnzoErrorKind},
    query::{AnswerKind, Question},
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, fs::File, io::prelude::*, path::PathBuf, process::Command};

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
        template: PathBuf,
        output: String,
    },
}

pub fn execute(instructions: &Vec<Instruction>) -> Result<(), EnzoError> {
    let mut answers = HashMap::new();
    if let Some((_, v)) = env::vars().find(|(k, _)| k == "repo") {
        answers.insert("$repo".to_string(), AnswerKind::Single(v));
    }
    for instruction in instructions {
        execute_instruction(instruction, &mut answers)?;
    }
    Ok(())
}

// TODO remove pub
fn execute_instruction(
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
                    if let Some(val) = answers.get(arg) {
                        match val {
                            AnswerKind::Single(s) => args.push(s.as_str()),
                            _ => args.push(arg),
                        }
                    } else {
                        args.push(arg);
                    }
                }
                Command::new(cmd).args(&args).status()?;
            }
        }
        Instruction::Pipe { template, output } => {
            let handlebars = Handlebars::new();
            let mut file = File::open(template)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            let out = match handlebars.render_template(&buffer, &answers) {
                Ok(s) => s,
                Err(e) => return Err(EnzoError::new(format!("{}", e), EnzoErrorKind::FatalError)),
            };

            let mut output = output.clone();
            if let Some(AnswerKind::Single(val)) = answers.get("$repo") {
                output = output.replace("$repo", val);
            }

            let path = PathBuf::from(output);
            let mut file = File::create(&path)?;
            file.write_all(out.as_bytes())?;
        }
    }
    Ok(())
}
