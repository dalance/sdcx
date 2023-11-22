pub mod clock;
pub mod object;

use crate::sdc::{Command, CommandKind, Sdc};
pub use clock::*;
pub use object::*;

#[derive(Clone, Debug)]
pub struct Constraints {
    sdc: Sdc,
}

impl Constraints {
    pub fn clocks(&mut self) -> Vec<Clock> {
        let mut ret = vec![];
        for clock in self.sdc.extract_mut(CommandKind::CreateClock) {
            if let Command::CreateClock(x) = clock {
                ret.push(Clock::from(x));
            }
        }
        ret
    }
}

impl From<Sdc> for Constraints {
    fn from(value: Sdc) -> Self {
        Self { sdc: value }
    }
}

impl From<Constraints> for Sdc {
    fn from(value: Constraints) -> Self {
        value.sdc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    fn parse(code: &str) -> Constraints {
        let sdc = Parser::parse(&code, &"").unwrap();
        sdc.into()
    }

    #[test]
    fn clock() {
        let code = r##"
create_clock -period 10 -name CLK [get_ports i_clk0]
create_clock -period 3.3 [get_ports i_clk1]
create_clock -period 1 [get_pins i_clk2] -waveform {1.0 2.0 3.0 4.0}
        "##;

        let mut constraints = parse(code);
        let clocks = constraints.clocks();

        let name = clocks[0].name().unwrap();
        let source = clocks[0].source().unwrap();
        let period = clocks[0].period().unwrap();
        assert_eq!(name, "CLK");
        assert_eq!(source, Some(Object::Port("i_clk0".into())));
        assert_eq!(period, 10.0);

        let name = clocks[1].name().unwrap();
        let source = clocks[1].source().unwrap();
        let period = clocks[1].period().unwrap();
        assert_eq!(name, "i_clk1");
        assert_eq!(source, Some(Object::Port("i_clk1".into())));
        assert_eq!(period, 3.3);

        let name = clocks[2].name().unwrap();
        let source = clocks[2].source().unwrap();
        let period = clocks[2].period().unwrap();
        let waveform = clocks[2].waveform().unwrap();
        assert_eq!(name, "i_clk2");
        assert_eq!(source, Some(Object::Pin("i_clk2".into())));
        assert_eq!(period, 1.0);
        assert_eq!(waveform[0], 1.0);
        assert_eq!(waveform[1], 2.0);
        assert_eq!(waveform[2], 3.0);
        assert_eq!(waveform[3], 4.0);
    }
}
