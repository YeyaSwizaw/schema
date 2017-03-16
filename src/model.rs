use ::view::{View, Index};
use ::values::*;

struct Table {
    name: String,
}

pub struct Model {
    pub view: View,

    tables: Vec<Table>
}

impl Table {
    fn new<S: AsRef<str>>(name: S) -> Table {
        Table {
            name: name.as_ref().to_owned()
        }
    }
}

impl Model {
    pub fn new() -> Model {
        Model {
            view: View::new(),

            tables: Vec::new()
        }
    }

    pub fn add_table<S: AsRef<str>>(&mut self, name: S, pos: WorldCoord) -> Index {
        let table = Table::new(name);
        let idx = self.tables.len();
        self.tables.push(table);
        self.view.add_table(idx, pos)
    }
}
