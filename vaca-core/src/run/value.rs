use std::{fmt::Display, iter::zip};

use crate::build::atom::Atom;

use self::{array::Array, external::External, function::Function, macros::Macro, object::Object};

pub mod function;
pub mod macros;
pub mod array;
pub mod object;
pub mod external;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    NotANumber,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Array(Array),
    Object(Object),
    Function(Function),
    Macro(Macro),
    Atom(Atom),
    External(External)
}

impl Into<Array> for Value {
    fn into(self) -> Array {
        match self {
            Self::Array(a) => a,
            d => panic!("Can't turn a {} into an array like", d)
        }
    }
}

impl Value {
    /// Copies the Value and unwraps an array
    pub fn to_array(&self) -> Array {
        match self {
            Self::Array(a) => a.clone(),
            d => panic!("Can't turn a {} into an array like", d)
        }
    }

    pub fn as_boolean(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::NotANumber => false,
            Value::Bool(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.,
            Value::Char(c) => *c != '\0',
            Value::String(s) => s.len() != 0,
            Value::Array(a) => a.len() != 0,
            Value::Object(o) => !o.is_empty(),
            Value::Function(_) => true,
            Value::Macro(_) => true,
            Value::Atom(a) => a == &Atom::from(":true"),
            Value::External(_) => true,
        }
    }

    pub fn is_nil(&self) -> bool {
        if let Value::Nil = self {
            true
        } else {
            false
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Nil => format!("'nil"),
            Self::NotANumber => format!("'nan"),
            Self::Bool(b) => format!("{b}"),
            Self::Char(c) => format!("{c}"),
            Self::Integer(i) => format!("{i}"),
            Self::Float(f) => format!("{f}"),
            Self::Array(a) => format!("[ {}]", a.iter()
                .map(|w| format!("{} ", w.as_ref()))
                .reduce(|acc, f| format!("{acc}{f}"))
                .unwrap_or(String::from(""))
            ),
            Self::Object(o) => format!(":{{ {}}}", o.iter()
                .map(|(k, v)| format!("{}: {} ", k, v.as_ref()))
                .reduce(|acc, f| format!("{acc}{f}"))
                .unwrap_or(String::from(""))
            ),
            Self::Function(f) => format!("'func\\{}", f.arity()),
            Self::Macro(m) => format!("'macro\\{}", m.arity()),
            Self::String(s) => s.clone(),
            Self::Atom(a) => a.to_string(),
            Self::External(External { lib, symbol, kind, arity, is_action: _}) => match kind {
                external::ExternalKind::Function => format!("'func::{lib}::{symbol}\\{arity}"),
                external::ExternalKind::Macro => format!("'macro::{lib}::{symbol}\\{arity}"),
            }
        })
    }
}


impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::Integer(l0), Self::Float(r0)) => (*l0 as f64).partial_cmp(r0),
            (Self::Float(l0), Self::Float(r0)) => l0.partial_cmp(r0),
            (Self::Float(l0), Self::Integer(r0)) => l0.partial_cmp(&(*r0 as f64)),
            (Self::Char(l0), Self::Char(r0)) => l0.partial_cmp(r0),
            (Self::String(l0), Self::String(r0)) => l0.partial_cmp(r0),
            _ => None
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::NotANumber, Self::NotANumber) => true,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Float(r0)) => *l0 as f64 == *r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Float(l0), Self::Integer(r0)) => *l0 == *r0 as f64,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => {
                if l0.len() != r0.len() {
                    false
                } else {
                    zip(l0, r0).all(|(l, r)| l.as_ref() == r.as_ref())
                }
            },
            (Self::Object(l0), Self::Object(r0)) => {
                l0 == r0
            },
            (Self::Atom(l0), Self::Atom(r0)) => l0 == r0,
            _ => false//core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}