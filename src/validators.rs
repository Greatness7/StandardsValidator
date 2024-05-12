pub mod autocalc;
pub mod books;
pub mod cells;
pub mod classes;
pub mod corpse;
pub mod dialogue;
pub mod doors;
pub mod duplicates;
pub mod ids;
pub mod keys;
pub mod leveled;
pub mod magic;
pub mod missing;
pub mod npc;
pub mod orphans;
pub mod persistent;
pub mod scripts;
pub mod services;
pub mod soundgens;
pub mod supplies;
pub mod todo;
pub mod travel;
pub mod unicode;
pub mod uniques;

use crate::{
    context::Context,
    handlers::{Handler, Handlers},
};
use clap::ArgMatches;
use std::error::Error;
use tes3::esp::{Dialogue, FixedString, ObjectFlags, TES3Object};

pub struct Validator<'a> {
    handlers: Handlers<'a>,
    context: Context,
}

impl<'a> Validator<'a> {
    pub fn new<'b>(context: Context, args: &ArgMatches) -> Result<Validator<'b>, Box<dyn Error>> {
        return Ok(Validator {
            handlers: Handlers::new(&context, args)?,
            context,
        });
    }

    pub fn validate(&mut self, records: &'a Vec<TES3Object>) {
        let dummy = Dialogue {
            flags: ObjectFlags::empty(),
            id: String::new(),
            kind: None,
        };
        let mut current_topic = &dummy;
        for record in records {
            match record {
                TES3Object::Activator(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Alchemy(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Apparatus(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Armor(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Birthsign(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Bodypart(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Book(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Cell(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    let refs: Vec<_> = r.references.values().collect();
                    for (i, reference) in refs.iter().enumerate() {
                        self.handlers.on_cellref(
                            &self.context,
                            r,
                            reference,
                            &reference.id.to_ascii_lowercase(),
                            &refs,
                            i,
                        );
                    }
                }
                TES3Object::Class(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Clothing(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Container(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::Creature(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::Dialogue(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    current_topic = r;
                }
                TES3Object::Door(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Enchanting(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Faction(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::GameSetting(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::GlobalVariable(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Header(_) => {}
                TES3Object::Info(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    self.handlers.on_info(&self.context, r, current_topic);
                    self.on_script(record, &r.script_text, current_topic);
                }
                TES3Object::Ingredient(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Landscape(_) => {}
                TES3Object::LandscapeTexture(_) => {}
                TES3Object::LeveledCreature(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    self.on_leveled(record, &r.creatures);
                }
                TES3Object::LeveledItem(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    self.on_leveled(record, &r.items);
                }
                TES3Object::Light(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Lockpick(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::MagicEffect(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &String::new())
                }
                TES3Object::MiscItem(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Npc(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    self.on_inventory(record, &r.inventory);
                }
                TES3Object::PathGrid(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &String::new())
                }
                TES3Object::Probe(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Race(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Region(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::RepairItem(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Script(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id);
                    self.on_script(record, &r.script_text, &dummy);
                }
                TES3Object::Skill(_) => {}
                TES3Object::Sound(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::SoundGen(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Spell(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::StartScript(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Static(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
                TES3Object::Weapon(r) => {
                    self.handlers
                        .on_record(&self.context, record, r.type_name(), &r.id)
                }
            }
        }
        self.handlers.on_end(&self.context);
    }

    fn on_leveled(&mut self, record: &TES3Object, entries: &Option<Vec<(String, u16)>>) {
        if let Some(list) = entries {
            for entry in list {
                self.handlers.on_leveled(&self.context, record, entry);
            }
        }
    }

    fn on_inventory(
        &mut self,
        record: &TES3Object,
        inventory: &Option<Vec<(i32, FixedString<32>)>>,
    ) {
        if let Some(list) = inventory {
            for entry in list {
                self.handlers.on_inventory(&self.context, record, entry);
            }
        }
    }

    fn on_script(&mut self, record: &TES3Object, script: &Option<String>, topic: &Dialogue) {
        if let Some(text) = script {
            let empty = "";
            for line in text.trim().split('\n') {
                let code: &str;
                let comment: &str;
                match line.split_once(';') {
                    Some(s) => {
                        code = s.0.trim();
                        comment = s.1;
                    }
                    None => {
                        code = line.trim();
                        comment = empty;
                    }
                }
                if !code.is_empty() || !comment.is_empty() {
                    self.handlers.on_scriptline(
                        &self.context,
                        record,
                        &code.to_ascii_lowercase(),
                        comment,
                        topic,
                    );
                }
            }
        }
    }
}