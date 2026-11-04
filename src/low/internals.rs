pub struct LineInternals {
    op_mangler: Mangler,
}

struct Mangler {
    id: usize,
}

impl Mangler {
    fn new() -> Self {
        Self { id: 0 }
    }
    fn id(&mut self) -> usize {
        self.id += 1;
        self.id - 1
    }
}

impl LineInternals {
    pub(crate) fn op_id(&mut self) -> String {
        self.op_mangler.id().to_string()
    }
}
