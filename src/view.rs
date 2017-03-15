use ::values::*;

pub struct Table {
    id: usize,

    pub pos: WorldCoord,
    pub size: (u32, u32),
}

pub struct View {
    tables: Vec<Table>
}

impl Table {
    fn new(id: usize, pos: WorldCoord) -> Table {
        Table {
            id: id,
            pos: pos,
            size: (260, 340)
        }
    }
}

impl View {
    pub fn new() -> View {
        View {
            tables: Vec::new()
        }
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn add_table(&mut self, id: usize, pos: WorldCoord) {
        self.tables.push(Table::new(id, pos))
    }
}
