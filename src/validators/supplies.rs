use std::collections::HashMap;

use super::Context;
use crate::{handlers::Handler, util::get_cell_name};
use tes3::esp::{Cell, Reference};

include!(concat!(env!("OUT_DIR"), "/gen_supplies.rs"));

pub struct SupplyChestValidator {
    chests: HashMap<&'static str, &'static str>,
}

const ALL_RANKS: u32 = 4294967295;

impl Handler<'_> for SupplyChestValidator {
    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        id: &String,
        _: &Vec<&Reference>,
        _: usize,
    ) {
        if let Some(faction) = self.chests.get(id.as_str()) {
            if !reference
                .owner_faction
                .as_ref()
                .map(|f| f.eq_ignore_ascii_case(faction))
                .unwrap_or(false)
            {
                println!(
                    "Cell {} contains {} not owned by the {}",
                    get_cell_name(record),
                    reference.id,
                    faction
                );
            } else {
                let rank = reference.owner_faction_rank.unwrap_or(0);
                if rank != 0 && rank != ALL_RANKS {
                    println!(
                        "Cell {} contains {} not available to all ranks",
                        get_cell_name(record),
                        reference.id
                    );
                }
            }
        }
    }
}

impl SupplyChestValidator {
    pub fn new() -> Self {
        return Self {
            chests: get_supplies_data(),
        };
    }
}
