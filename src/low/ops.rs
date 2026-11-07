use super::{
    internals::LineInternals,
    raw::{VariableScope::Line, *},
    text_codes::{NumTextCode::*, *},
};

pub enum Op {
    Raw(RawTextNum),
    Var(StrTextCode),
    Add(B),
    Sub(B),
    Mul(B),
    Div(B),
    DivF(B),
    Mod(B),
    Exp(B),
    Root(B),
    Log(B),
    Parse(StrTextCode),
    Abs(U),
    Clamp(T),
    Wrap(T),
    Bounce(T),
    Random(B),
    RandomF(B),
    RoundD(U),
    FloorU(U),
    FloorB(B),
    RoundU(U),
    RoundB(B),
    CeilU(U),
    CeilB(B),
    InterpU(U),
    InterpT(T),
    Sine(U),
    Cos(U),
    Tan(U),
    ATanB(B),
    BOR(B),
    FBOR(B),
    BAND(B),
    FBAND(B),
    BNOT(U),
    FBNOT(U),
    BXOR(B),
    FBXOR(B),
    BLShift(B),
    FBLShift(B),
    BARShift(B),
    FBARShift(B),
    BRShift(B),
    FBRShift(B),
}

type U = Box<Op>;
type B = Box<(Op, Op)>;
type T = Box<(Op, Op, Op)>;

enum OpCat {
    Add,
    Mult,
    None,
    Other,
}

struct AlgebraState {
    negate: bool,
    t: OpCat,
}

impl AlgebraState {
    fn new() -> Self {
        Self {
            negate: false,
            t: OpCat::None,
        }
    }
}

#[derive(Clone)]
enum PassState {
    Math(NumTextCode, Vec<(Operation, NumTextCode)>),
    Norm(NumTextCode),
    Var(StrTextCode),
}

#[derive(Clone)]
struct Pass {
    state: PassState,
    i: Vec<String>,
    o: Vec<String>,
}

impl Pass {
    fn add_in(mut self, i: &StrTextCode) -> Result<Self, String> {
        self.i.push(i.compile()?);
        Ok(self)
    }
    fn add_out(mut self, o: &StrTextCode) -> Result<Self, String> {
        self.o.push(o.compile()?);
        Ok(self)
    }
    fn wrap(mut self) -> Self {
        self.state = PassState::Norm(self.state.into());
        self
    }
    fn var(mut self, rhs: Self, name: String) -> Self {
        for i in rhs.i {
            match self.i.binary_search(&i) {
                Ok(_) => {}
                Err(t) => self.i.insert(t, i),
            }
        }
        self.o = Vec::new();
        self.o.push(name.clone());
        self.state = PassState::Var(StrTextCode::Raw { v: name });
        self
    }
}

impl From<PassState> for Pass {
    fn from(value: PassState) -> Self {
        Self {
            state: value,
            i: Vec::new(),
            o: Vec::new(),
        }
    }
}

impl std::ops::Add for PassState {
    type Output = PassState;
    fn add(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::Math(rhs_start, rhs_rest) => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Add, rhs_start));
                    rest.append(&mut rhs_rest.clone());
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Add, rhs_start));
                    Self::Math(o.into(), rest)
                }
            },
            rh => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Add, rh.into()));
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Add, rh.into()));
                    Self::Math(o.into(), rest)
                }
            },
        }
    }
}

impl std::ops::Add for Pass {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in rhs.i {
            match self.i.binary_search(&i) {
                Ok(_) => {}
                Err(t) => self.i.insert(t, i),
            }
        }
        for o in rhs.o {
            match self.o.binary_search(&o) {
                Ok(_) => {}
                Err(t) => self.o.insert(t, o),
            }
        }
        self.state = self.state + rhs.state;
        self
    }
}

impl std::ops::Sub for PassState {
    type Output = PassState;
    fn sub(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::Math(rhs_start, rhs_rest) => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Sub, rhs_start));
                    rest.append(&mut rhs_rest.clone());
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Sub, rhs_start));
                    Self::Math(o.into(), rest)
                }
            },
            rh => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Sub, rh.into()));
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Sub, rh.into()));
                    Self::Math(o.into(), rest)
                }
            },
        }
    }
}

impl std::ops::Sub for Pass {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in rhs.i {
            match self.i.binary_search(&i) {
                Ok(_) => {}
                Err(t) => self.i.insert(t, i),
            }
        }
        for o in rhs.o {
            match self.o.binary_search(&o) {
                Ok(_) => {}
                Err(t) => self.o.insert(t, o),
            }
        }
        self.state = self.state - rhs.state;
        self
    }
}

impl std::ops::Mul for PassState {
    type Output = PassState;
    fn mul(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::Math(rhs_start, rhs_rest) => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Mult, rhs_start));
                    rest.append(&mut rhs_rest.clone());
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Mult, rhs_start));
                    Self::Math(o.into(), rest)
                }
            },
            rh => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Mult, rh.into()));
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Mult, rh.into()));
                    Self::Math(o.into(), rest)
                }
            },
        }
    }
}

impl std::ops::Mul for Pass {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in rhs.i {
            match self.i.binary_search(&i) {
                Ok(_) => {}
                Err(t) => self.i.insert(t, i),
            }
        }
        for o in rhs.o {
            match self.o.binary_search(&o) {
                Ok(_) => {}
                Err(t) => self.o.insert(t, o),
            }
        }
        self.state = self.state * rhs.state;
        self
    }
}

impl std::ops::Div for PassState {
    type Output = PassState;
    fn div(self, rhs: Self) -> Self::Output {
        match rhs {
            Self::Math(rhs_start, rhs_rest) => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Div, rhs_start));
                    rest.append(&mut rhs_rest.clone());
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Div, rhs_start));
                    Self::Math(o.into(), rest)
                }
            },
            rh => match self {
                Self::Math(s_start, s_rest) => {
                    let mut rest = s_rest;
                    rest.push((Operation::Div, rh.into()));
                    Self::Math(s_start, rest)
                }
                o => {
                    let mut rest = Vec::new();
                    rest.push((Operation::Div, rh.into()));
                    Self::Math(o.into(), rest)
                }
            },
        }
    }
}

impl std::ops::Div for Pass {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        for i in rhs.i {
            match self.i.binary_search(&i) {
                Ok(_) => {}
                Err(t) => self.i.insert(t, i),
            }
        }
        for o in rhs.o {
            match self.o.binary_search(&o) {
                Ok(_) => {}
                Err(t) => self.o.insert(t, o),
            }
        }
        self.state = self.state / rhs.state;
        self
    }
}

impl Into<NumTextCode> for PassState {
    fn into(self) -> NumTextCode {
        match self {
            Self::Math(start, rest) => Math {
                start: Box::new(start),
                rest,
            },
            Self::Norm(t) => t,
            Self::Var(name) => Var { name },
        }
    }
}

impl Into<NumTextCode> for Pass {
    fn into(self) -> NumTextCode {
        self.state.into()
    }
}

impl TryInto<Item> for PassState {
    type Error = String;
    fn try_into(self) -> Result<Item, Self::Error> {
        match self {
            Self::Var(name) => Ok(Item::Variable {
                name: name.compile()?,
                scope: Line,
            }),
            o => Ok(Item::Number(Into::<NumTextCode>::into(o).compile()?)),
        }
    }
}

impl TryInto<Item> for Pass {
    type Error = String;
    fn try_into(self) -> Result<Item, Self::Error> {
        self.state.try_into()
    }
}

impl Op {
    pub fn compile(self, internals: &LineInternals) -> Result<(Chunk, NumTextCode), String> {
        let mut chunk = Chunk::new();
        let mut m_internals = internals.clone();
        let code = self.compile_helper(&mut AlgebraState::new(), &mut m_internals, &mut chunk)?;
        chunk.set_inputs(code.i);
        chunk.set_outputs(code.o);
        Ok((chunk, code.state.into()))
    }
    fn compile_helper(
        self,
        algebra: &mut AlgebraState,
        internals: &mut LineInternals,
        chunk: &mut Chunk,
    ) -> Result<Pass, String> {
        match self {
            Self::Raw(v) => Ok(PassState::Norm(Raw { v }).into()),
            Self::Var(name) => Ok(Pass::from(PassState::Var(name.clone()))
                .add_in(&name)?
                .add_out(&name)?),
            Self::Add(args) => {
                let a = args.0;
                let b = args.1;
                match algebra.t {
                    OpCat::Add | OpCat::None => {
                        let a_out = a.compile_helper(algebra, internals, chunk)?;
                        algebra.t = OpCat::Add;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok(if algebra.negate {
                            a_out - b_out
                        } else {
                            a_out + b_out
                        })
                    }
                    OpCat::Mult | OpCat::Other => {
                        let mut new_algebra = AlgebraState::new();
                        let a_out = a.compile_helper(&mut new_algebra, internals, chunk)?;
                        algebra.t = OpCat::Add;
                        algebra.negate = false;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok((a_out + b_out).wrap())
                    }
                }
            }
            Self::Sub(args) => {
                let a = args.0;
                let b = args.1;
                match algebra.t {
                    OpCat::Add | OpCat::None => {
                        let a_out = a.compile_helper(algebra, internals, chunk)?;
                        algebra.t = OpCat::Add;
                        algebra.negate = !algebra.negate;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok(if algebra.negate {
                            a_out + b_out
                        } else {
                            a_out - b_out
                        })
                    }
                    OpCat::Mult | OpCat::Other => {
                        let mut new_algebra = AlgebraState::new();
                        let a_out = a.compile_helper(&mut new_algebra, internals, chunk)?;
                        algebra.t = OpCat::Add;
                        algebra.negate = true;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok((a_out - b_out).wrap())
                    }
                }
            }
            Self::Mul(args) => {
                let a = args.0;
                let b = args.1;
                match algebra.t {
                    OpCat::Mult | OpCat::None => {
                        let a_out = a.compile_helper(algebra, internals, chunk)?;
                        algebra.t = OpCat::Mult;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok(if algebra.negate {
                            a_out / b_out
                        } else {
                            a_out * b_out
                        })
                    }
                    OpCat::Add | OpCat::Other => {
                        let mut new_algebra = AlgebraState::new();
                        let a_out = a.compile_helper(&mut new_algebra, internals, chunk)?;
                        algebra.t = OpCat::Mult;
                        algebra.negate = false;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok((a_out * b_out).wrap())
                    }
                }
            }
            Self::Div(args) => {
                let a = args.0;
                let b = args.1;
                match algebra.t {
                    OpCat::Mult | OpCat::None => {
                        let a_out = a.compile_helper(algebra, internals, chunk)?;
                        algebra.t = OpCat::Mult;
                        algebra.negate = !algebra.negate;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok(if algebra.negate {
                            a_out / b_out
                        } else {
                            a_out * b_out
                        })
                    }
                    OpCat::Add | OpCat::Other => {
                        let mut new_algebra = AlgebraState::new();
                        let a_out = a.compile_helper(&mut new_algebra, internals, chunk)?;
                        algebra.t = OpCat::Mult;
                        algebra.negate = true;
                        let b_out = b.compile_helper(algebra, internals, chunk)?;
                        Ok((a_out * b_out).wrap())
                    }
                }
            }
            Self::DivF(args) => {
                let a = args.0;
                let b = args.1;
                let mut n_algebra = AlgebraState::new();
                let a_out = a.compile_helper(&mut n_algebra, internals, chunk)?;
                n_algebra = AlgebraState::new();
                let b_out = b.compile_helper(&mut n_algebra, internals, chunk)?;
                let out_name = internals.op_var();
                chunk.push(
                    Block::action(SetVar::Empty4 {
                        params: (
                            Item::Variable {
                                name: out_name.clone(),
                                scope: Line,
                            },
                            vec![a_out.clone().try_into()?, b_out.clone().try_into()?],
                        ),
                        tags: (Some(TagOptionsDefaultFloorResult::FloorResult),),
                    })
                    .unwrap(),
                );
                Ok(a_out.var(b_out, out_name))
            }
            Self::Mod(args) => {
                let a = args.0;
                let b = args.1;
                let mut new_algebra = AlgebraState {
                    negate: false,
                    t: OpCat::Other,
                };
                let a_out = a.compile_helper(&mut new_algebra, internals, chunk)?;
                algebra.t = OpCat::Other;
                algebra.negate = true;
                let b_out = b.compile_helper(algebra, internals, chunk)?;
                Ok((a_out * b_out).wrap())
            }
            _ => panic!("Not implemented yet"),
        }
    }
}
