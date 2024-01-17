use std::{fmt::Display, rc::{Rc, Weak}, collections::LinkedList};

use super::Data;

#[derive(Debug)]
pub struct OwnScope(Vec<Rc<Data>>);

#[derive(Debug)]
pub struct Owner {
    scopes: LinkedList<OwnScope>
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Nil => format!("'nil"),
            Self::Bool(b) => format!("{b}"),
            Self::Char(c) => format!("{c}"),
            Self::Integer(i) => format!("{i}"),
            Self::Float(f) => format!("{f}"),
            Self::Array(a) => format!("[{}]", a.iter()
                .map(|w| format!("{}, ", match w.upgrade() {
                    Some(d) => format!("{d}"),
                    None => format!("'undefined")
                }))
                .reduce(|f, acc| format!("{acc}{f}"))
                .unwrap_or(String::from(""))
            ),
            Self::Function(f) => format!("'function\\{}", f.arity()),
            Self::String(s) => s.clone()
        })
    }
}

impl OwnScope {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl Owner {
    pub fn new() -> Self {
        Self {
            scopes: LinkedList::new()
        }
    }

    pub fn create_scope(&mut self) {
        self.scopes.push_back(OwnScope::new());
    }

    pub fn drop_scope(&mut self) {
        self.scopes.pop_back();
    }

    pub fn insert(&mut self, data: Data) -> Weak<Data> {
        { self.scopes.back_mut().unwrap().0.push(Rc::new(data)); }

        Rc::downgrade(self.scopes.back().unwrap().0.last().unwrap())
    }

    pub fn insert_return(&mut self, data: Weak<Data>) -> Weak<Data> {
        let mut iter = self.scopes.iter_mut();
        
        if let Some(_) = iter.next_back() {
            if let Some(scope) = iter.next_back() {
                // second_last is now the second last element
                scope.0.push(data.upgrade().unwrap().clone());

                return Rc::downgrade(scope.0.last().unwrap());
            }
        }

        panic!("No scope to insert a return value");
    }
}