use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ValueType {
    Bool,
    Nil,
    Number,
}

// This basically implements tagged unions aka enums in Rust again.
#[derive(Clone, Copy)]
union V {
    boolean: bool,
    number: f64,
}

#[derive(Clone)]
pub struct Value {
    typ: ValueType,
    _as: V,
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value {
                typ: ValueType::Bool,
                ..
            } => write!(f, "Value({})", self.as_bool()),
            Value {
                typ: ValueType::Nil,
                ..
            } => write!(f, "Value(Nil)"),
            Value {
                typ: ValueType::Number,
                ..
            } => write!(f, "Value({})", self.as_number()),
        }
    }
}

impl Value {
    pub fn new_bool(value: bool) -> Value {
        Value {
            typ: ValueType::Bool,
            _as: V { boolean: value },
        }
    }

    pub fn new_nil() -> Value {
        Value {
            typ: ValueType::Nil,
            _as: V { number: 0.0 },
        }
    }

    pub fn new_number(value: f64) -> Value {
        Value {
            typ: ValueType::Number,
            _as: V { number: value },
        }
    }

    pub fn as_bool(&self) -> bool {
        unsafe { self._as.boolean }
    }

    pub fn as_number(&self) -> f64 {
        unsafe { self._as.number }
    }

    pub fn is_bool(&self) -> bool {
        self.typ == ValueType::Bool
    }

    pub fn is_nil(&self) -> bool {
        self.typ == ValueType::Nil
    }

    pub fn is_number(&self) -> bool {
        self.typ == ValueType::Number
    }
}

// We are not repeating the array implementation.
pub type ValueArray = Vec<Value>;

pub fn print_value(value: &Value) {
    // {} will print 100.0 as 100.
    print!("{}", value.as_number());
}
