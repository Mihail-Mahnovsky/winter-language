use std::{
    ffi::NulError,
    fmt::{self, write},
};

use crate::parser::parser::Type;

#[derive(Debug, Clone)]
pub enum Object {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Char(char),
    Long(i128),
    Short(i8),
    Void,
}

impl Object {
    pub fn get_type(&self) -> Type {
        match self {
            Object::Int(_) => Type::Int,
            Object::Float(_) => Type::Float,
            Object::String(_) => Type::String,
            Object::Bool(_) => Type::Bool,
            Object::Char(_) => Type::Char,
            Object::Long(_) => Type::Long,
            Object::Short(_) => Type::Short,
            Object::Void => Type::Void,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        if let Self::Int(value) = *self {
            return Some(value);
        } else {
            return None;
        }
    }
    pub fn as_long(&self) -> Option<i128> {
        if let Self::Long(value) = *self {
            return Some(value);
        } else {
            return None;
        }
    }
    pub fn as_float(&self) -> Option<f32> {
        if let Self::Float(value) = *self {
            return Some(value);
        } else {
            return None;
        }
    }
    pub fn as_string(&self) -> Option<String> {
        if let Self::String(value) = self {
            return Some(value.clone());
        } else {
            return None;
        }
    }
    pub fn as_char(&self) -> Option<char> {
        if let Self::Char(value) = self {
            return Some(value.clone());
        } else {
            return None;
        }
    }
    pub fn as_short(&self) -> Option<i8> {
        if let Self::Short(value) = self {
            return Some(value.clone());
        } else {
            return None;
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Bool(value) = self {
            return Some(value.clone());
        } else {
            return None;
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(i) => write!(f, "{}", i),
            Object::Float(fl) => write!(f, "{}", fl),
            Object::String(s) => write!(f, "{}", s),
            Object::Bool(s) => write!(f, "{}", s),
            Object::Char(s) => write!(f, "{}", s),
            Object::Long(l) => write!(f, "{}", l),
            Object::Short(l) => write!(f, "{}", l),
            Object::Void => write!(f, "void"),
        }
    }
}
