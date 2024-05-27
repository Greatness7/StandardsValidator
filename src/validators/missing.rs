use super::Context;
use crate::{handlers::Handler, util::is_marker};
use tes3::esp::{LightFlags, TES3Object};

pub struct FieldValidator {}

fn check(typename: &str, id: &str, field: &str, value: &str) {
    if !value.is_empty() && !value.trim().is_empty() {
        if field != "name" && !value.contains('.') {
            println!("{} {} has invalid {} {}", typename, id, field, value);
        }
        return;
    }
    println!("{} {} has a missing {}", typename, id, field);
}

impl Handler<'_> for FieldValidator {
    fn on_record(&mut self, _: &Context, record: &TES3Object, typename: &str, id: &str) {
        match record {
            TES3Object::Activator(r) => {
                check(typename, id, "mesh", &r.mesh);
            }
            TES3Object::Alchemy(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Apparatus(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Armor(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Book(r) => {
                if !is_marker(r) {
                    check(typename, id, "icon", &r.icon);
                    check(typename, id, "mesh", &r.mesh);
                    check(typename, id, "name", &r.name);
                }
            }
            TES3Object::Clothing(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Container(r) => {
                check(typename, id, "mesh", &r.mesh);
            }
            TES3Object::Creature(r) => {
                check(typename, id, "mesh", &r.mesh);
            }
            TES3Object::Door(r) => {
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Ingredient(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Light(r) => {
                if r.data.flags.contains(LightFlags::CAN_CARRY) {
                    check(typename, id, "icon", &r.icon);
                    check(typename, id, "mesh", &r.mesh);
                    check(typename, id, "name", &r.name);
                }
            }
            TES3Object::Lockpick(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::MiscItem(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Npc(r) => {
                check(typename, id, "name", &r.name);
            }
            TES3Object::Probe(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::RepairItem(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            TES3Object::Static(r) => {
                check(typename, id, "mesh", &r.mesh);
            }
            TES3Object::Weapon(r) => {
                check(typename, id, "icon", &r.icon);
                check(typename, id, "mesh", &r.mesh);
                check(typename, id, "name", &r.name);
            }
            _ => {}
        }
    }
}
