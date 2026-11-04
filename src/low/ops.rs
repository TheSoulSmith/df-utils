use super::{
    internals::LineInternals,
    raw::*,
    text_codes::{NumTextCode::*, *},
};

pub enum Op {
    Raw(RawTextNum),
    Var(StrTextCode),
    Add(B),
    Sub(B),
    Mul(B),
    Div(B),
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

enum PassState {
    Math(NumTextCode, Vec<(Operation, NumTextCode)>),
    Norm(NumTextCode),
    Var(StrTextCode),
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

impl Op {
    pub fn compile(self, mut internals: LineInternals) -> (Chunk, NumTextCode) {
        let mut chunk = Chunk::new();
        let code = self.compile_helper(&mut AlgebraState::new(), &mut internals, &mut chunk);
        (chunk, code.into())
    }
    fn compile_helper(
        self,
        algebra: &mut AlgebraState,
        internals: &mut LineInternals,
        chunk: &mut Chunk,
    ) -> PassState {
        match self {
            Self::Raw(v) => PassState::Norm(Raw { v }),
            Self::Var(name) => PassState::Var(name),
            Self::Add(args) => {
                let a = args.0;
                let b = args.1;
                match algebra.t {
                    OpCat::Add | OpCat::None => {
                        let a_out = a.compile_helper(algebra, internals, chunk);
                        algebra.t = OpCat::Add;
                        let b_out = b.compile_helper(algebra, internals, chunk);
                        if algebra.negate {
                            a_out - b_out
                        } else {
                            a_out + b_out
                        }
                    }
                    OpCat::Mult => {
                        let mut new_algebra = AlgebraState::new();
                        let a_out = a.compile_helper(&mut new_algebra, internals, chunk);
                        algebra.t = OpCat::Mult;
                        let b_out = b.compile_helper(&mut new_algebra, internals, chunk);
                        PassState::Norm((a_out + b_out).into())
                    }
                }
            }
            _ => panic!("Not implemented yet"),
        }
    }
}
