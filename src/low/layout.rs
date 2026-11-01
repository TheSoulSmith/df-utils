/// Layout metadata for diamondfire. Slot counts are not byte counts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Layout {
    Atom,
    Struct(Vec<Layout>),
    Array {
        element: Box<Layout>,
        len: usize,
    },
    /// A runtime-sized sequence whose elements share this layout.
    List(Box<Layout>),
    /// Known dictionary keys and the layouts of their corresponding values.
    Dict(Box<Layout>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathElement {
    Field(usize),
    Index(usize),
    /// Something that is known only at runtime
    /// Whatever produces this guarantees that it is valid and within range
    Runtime,
}

impl Layout {
    /// Number of statically addressed slots occupied in the containing layout.
    ///
    /// Dynamic containers occupy one slot here; their contents are described
    /// by their child layouts and are not part of this static slot count.
    pub fn slot_size(&self) -> usize {
        match self {
            Self::Atom | Self::List(_) | Self::Dict(_) => 1,
            Self::Struct(fields) => fields.iter().map(Self::slot_size).sum(),
            Self::Array { element, len } => element.slot_size() * len,
        }
    }

    /// Find the layout reached by following a structural path.
    ///
    /// List indices select the repeated element layout; the index cannot be
    /// bounds-checked here because list length is only known at runtime.
    pub fn layout_at(&self, path: &[PathElement]) -> Option<&Layout> {
        let mut layout = self;

        for element in path {
            match (layout, element) {
                (Self::Struct(fields), PathElement::Field(index)) => {
                    layout = fields.get(*index)?;
                }
                (Self::Array { element, len }, PathElement::Index(index)) if index < len => {
                    layout = element;
                }
                (Self::List(element), PathElement::Runtime) => layout = element,
                (Self::Dict(element), PathElement::Runtime) => layout = element,
                _ => return None,
            }
        }

        Some(layout)
    }

    /// Find a slot offset for a path made only of fixed-layout fields/arrays.
    ///
    /// This deliberately cannot return offsets inside lists or dictionaries:
    /// their contents are not part of the parent's static slot sequence.
    ///
    /// You can, however, use `layout_at` to get the layout of the data in lists or dictionaries and then use this on that.
    /// Note that if you do this for a list, you will want to add the size of the layout * the number of preceding entries.
    pub fn slot_at(&self, path: &[PathElement]) -> Option<(usize, &Layout)> {
        let mut layout = self;
        let mut offset = 0;

        for element in path {
            match (layout, element) {
                (Self::Struct(fields), PathElement::Field(index)) => {
                    layout = fields.get(*index)?;
                    offset += fields[..*index].iter().map(Self::slot_size).sum::<usize>();
                }
                (Self::Array { element, len }, PathElement::Index(index)) if index < len => {
                    offset += element.slot_size() * index;
                    layout = element;
                }
                _ => return None,
            }
        }

        Some((offset, layout))
    }
}
