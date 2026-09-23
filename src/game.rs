use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

use crate::config::{CellID, Config, CELL_ID_MAX};

pub struct Map {
    // data: Vec<Vec<CellState>>,
    data: HashMap<(i16, i16), CellID>,
    default_cell: CellID,
}

impl Map {
    fn default(default_cell: CellID) -> Map {
        Map {
            data: HashMap::default(),
            default_cell,
        }
    }

    pub fn get(&self, x: i16, y: i16) -> CellID {
        match self.data.get(&(x, y)) {
            None => self.default_cell,
            Some(cellid) => *cellid,
        }
    }

    pub fn set(&mut self, x: i16, y: i16, cell_type: CellID) {
        if cell_type == self.default_cell {
            self.data.remove(&(x, y));
        }
        self.data.insert((x, y), cell_type);
    }

    pub fn get_around(&self, x: i16, y: i16) -> [CellID; 8] {
        let mut ret_list: [CellID; 8] = [self.default_cell; 8];
        let mut index = 0;
        for n in x - 1..=x + 1 {
            for m in y - 1..=y + 1 {
                if n == x && m == y {
                    continue;
                }
                ret_list[index] = self.get(n, m);
                index += 1;
            }
        }
        ret_list
    }

    /**
     * Return a copi off all the current Not default cell
     */
    pub fn get_all_cell(&self) -> Vec<(i16, i16)> {
        self.data
            .keys()
            .filter(|cell| self.get(cell.0, cell.1) != self.default_cell)
            .copied()
            .collect()
    }

    pub fn display_term(&self, x: i16, y: i16, rangeX: i16, rangeY: i16) {

        for m in y - rangeY..y + rangeY {
            for n in x - rangeX..x + rangeX {
                let id = self.get(n, m);
                if id == 0 {
                    print!("\x1b[100m  ");
                } else if id == 1 {
                    print!("\x1b[47m  ");
                } else {
                    print!("\x1b[42m  ");
                }
            }
            println!("\x1b[0m");
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.data {
            write!(f, "{:?}", line)?
        }
        Ok(())
    }
}

pub struct Context {
    pub map: Map,
    // nb_tick : u32,
    config: Config,
    // default_cell_state: CellState,
    // cell_state_list: HashSet<String>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Context {
            map: Map::default(config.engine.default_cell),
            // nb_tick: 0,
            config,
            // default_cell_state: CellState::Dead,
            // cell_state_list : default(),
        }
    }

    pub fn update(&mut self) {
        let all_not_default_cell: Vec<(i16, i16)> = self.map.get_all_cell();
        println!("len size - {:}", all_not_default_cell.len());
        // let mut proximity_counter: HashMap<(i16, i16), HashMap<CellID,u8>> = Default::default();
        let mut proximity_counter: HashMap<(i16, i16), [u8; CELL_ID_MAX]> = Default::default(); // counter inner_map[cellType] +=1 mean 1 neighbor   of said type
        #[rustfmt::skip] // Skip the formating of next line
        let offsets: [(i16, i16); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            ( 0, -1),          ( 0, 1),
            ( 1, -1), ( 1, 0), ( 1, 1),
        ];
        for (n, m) in all_not_default_cell {
            let cell_type: CellID = self.map.get(n, m);
            proximity_counter.entry((n, m)).or_default();
            for &(ox, oy) in offsets.iter() {
                // let inner_map: &mut HashMap<CellID, u8> = proximity_counter.entry((n+ox, m+oy)).or_default();
                let inner_map: &mut [u8; CELL_ID_MAX] = proximity_counter.entry((n + ox, m + oy)).or_default();
                inner_map[cell_type as usize] += 1;
            }
        }

        for (pos, surounding) in proximity_counter {
            let current_id: CellID = self.map.get(pos.0, pos.1);
            self.map.set(
                pos.0,
                pos.1,
                self.config.cells[&(self.config.id_to_string.as_ref().unwrap())[&current_id]]
                    .update(&surounding),
            );
        }
    }
}
