pub struct Table {
    id: usize,

    pub pos: (f32, f32)
}

pub struct View {
    tables: Vec<Table>
}

impl Table {
    fn new(id: usize, pos: (f32, f32)) -> Table {
        Table {
            id: id,
            pos: pos
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

    pub fn add_table(&mut self, id: usize, pos: (f32, f32)) {
        self.tables.push(Table::new(id, pos))
    }
}
