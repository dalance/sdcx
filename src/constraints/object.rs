use crate::errors::InterpretError;
use crate::sdc::{Argument, Command};
use std::fmt;

/// Object
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Object {
    Pin(String),
    Port(String),
    Net(String),
    Cell(String),
}

impl TryFrom<&Argument> for Object {
    type Error = InterpretError;

    fn try_from(value: &Argument) -> Result<Self, Self::Error> {
        let location = value.location();
        if let Argument::CommandSubstitution(value, _) = value {
            match **value {
                Command::GetPins(ref x) if x.patterns.is_some() => {
                    match x.patterns.clone().unwrap() {
                        Argument::Word(x) => return Ok(Object::Pin(x.text.clone())),
                        Argument::BraceGroup(x) => {
                            let name = x.text[1..x.text.len() - 1].to_string();
                            if name.contains(char::is_whitespace) {
                                return Err(InterpretError::Something(location));
                            }
                            return Ok(Object::Pin(name));
                        }
                        _ => (),
                    }
                }
                Command::GetPorts(ref x) if x.patterns.is_some() => {
                    match x.patterns.clone().unwrap() {
                        Argument::Word(x) => return Ok(Object::Port(x.text.clone())),
                        Argument::BraceGroup(x) => {
                            let name = x.text[1..x.text.len() - 1].to_string();
                            if name.contains(char::is_whitespace) {
                                return Err(InterpretError::Something(location));
                            }
                            return Ok(Object::Port(name));
                        }
                        _ => (),
                    }
                }
                Command::GetNets(ref x) if x.patterns.is_some() => {
                    match x.patterns.clone().unwrap() {
                        Argument::Word(x) => return Ok(Object::Net(x.text.clone())),
                        Argument::BraceGroup(x) => {
                            let name = x.text[1..x.text.len() - 1].to_string();
                            if name.contains(char::is_whitespace) {
                                return Err(InterpretError::Something(location));
                            }
                            return Ok(Object::Net(name));
                        }
                        _ => (),
                    }
                }
                Command::GetCells(ref x) if x.patterns.is_some() => {
                    match x.patterns.clone().unwrap() {
                        Argument::Word(x) => return Ok(Object::Cell(x.text.clone())),
                        Argument::BraceGroup(x) => {
                            let name = x.text[1..x.text.len() - 1].to_string();
                            if name.contains(char::is_whitespace) {
                                return Err(InterpretError::Something(location));
                            }
                            return Ok(Object::Cell(name));
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        Err(InterpretError::Something(location))
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Object::Pin(x) => format!("pin({x})"),
            Object::Port(x) => format!("port({x})"),
            Object::Net(x) => format!("net({x})"),
            Object::Cell(x) => format!("cell({x})"),
        };
        text.fmt(f)
    }
}
