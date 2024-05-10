use super::Context;
use crate::handlers::Handler;
use regex::{Error, Regex, RegexBuilder};
use tes3::esp::TES3Object;

pub struct ToDoValidator {
    todo: Regex,
}

impl Handler<'_> for ToDoValidator {
    fn on_scriptline(
        &mut self,
        _: &Context,
        record: &TES3Object,
        _: &str,
        comment: &str,
        topic: &tes3::esp::Dialogue,
    ) {
        if self.todo.is_match(comment) {
            if let TES3Object::Script(script) = record {
                println!("Script {} contains comment {}", script.id, comment);
            } else if let TES3Object::Info(info) = record {
                println!(
                    "Info {} in topic {} contains comment {}",
                    info.id, topic.id, comment
                );
            }
        }
    }
}

impl ToDoValidator {
    pub fn new() -> Result<Self, Error> {
        let todo = RegexBuilder::new(r"(^(todo|fixme|to do|fix me))|(^|\s)merge")
            .case_insensitive(true)
            .build()?;
        return Ok(Self { todo });
    }
}
