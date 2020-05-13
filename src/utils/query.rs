use ansi_term::Color::{Green, White, Yellow};
use std::fmt;

pub struct Question<'a> {
    question: &'a str,
    default: Option<&'a str>,
    prefill: Option<&'a str>,
    hints: Option<Vec<&'a str>>,
}

impl<'a> Question<'a> {
    pub fn new(
        question: &'a str,
        default: Option<&'a str>,
        prefill: Option<&'a str>,
        hints: Option<Vec<&'a str>>,
    ) -> Self {
        Question {
            question,
            default,
            prefill,
            hints,
        }
    }

    pub fn new_question(question: &'a str) -> Self {
        Question {
            question,
            default: None,
            prefill: None,
            hints: None,
        }
    }

    pub fn query(&self) -> String {
        String::new()
    }
}

// TODO add colors
impl<'a> fmt::Display for Question<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut q = format!("{} {} ", Green.bold().paint("?"), self.question);
        if let Some(default) = self.default {
            q.push_str(format!("(default {})", White.dimmed().paint(default)).as_str());
        }
        if let Some(hints) = &self.hints {
            for hint in hints {
                q.push_str(format!("\nhint: {}", White.dimmed().paint(hint.to_string())).as_str());
            }
        }
        q.push_str("\n> ");
        if let Some(prefill) = self.prefill {
            q.push_str(format!("{}", Yellow.paint(prefill)).as_str());
        }
        write!(f, "{}", q)
    }
}
