use crate::constraints::Object;
use crate::errors::InterpretError;
use crate::sdc::util::CommandExt;
use crate::sdc::CreateClock;
use std::fmt;

/// Clock definition
#[derive(Debug)]
pub struct Clock {
    name: String,
    source: Option<Object>,
    period: f32,
    waveform: Vec<f32>,
}

impl Clock {
    fn interpret_name(command: &CreateClock) -> Result<String, InterpretError> {
        let location = command.location();
        if let Some(name) = &command.name {
            Ok(format!("{name}"))
        } else if let Some(source) = Self::interpret_source(command)? {
            match source {
                Object::Pin(x) => Ok(x),
                Object::Port(x) => Ok(x),
                Object::Net(x) => Ok(x),
                _ => Err(InterpretError::Something(location)),
            }
        } else {
            Err(InterpretError::Something(location))
        }
    }

    fn interpret_source(command: &CreateClock) -> Result<Option<Object>, InterpretError> {
        if let Some(ref source) = command.source_objects {
            Ok(Some(source.try_into()?))
        } else {
            Ok(None)
        }
    }

    fn interpret_period(command: &CreateClock) -> Result<f32, InterpretError> {
        let period = format!("{}", command.period);
        let location = command.period.location();
        let period = match period.parse::<f32>() {
            Ok(x) => x,
            Err(_) => {
                return Err(InterpretError::Something(location));
            }
        };
        Ok(period)
    }

    fn interpret_waveform(command: &CreateClock) -> Result<Vec<f32>, InterpretError> {
        let mut ret = vec![];
        if let Some(ref waveform) = command.waveform {
            let waveform = format!("{}", waveform);
            let waveform = &waveform[1..waveform.len() - 1];
            let waveform = waveform.split_whitespace();
            let location = command.period.location();

            for w in waveform {
                let w = match w.parse::<f32>() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(InterpretError::Something(location));
                    }
                };
                ret.push(w);
            }
        }
        Ok(ret)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn source(&self) -> &Option<Object> {
        &self.source
    }

    pub fn period(&self) -> &f32 {
        &self.period
    }

    pub fn waveform(&self) -> &[f32] {
        &self.waveform
    }
}

impl TryFrom<&CreateClock> for Clock {
    type Error = InterpretError;

    fn try_from(value: &CreateClock) -> Result<Self, InterpretError> {
        let name = Self::interpret_name(value)?;
        let source = Self::interpret_source(value)?;
        let period = Self::interpret_period(value)?;
        let waveform = Self::interpret_waveform(value)?;
        Ok(Clock {
            name,
            source,
            period,
            waveform,
        })
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "Clock:".to_string();
        text.push_str(&format!(" name={}", self.name));
        text.push_str(&format!(" period={}", self.period));
        if let Some(source) = &self.source {
            text.push_str(&format!(" source={source}"));
        }
        if !self.waveform.is_empty() {
            text.push_str(&format!(" waveform=\""));
        }
        for w in &self.waveform {
            text.push_str(&format!("{w} "));
        }
        if !self.waveform.is_empty() {
            text.push_str(&format!("\""));
        }
        text.fmt(f)
    }
}

#[derive(Debug)]
pub struct ClockMut<'a> {
    command: &'a mut CreateClock,
    inner: Clock,
}

impl<'a> TryFrom<&'a mut CreateClock> for ClockMut<'a> {
    type Error = InterpretError;

    fn try_from(value: &'a mut CreateClock) -> Result<Self, InterpretError> {
        let inner = (value as &CreateClock).try_into()?;
        Ok(ClockMut {
            command: value,
            inner,
        })
    }
}

impl<'a> ClockMut<'a> {
    pub fn name(&self) -> &str {
        &self.inner.name()
    }

    pub fn source(&self) -> &Option<Object> {
        &self.inner.source()
    }

    pub fn period(&self) -> &f32 {
        &self.inner.period()
    }

    pub fn waveform(&self) -> &[f32] {
        &self.inner.waveform()
    }

    pub fn rename(&mut self, name: &str) {
        self.command.name = Some(name.into());
    }
}

impl<'a> fmt::Display for ClockMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
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
