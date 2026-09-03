use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use serde::Serialize;

pub type CellID = u8;
pub const CELL_ID_MAX: usize = 16;

#[derive(Deserialize, Serialize, Debug)]
pub struct Engine {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub wrap_around: Option<bool>,
    pub default_cell: CellID,
    pub simulation_dist: u8,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct CellInfo {
    pub id: CellID,
    pub color: String,
    pub rules: Vec<RuleConfig>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RuleConfig {
    pub look_for: Vec<CellID>,
    pub min: u8,
    pub max: u8,
    pub next_state: CellID,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub engine: Engine,
    pub cells: HashMap<String, CellInfo>,
    pub id_to_string: Option<HashMap<CellID, String>>,
}


impl Config {

    pub fn from_file(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let content: String = fs::read_to_string(filename)?;
        let mut config: Config = toml::from_str(&content)?;
        let mut id_to_string:HashMap<CellID, String> = Default::default();
        for (name, info) in &config.cells{
            id_to_string.entry(info.id).insert_entry(name.clone());
        }
        config.id_to_string = Some(id_to_string);
        Ok(config)
    }

}


impl CellInfo {
    pub fn update(&self, surounding: &[u8; CELL_ID_MAX]) -> CellID{
        for rule in &self.rules{
            // println!("{surounding:?} -- {}", self.id);
            // println!("{:?}", self.rules);
            // let mut _s = String ::new();
            // _ = stdin().read_line(&mut _s);
            if surounding[rule.look_for[0] as usize] >= rule.min && surounding[rule.look_for[0] as usize] <= rule.max
                {
                    // println!("{surounding:?} -- {}", self.id);
                    return rule.next_state;
                }
        }
        self.id
    }
}
