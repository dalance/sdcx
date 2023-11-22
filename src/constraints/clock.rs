use crate::constraints::Object;
use crate::errors::SemanticError;
use crate::sdc::util::Validate;
use crate::sdc::CreateClock;
use std::fmt;

/// Clock definition
#[derive(Debug)]
pub struct Clock<'a> {
    command: &'a mut CreateClock,
}

impl<'a> Clock<'a> {
    pub fn name(&self) -> Result<String, SemanticError> {
        let location = self.command.location();
        if let Some(name) = &self.command.name {
            Ok(format!("{name}"))
        } else if let Some(source) = self.source()? {
            match source {
                Object::Pin(x) => Ok(x),
                Object::Port(x) => Ok(x),
                _ => Err(SemanticError::Interpret(location)),
            }
        } else {
            Err(SemanticError::Interpret(location))
        }
    }

    pub fn source(&self) -> Result<Option<Object>, SemanticError> {
        if let Some(ref source) = self.command.source_objects {
            Ok(Some(source.try_into()?))
        } else {
            Ok(None)
        }
    }

    pub fn period(&self) -> Result<f32, SemanticError> {
        let period = format!("{}", self.command.period);
        let location = self.command.period.location();
        let period = match period.parse::<f32>() {
            Ok(x) => x,
            Err(_) => {
                return Err(SemanticError::Interpret(location));
            }
        };
        Ok(period)
    }

    pub fn waveform(&self) -> Result<Vec<f32>, SemanticError> {
        let mut ret = vec![];
        if let Some(ref waveform) = self.command.waveform {
            let waveform = format!("{}", waveform);
            let waveform = &waveform[1..waveform.len() - 1];
            let waveform = waveform.split_whitespace();
            let location = self.command.period.location();

            for w in waveform {
                let w = match w.parse::<f32>() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(SemanticError::Interpret(location));
                    }
                };
                ret.push(w);
            }
        }
        Ok(ret)
    }

    pub(crate) fn from(value: &'a mut CreateClock) -> Self {
        Clock { command: value }
    }
}

impl<'a> fmt::Display for Clock<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().map_err(|_| fmt::Error)?;
        let source = self.source().map_err(|_| fmt::Error)?;
        let period = self.period().map_err(|_| fmt::Error)?;
        let text = if let Some(source) = source {
            format!("Clock: name={name}, period={period}, source={source}")
        } else {
            format!("Clock: name={name}, period={period}")
        };
        text.fmt(f)
    }
}

///// GeneratedClock definition
//#[derive(Clone, Debug)]
//pub struct GeneratedClock {
//    name: String,
//    source: Option<Object>,
//    master_pin: Object,
//    master_clock: Option<String>,
//    factor: Factor,
//    duty_cycle: i32,
//    edges: Vec<i32>,
//    edge_shift: Vec<f32>,
//    invert: bool,
//    combinational: bool,
//    comment: String,
//}
//
//impl GeneratedClock {
//    pub fn name(&self) -> &str {
//        &self.name
//    }
//
//    pub fn source(&self) -> &Option<Object> {
//        &self.source
//    }
//
//    pub fn master_pin(&self) -> &Object {
//        &self.master_pin
//    }
//
//    pub fn master_clock(&self) -> &Option<String> {
//        &self.master_clock
//    }
//
//    pub fn factor(&self) -> Factor {
//        self.factor
//    }
//
//    pub fn duty_cycle(&self) -> i32 {
//        self.duty_cycle
//    }
//
//    pub fn edges(&self) -> &[i32] {
//        &self.edges
//    }
//
//    pub fn edge_shift(&self) -> &[f32] {
//        &self.edge_shift
//    }
//
//    pub fn invert(&self) -> bool {
//        self.invert
//    }
//
//    pub fn combinational(&self) -> bool {
//        self.combinational
//    }
//
//    pub fn comment(&self) -> &str {
//        &self.comment
//    }
//}
//
//#[derive(Copy, Clone, Debug)]
//pub enum Factor {
//    Multiply(i32),
//    Divide(i32),
//}
