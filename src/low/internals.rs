use super::raw::{Item, VariableScope::Line};
#[derive(Clone)]
pub struct LineInternals {
    op_mangler: Mangler,
}

#[derive(Clone)]
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
    pub(crate) fn op_var(&mut self) -> String {
        format!("df-utils-internals-op:{}", self.op_mangler.id())
    }
}
