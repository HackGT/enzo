use crate::utils::{
    error::{EnzoError, EnzoErrorKind},
    query::{AnswerKind, Question},
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::prelude::*, path::PathBuf, process::Command};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Section(pub String);

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
        template: String,
        output: String,
    },
}

#[derive(Debug)]
pub struct ExecutionContext {
    pub repo: PathBuf,
    pub curr: PathBuf,
    pub remote: String,
}

pub fn execute(instructions: &Vec<Instruction>, ctx: &ExecutionContext) -> Result<(), EnzoError> {
    let mut answers = HashMap::new();
    answers.insert("remote".into(), AnswerKind::Single(ctx.remote.clone()));
    for instruction in instructions {
        execute_instruction(instruction, &mut answers, ctx)?;
    }
    Ok(())
}

fn execute_instruction(
    instruction: &Instruction,
    answers: &mut HashMap<String, AnswerKind>,
    ctx: &ExecutionContext,
) -> Result<(), EnzoError> {
    match instruction {
        Instruction::Ask {
            question,
            answer,
            default,
            ..
        } => {
            let question = Question {
                question: &question,
                default: default.as_ref().map(|i| i.as_str()),
                hints: None,
                prefill: None,
            };
            let mut answer_kind = AnswerKind::Single(String::new());
            question.ask(&mut answer_kind);
            answers.insert(answer.to_string(), answer_kind);
        }
        Instruction::Run(commands) => {
            for command in commands {
                println!(
                    "$ {}",
                    ansi_term::Color::White.dimmed().paint(command.clone())
                );
                run_command(command, answers, ctx)?;
            }
        }
        Instruction::Pipe { template, output } => {
            let handlebars = Handlebars::new();

            let mut file = File::open(process_arg(template, answers, ctx))?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;

            let out = match handlebars.render_template(&buffer, &answers) {
                Ok(s) => s,
                Err(e) => return Err(EnzoError::new(format!("{}", e), EnzoErrorKind::FatalError)),
            };

            let mut file = File::create(process_arg(output, answers, ctx))?;
            file.write_all(out.as_bytes())?;
        }
    }
    Ok(())
}

fn run_command(
    command: &String,
    answers: &HashMap<String, AnswerKind>,
    ctx: &ExecutionContext,
) -> Result<(), EnzoError> {
    let mut it = command.split(" ");
    let cmd = process_arg(it.next().unwrap(), answers, ctx);
    let mut args = vec![];

    for arg in it {
        args.push(process_arg(&arg, answers, ctx));
    }

    Command::new(cmd).args(&args).status()?;
    Ok(())
}

fn process_arg(arg: &str, answers: &HashMap<String, AnswerKind>, ctx: &ExecutionContext) -> String {
    if let Some(kind) = answers.get(arg) {
        match kind {
            AnswerKind::Single(val) => val.clone(),
            _ => unimplemented!(),
        }
    } else {
        arg.replace("$repo", ctx.repo.to_str().unwrap())
            .replace("$curr", ctx.curr.to_str().unwrap())
    }
}
