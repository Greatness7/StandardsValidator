use super::Context;
use crate::{
    handlers::Handler,
    util::{get_cell_grid, CELL_SIZE},
};
use std::collections::{HashMap, HashSet};
use tes3::esp::{Cell, PathGrid, PathGridPoint, Reference, TES3Object};

include!(concat!(env!("OUT_DIR"), "/gen_broken.rs"));

const MAX_Z: f32 = 64000.;
const MIN_Z: f32 = -32000.;
const MAX_SAFE_INT: f32 = 9007199254740991.;
const MIN_SAFE_INT: f32 = -9007199254740991.;
const FLAG_CELL_WATER: u32 = 2;
const BLACK_SQUARES: [&str; 4] = [
    "in_lava_blacksquare",
    "t_aid_blackcircle_01",
    "t_aid_blackcircle_02",
    "t_aid_blacktriangle_01",
];

pub struct CellValidator {
    seen: HashSet<String>,
    broken: HashMap<&'static str, &'static str>,
}

fn get_cell_name(pathgrid: &PathGrid) -> String {
    if let Some(data) = &pathgrid.data {
        let (x, y) = data.grid;
        if x != 0 || y != 0 {
            if let Some(cell) = &pathgrid.cell {
                return format!("{} {},{}", cell, x, y);
            }
            return format!("{},{}", x, y);
        }
    }
    if let Some(cell) = &pathgrid.cell {
        return cell.clone();
    }
    return String::new();
}

fn get_point_coords(point: &PathGridPoint, record: &PathGrid) -> String {
    let [x_pos, y_pos, _] = point.location;
    let location = format!("[{}, {}]", x_pos, y_pos);
    if let Some(data) = &record.data {
        let (x, y) = data.grid;
        if x != 0 || y != 0 {
            let ext_x = x_pos + x * CELL_SIZE as i32;
            let ext_y = y_pos + y * CELL_SIZE as i32;
            return format!("{} ({}, {})", location, ext_x, ext_y);
        }
    }
    return location;
}

fn get_water_height(cell: &Cell) -> Option<f32> {
    if cell.is_exterior() {
        return Some(-1.);
    } else if (cell.data.flags & FLAG_CELL_WATER) != 0 {
        return cell.water_height;
    }
    return None;
}

impl Handler<'_> for CellValidator {
    fn on_record(&mut self, context: &Context, record: &TES3Object, _: &str, _: &String) {
        match record {
            TES3Object::Cell(cell) => {
                if cell.is_interior()
                    && cell
                        .atmosphere_data
                        .as_ref()
                        .map(|d| d.fog_density == 0f32)
                        .unwrap_or(false)
                    && !context.projects.iter().any(|p| p.matches(&cell.id))
                {
                    println!(
                        "Cell {} has a fog density of 0",
                        crate::util::get_cell_name(cell)
                    );
                }
            }
            TES3Object::PathGrid(pathgrid) => {
                if let Some(points) = &pathgrid.points {
                    if points.is_empty() {
                        return;
                    }
                    let mut connected: HashSet<u32> = HashSet::new();
                    if let Some(connections) = &pathgrid.connections {
                        connected.extend(connections);
                    }
                    for (i, point) in points.into_iter().enumerate() {
                        if point.connection_count > 0 {
                            connected.insert(i as u32);
                        }
                        for other_point in points[i + 1..].into_iter() {
                            if point
                                .location
                                .into_iter()
                                .enumerate()
                                .all(|(index, l)| l == other_point.location[index])
                            {
                                println!(
                                    "PathGrid {} contains duplicate node at {}",
                                    get_cell_name(pathgrid),
                                    get_point_coords(point, pathgrid)
                                );
                                break;
                            }
                        }
                    }
                    if points.len() != connected.len() {
                        for (i, point) in points.iter().enumerate() {
                            if !connected.contains(&(i as u32)) {
                                println!(
                                    "PathGrid {} contains unconnected node at {}",
                                    get_cell_name(pathgrid),
                                    get_point_coords(point, pathgrid)
                                );
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn on_cellref(
        &mut self,
        _: &Context,
        record: &Cell,
        reference: &Reference,
        id: &String,
        _: &Vec<&Reference>,
        _: usize,
    ) {
        if !reference.deleted.unwrap_or(false) && !record.is_interior() {
            let (x, y) = record.data.grid;
            let x_bound = CELL_SIZE * x as f64;
            let y_bound = CELL_SIZE * y as f64;
            let x_pos = reference.translation[0] as f64;
            let y_pos = reference.translation[1] as f64;
            let z_pos = reference.translation[2];
            if reference
                .translation
                .iter()
                .any(|coord| !coord.is_finite() || *coord > MAX_SAFE_INT || *coord < MIN_SAFE_INT)
                || z_pos < MIN_Z
                || z_pos > MAX_Z
            {
                println!(
                    "Cell {} contains far out reference {} at [{}, {}, {}]",
                    crate::util::get_cell_name(record),
                    reference.id,
                    x_pos,
                    y_pos,
                    z_pos
                );
            } else if x_pos < x_bound
                || y_pos < y_bound
                || x_pos >= x_bound + CELL_SIZE
                || y_pos >= y_bound + CELL_SIZE
            {
                let (actual_x, actual_y) = get_cell_grid(x_pos.into(), y_pos.into());
                println!(
                    "Cell {} contains out of bounds reference {} \
                at [{}, {}, {}] which should be in ({}, {})",
                    crate::util::get_cell_name(record),
                    reference.id,
                    x_pos,
                    y_pos,
                    z_pos,
                    actual_x,
                    actual_y
                );
            }
        }
        if let Some(replacement) = self.broken.get(id.as_str()) {
            if replacement.is_empty() {
                println!(
                    "Cell {} contains broken reference {}",
                    crate::util::get_cell_name(record),
                    reference.id
                );
            } else {
                println!(
                    "Cell {} contains broken reference {} which should be {}",
                    crate::util::get_cell_name(record),
                    reference.id,
                    replacement
                );
            }
        }
        if let Some(height) = get_water_height(record) {
            let [x, y, _] = reference.rotation;
            if record.is_interior()
                && reference.translation[2] >= height
                && x == 0.
                && y == 0.
                && BLACK_SQUARES
                    .iter()
                    .any(|id| id.eq_ignore_ascii_case(&reference.id))
            {
                let name = crate::util::get_cell_name(record);
                let key = format!("{}_{}", name, id);
                if self.seen.insert(key) {
                    println!(
                        "Cell {} contains above water black square {}",
                        name, reference.id
                    );
                }
            }
        }
    }
}

impl CellValidator {
    pub fn new() -> Self {
        return Self {
            seen: HashSet::new(),
            broken: get_broken_data(),
        };
    }
}