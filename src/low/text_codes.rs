use super::raw::Item;

#[derive(Clone, Copy, Debug)]
pub struct RawTextNum(usize);

impl RawTextNum {
    fn stringify(self) -> String {
        let num = self.0;
        let mut out = num.to_string();
        if num % 100 == 0 {
            out.pop();
            out.pop();
            if num % 1000 != 0 {
                out.pop();
                out.insert(out.len() - 1, '.');
            }
        } else {
            if num % 10 == 0 {
                out.insert(out.len() - 3, '.');
            } else {
                out.pop();
                out.insert(out.len() - 4, '.');
            }
        }
        out
    }
}

fn lint(text: &String) -> Result<(), String> {
    let mut paren_count: usize = 0;
    let chars = text.clone();
    for char in chars.chars() {
        match char {
            ',' => {
                if paren_count == 0 {
                    return Err(format!(
                        "Could not use \"{}\" in a text code, as it has a comma that is not inside any parenthesis",
                        text
                    ));
                }
            }
            '(' => paren_count += 1,
            ')' => {
                if paren_count == 0 {
                    return Err(format!(
                        "Could not use \"{}\" in a text code, as it has a lonely close parenthesis",
                        text
                    ));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub enum NumTextCode {
    Raw {
        v: RawTextNum,
    },
    Math {
        start: Box<Self>,
        rest: Vec<(Operation, Self)>,
    },
    Random {
        min: Box<Self>,
        max: Box<Self>,
    },
    Round {
        v: Box<Self>,
    },
    Index {
        var: StrTextCode,
        index: Box<Self>,
    },
    Entry {
        var: StrTextCode,
        key: StrTextCode,
    },
    Var {
        name: StrTextCode,
    },
    BVar {
        n: StrTextCode,
        b: StrTextCode,
        var: StrTextCode,
    },
}

#[derive(Clone, Debug)]
pub enum StrTextCode {
    Raw {
        v: String,
    },
    Concat {
        v: Vec<Self>,
    },
    Random {
        min: Box<NumTextCode>,
        max: Box<NumTextCode>,
    },
    Round {
        v: Box<NumTextCode>,
    },
    Index {
        var: Box<Self>,
        index: Box<NumTextCode>,
    },
    Entry {
        var: Box<Self>,
        key: Box<Self>,
    },
    Var {
        name: Box<Self>,
    },
    BVar {
        n: Box<Self>,
        b: Box<Self>,
        var: Box<Self>,
    },
    Default,
    DefaultUUID,
    Damager,
    DamagerUUID,
    Killer,
    KillerUUID,
    Shooter,
    ShooterUUID,
    Victim,
    VictimUUID,
    Proj,
    ProjUUID,
    Selected,
    SelectedUUID,
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

impl NumTextCode {
    pub fn compile(&self) -> Result<String, String> {
        let mut out = String::new();
        match self.compile_helper(&mut out) {
            Ok(_) => Ok(out),
            Err(e) => Err(e),
        }
    }
    fn compile_helper(&self, out: &mut String) -> Result<(), String> {
        match self {
            Self::Raw { v } => {
                out.push_str(v.stringify().as_str());
            }
            Self::Math { start, rest } => {
                out.push_str("%math(");
                start.compile_helper(out)?;
                for (op, code) in rest {
                    out.push_str(match op {
                        Operation::Add => "+",
                        Operation::Sub => "-",
                        Operation::Mult => "*",
                        Operation::Div => "/",
                        Operation::Mod => "%",
                    });
                    code.compile_helper(out)?;
                }
                out.push(')');
            }
            Self::Random { min, max } => {
                out.push_str("%random(");
                min.compile_helper(out)?;
                out.push(',');
                max.compile_helper(out)?;
                out.push(')');
            }
            Self::Round { v } => {
                out.push_str("%round(");
                v.compile_helper(out)?;
                out.push(')');
            }
            Self::Index { var, index } => {
                out.push_str("%index(");
                var.compile_helper(out, true)?;
                out.push(',');
                index.compile_helper(out)?;
                out.push(')');
            }
            Self::Entry { var, key } => {
                out.push_str("%entry(");
                var.compile_helper(out, true)?;
                out.push(',');
                key.compile_helper(out, true)?;
                out.push(')');
            }
            Self::Var { name } => {
                out.push_str("%var(");
                name.compile_helper(out, true)?;
                out.push(')');
            }
            Self::BVar { n, b, var } => {
                out.push_str("%bvar(");
                n.compile_helper(out, true)?;
                out.push(',');
                b.compile_helper(out, true)?;
                out.push(',');
                var.compile_helper(out, true)?;
                out.push(')');
            }
        };
        Ok(())
    }
}

impl StrTextCode {
    pub fn compile(&self) -> Result<String, String> {
        let mut out = String::new();
        match self.compile_helper(&mut out, false) {
            Ok(_) => Ok(out),
            Err(e) => Err(e),
        }
    }
    fn compile_helper(&self, out: &mut String, should_lint: bool) -> Result<(), String> {
        match self {
            Self::Raw { v } => {
                if should_lint {
                    lint(&v)?
                }
                out.push_str(v.as_str());
            }
            Self::Concat { v } => {
                for i in v {
                    i.compile_helper(out, should_lint)?;
                }
            }
            Self::Random { min, max } => {
                out.push_str("%random(");
                min.compile_helper(out)?;
                out.push(',');
                max.compile_helper(out)?;
                out.push(')');
            }
            Self::Round { v } => {
                out.push_str("%round(");
                v.compile_helper(out)?;
                out.push(')');
            }
            Self::Index { var, index } => {
                out.push_str("%index(");
                var.compile_helper(out, true)?;
                out.push(',');
                index.compile_helper(out)?;
                out.push(')');
            }
            Self::Entry { var, key } => {
                out.push_str("%entry(");
                var.compile_helper(out, true)?;
                out.push(',');
                key.compile_helper(out, true)?;
                out.push(')');
            }
            Self::Var { name } => {
                out.push_str("%var(");
                name.compile_helper(out, true)?;
                out.push(')');
            }
            Self::BVar { n, b, var } => {
                out.push_str("%bvar(");
                n.compile_helper(out, true)?;
                out.push(',');
                b.compile_helper(out, true)?;
                out.push(',');
                var.compile_helper(out, true)?;
                out.push(')');
            }
            Self::Default => out.push_str("%default"),
            Self::DefaultUUID => out.push_str("%defaultuuid"),
            Self::Damager => out.push_str("%damager"),
            Self::DamagerUUID => out.push_str("%damageruuid"),
            Self::Killer => out.push_str("%killer"),
            Self::KillerUUID => out.push_str("%killeruuid"),
            Self::Proj => out.push_str("%projectile"),
            Self::ProjUUID => out.push_str("%projectileuuid"),
            Self::Selected => out.push_str("%selected"),
            Self::SelectedUUID => out.push_str("%uuid"),
            Self::Shooter => out.push_str("%shooter"),
            Self::ShooterUUID => out.push_str("%shooteruuid"),
            Self::Victim => out.push_str("%victim"),
            Self::VictimUUID => out.push_str("%victimuuid"),
        };
        Ok(())
    }
}

impl TryInto<Item> for NumTextCode {
    type Error = String;
    fn try_into(self) -> Result<Item, Self::Error> {
        Ok(Item::Number(self.compile()?))
    }
}

impl TryInto<Item> for StrTextCode {
    type Error = String;
    fn try_into(self) -> Result<Item, Self::Error> {
        Ok(Item::Text(self.compile()?))
    }
}
