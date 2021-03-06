use ::values::*;

pub struct Table {
    id: usize,

    pub pos: WorldCoord,
    pub size: (i32, i32),

    pub inner_colour: (f32, f32, f32, f32),
    pub outer_colour: (f32, f32, f32, f32),
}

#[derive(Debug, Copy, Clone)]
pub struct Index(usize);

pub struct View {
    tables: Vec<Table>,
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
            tables: Vec::new(),
        }
    }

    pub fn get_table_mut(&mut self, index: Index) -> Option<&mut Table> {
        self.tables.get_mut(index.0)
    }

    pub fn get_table(&self, index: Index) -> Option<&Table> {
        self.tables.get(index.0)
    }

    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    pub fn bring_to_front(&mut self, index: Index) -> Index {
        let table = self.tables.remove(index.0);
        self.tables.push(table);
        Index(self.tables.len() - 1)
    }

    pub fn check_focus(&self, coord: WorldCoord) -> Option<Index> {
        for (index, table) in self.tables.iter().enumerate().rev() {
            let (x0, x1) = if table.size.0 > 0 {
                (table.pos.0, table.pos.0 + table.size.0)
            } else {
                (table.pos.0 + table.size.0, table.pos.0)
            };

            let (y0, y1) = if table.size.1 > 0 {
                (table.pos.1, table.pos.1 + table.size.1)
            } else {
                (table.pos.1 + table.size.1, table.pos.1)
            };

            if coord.0 > x0 && coord.1 > y0 && coord.0 < x1 && coord.1 < y1 {
                return Some(Index(index));
            }
        }

        None
    }

    pub fn add_table(&mut self, id: usize, pos: WorldCoord) -> Index {
        let idx = Index(self.tables.len());
        self.tables.push(Table::new(id, pos));
        idx
    }
}
