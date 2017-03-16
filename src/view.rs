use ::values::*;

pub struct Table {
    id: usize,

    pub pos: WorldCoord,
    pub size: (i32, i32),

    pub inner_colour: (f32, f32, f32, f32),
    pub outer_colour: (f32, f32, f32, f32),
}

#[derive(Copy, Clone)]
pub struct Index(usize);

pub struct View {
    tables: Vec<Table>
}

impl Table {
    fn new(id: usize, pos: WorldCoord) -> Table {
        Table {
            id: id,

            pos: pos,
            size: (0, 0),

            inner_colour: (0.8, 0.6, 0.4, 1.0),
            outer_colour: (0.2, 0.1, 0.4, 1.0)
        }
    }
}

impl View {
    pub fn new() -> View {
        View {
            tables: Vec::new()
        }
    }

    pub fn get_table_mut(&mut self, index: Index) -> Option<&mut Table> {
        self.tables.get_mut(index.0)
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn add_table(&mut self, id: usize, pos: WorldCoord) -> Index {
        let idx = Index(self.tables.len());
        self.tables.push(Table::new(id, pos));
        idx
    }
}
