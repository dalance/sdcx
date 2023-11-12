use crate::sdc_grammar_trait as grammar;
use parol_runtime::ParolError;
use thiserror::Error;

/// SDC Error
#[derive(Debug, Error)]
pub enum SdcError {
    #[error("WrongArgument: {0:?}")]
    WrongArgument(Vec<Argument>),

    #[error("UnknownCommand: {0}")]
    UnknownCommand(String),

    #[error("DuplicatedArgument")]
    DuplicatedArgument(Argument),

    #[error("MissingOptArgument: {0:?}")]
    MissingOptArgument(Argument),

    #[error("MissingPosArgument")]
    MissingPosArgument,

    #[error("TooManyArgument")]
    TooManyArgument,

    #[error("MissingMandatoryArgument: {0}")]
    MissingMandatoryArgument(String),

    #[error("ParseError: {0}")]
    ParseError(#[from] ParolError),

    #[error("SdcVersion")]
    SdcVersion,
}

/// SDC
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sdc {
    pub header: Vec<String>,
    pub version: Option<SdcVersion>,
    pub commands: Vec<Command>,
}

impl TryFrom<&grammar::Source<'_>> for Sdc {
    type Error = SdcError;

    fn try_from(value: &grammar::Source<'_>) -> Result<Self, SdcError> {
        let mut sdc = Sdc::default();
        let mut is_header = true;
        let mut is_first_command = true;
        for source in &value.source_list {
            match source.source_list_group.as_ref() {
                grammar::SourceListGroup::CommandLine(x) => {
                    is_header = false;
                    let command = x.command_line.command.as_ref().try_into()?;

                    match command {
                        Command::Set(x) if x.variable_name == "sdc_version".into() => {
                            if is_first_command {
                                match x.value.as_str() {
                                    "1.1" => sdc.version = Some(SdcVersion::SDC1_1),
                                    "1.2" => sdc.version = Some(SdcVersion::SDC1_2),
                                    "1.3" => sdc.version = Some(SdcVersion::SDC1_3),
                                    "1.4" => sdc.version = Some(SdcVersion::SDC1_4),
                                    "1.5" => sdc.version = Some(SdcVersion::SDC1_5),
                                    "1.6" => sdc.version = Some(SdcVersion::SDC1_6),
                                    "1.7" => sdc.version = Some(SdcVersion::SDC1_7),
                                    "1.8" => sdc.version = Some(SdcVersion::SDC1_8),
                                    "1.9" => sdc.version = Some(SdcVersion::SDC1_9),
                                    "2.0" => sdc.version = Some(SdcVersion::SDC2_0),
                                    "2.1" => sdc.version = Some(SdcVersion::SDC2_1),
                                    _ => return Err(SdcError::SdcVersion),
                                }
                            } else {
                                return Err(SdcError::SdcVersion);
                            }
                        }
                        _ => sdc.commands.push(command),
                    }

                    if is_first_command {
                        is_first_command = false;
                    }
                }
                grammar::SourceListGroup::TermComment(x) => {
                    if is_header {
                        sdc.header.push(x.term_comment.term_comment.text().into());
                    }
                }
                _ => (),
            }
        }
        Ok(sdc)
    }
}

/// SDC version
#[derive(Clone, Debug, PartialEq)]
pub enum SdcVersion {
    SDC1_1,
    SDC1_2,
    SDC1_3,
    SDC1_4,
    SDC1_5,
    SDC1_6,
    SDC1_7,
    SDC1_8,
    SDC1_9,
    SDC2_0,
    SDC2_1,
}

/// Argument
#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    Word(String),
    StringGroup(String),
    BraceGroup(String),
    CommandReplacement(Box<Command>),
}

impl Argument {
    fn as_str(&self) -> &str {
        match self {
            Argument::Word(x) => x.as_str(),
            Argument::StringGroup(x) => x.as_str(),
            Argument::BraceGroup(x) => x.as_str(),
            Argument::CommandReplacement(_) => "",
        }
    }
}

impl From<&str> for Argument {
    fn from(value: &str) -> Self {
        Argument::Word(value.into())
    }
}

impl TryFrom<&grammar::Argument<'_>> for Argument {
    type Error = SdcError;

    fn try_from(value: &grammar::Argument) -> Result<Self, SdcError> {
        match value {
            grammar::Argument::TokenWord(x) => Ok(Self::Word(
                x.token_word.term_word.term_word.text().to_string(),
            )),
            grammar::Argument::TokenStringGroup(x) => Ok(Self::StringGroup(
                x.token_string_group
                    .term_string_group
                    .term_string_group
                    .text()
                    .to_string(),
            )),
            grammar::Argument::TokenBraceGroup(x) => Ok(Self::BraceGroup(
                x.token_brace_group
                    .term_brace_group
                    .term_brace_group
                    .text()
                    .to_string(),
            )),
            grammar::Argument::CommandReplacement(x) => Ok(Self::CommandReplacement(Box::new(
                x.command_replacement.command.as_ref().try_into()?,
            ))),
        }
    }
}

/// SDC command
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    CurrentInstance(CurrentInstance),
    Expr(Expr),
    List(List),
    Set(Set),
    SetHierarchySeparator(SetHierarchySeparator),
    SetUnits(SetUnits),
    AllClocks,
    AllInputs(AllInputs),
    AllOutputs(AllOutputs),
    AllRegisters(AllRegisters),
    CurrentDesign,
    GetCells(GetCells),
    GetClocks(GetClocks),
    GetLibCells(GetLibCells),
    GetLibPins(GetLibPins),
    GetLibs(GetLibs),
    GetNets(GetNets),
    GetPins(GetPins),
    GetPorts(GetPorts),
    CreateClock(CreateClock),
    CreateGeneratedClock(CreateGeneratedClock),
    GroupPath(GroupPath),
    SetClockGatingCheck(SetClockGatingCheck),
    SetClockGroups(SetClockGroups),
    SetClockLatency(SetClockLatency),
    SetSense(SetSense),
    SetClockTransition(SetClockTransition),
    SetClockUncertainty(SetClockUncertainty),
    SetDataCheck(SetDataCheck),
    SetDisableTiming(SetDisableTiming),
    SetFalsePath(SetFalsePath),
    SetIdealLatency(SetIdealLatency),
    SetIdealNetwork(SetIdealNetwork),
    SetIdealTransition(SetIdealTransition),
    SetInputDelay(SetInputDelay),
    SetMaxDelay(SetMaxDelay),
    SetMaxTimeBorrow(SetMaxTimeBorrow),
    SetMinDelay(SetMinDelay),
    SetMinPulseWidth(SetMinPulseWidth),
    SetMulticyclePath(SetMulticyclePath),
    SetOutputDelay(SetOutputDelay),
    SetPropagatedClock(SetPropagatedClock),
    SetCaseAnalysis(SetCaseAnalysis),
    SetDrive(SetDrive),
    SetDrivingCell(SetDrivingCell),
    SetFanoutLoad(SetFanoutLoad),
    SetInputTransition(SetInputTransition),
    SetLoad(SetLoad),
    SetLogicDc(SetLogicDc),
    SetLogicOne(SetLogicOne),
    SetLogicZero(SetLogicZero),
    SetMaxArea(SetMaxArea),
    SetMaxCapacitance(SetMaxCapacitance),
    SetMaxFanout(SetMaxFanout),
    SetMaxTransition(SetMaxTransition),
    SetMinCapacitance(SetMinCapacitance),
    SetOperatingConditions(SetOperatingConditions),
    SetPortFanoutNumber(SetPortFanoutNumber),
    SetResistance(SetResistance),
    SetTimingDerate(SetTimingDerate),
    SetVoltage(SetVoltage),
    SetWireLoadMinBlockSize(SetWireLoadMinBlockSize),
    SetWireLoadMode(SetWireLoadMode),
    SetWireLoadModel(SetWireLoadModel),
    SetWireLoadSelectionGroup(SetWireLoadSelectionGroup),
    CreateVoltageArea(CreateVoltageArea),
    SetLevelShifterStrategy(SetLevelShifterStrategy),
    SetLevelShifterThreshold(SetLevelShifterThreshold),
    SetMaxDynamicPower(SetMaxDynamicPower),
    SetMaxLeakagePower(SetMaxLeakagePower),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CurrentInstance {
    pub instance: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub args: Vec<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct List {
    pub args: Vec<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Set {
    pub variable_name: Argument,
    pub value: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetHierarchySeparator {
    pub separator: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetUnits {
    pub capacitance: Option<Argument>,
    pub resistance: Option<Argument>,
    pub time: Option<Argument>,
    pub voltage: Option<Argument>,
    pub current: Option<Argument>,
    pub power: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllInputs {
    pub level_sensitive: bool,
    pub edge_triggered: bool,
    pub clock: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllOutputs {
    pub level_sensitive: bool,
    pub edge_triggered: bool,
    pub clock: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllRegisters {
    pub hsc: Option<Argument>,
    pub clock: Option<Argument>,
    pub rise_clock: Option<Argument>,
    pub fall_clock: Option<Argument>,
    pub cells: bool,
    pub data_pins: bool,
    pub clock_pins: bool,
    pub slave_clock_pins: bool,
    pub async_pins: bool,
    pub output_pins: bool,
    pub level_sensitive: bool,
    pub edge_triggered: bool,
    pub master_slave: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetCells {
    pub hierarchical: bool,
    pub regexp: bool,
    pub nocase: bool,
    pub of_objects: Option<Argument>,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetClocks {
    pub regexp: bool,
    pub nocase: bool,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetLibCells {
    pub regexp: bool,
    pub hsc: Option<Argument>,
    pub nocase: bool,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetLibPins {
    pub regexp: bool,
    pub nocase: bool,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetLibs {
    pub regexp: bool,
    pub nocase: bool,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetNets {
    pub hierarchical: bool,
    pub hsc: Option<Argument>,
    pub regexp: bool,
    pub nocase: bool,
    pub of_objects: Option<Argument>,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetPins {
    pub hierarchical: bool,
    pub hsc: Option<Argument>,
    pub regexp: bool,
    pub nocase: bool,
    pub of_objects: Option<Argument>,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GetPorts {
    pub regexp: bool,
    pub nocase: bool,
    pub patterns: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateClock {
    pub period: Argument,
    pub name: Option<Argument>,
    pub waveform: Option<Argument>,
    pub add: bool,
    pub comment: Option<Argument>,
    pub source_objects: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateGeneratedClock {
    pub name: Option<Argument>,
    pub source: Argument,
    pub edges: Option<Argument>,
    pub divide_by: Option<Argument>,
    pub multiply_by: Option<Argument>,
    pub duty_cycle: Option<Argument>,
    pub invert: bool,
    pub edge_shift: Option<Argument>,
    pub add: bool,
    pub master_clock: Option<Argument>,
    pub combinational: bool,
    pub comment: Option<Argument>,
    pub source_objects: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroupPath {
    pub name: Option<Argument>,
    pub default: bool,
    pub weight: Option<Argument>,
    pub from: Option<Argument>,
    pub rise_from: Option<Argument>,
    pub fall_from: Option<Argument>,
    pub to: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub through: Option<Argument>,
    pub rise_through: Option<Argument>,
    pub fall_through: Option<Argument>,
    pub comment: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetClockGatingCheck {
    pub setup: Option<Argument>,
    pub hold: Option<Argument>,
    pub rise: bool,
    pub fall: bool,
    pub high: bool,
    pub low: bool,
    pub object_list: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetClockGroups {
    pub group: Argument,
    pub logically_exclusive: bool,
    pub phisically_exclusive: bool,
    pub asynchronous: bool,
    pub allow_paths: bool,
    pub name: Option<Argument>,
    pub comment: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetClockLatency {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub source: bool,
    pub dynamic: bool,
    pub late: bool,
    pub early: bool,
    pub clock: Option<Argument>,
    pub delay: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetSense {
    pub r#type: Option<Argument>,
    pub non_unate: bool,
    pub positive: bool,
    pub negative: bool,
    pub clock_leaf: bool,
    pub stop_propagation: bool,
    pub pulse: Option<Argument>,
    pub clocks: Option<Argument>,
    pub pin_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetClockTransition {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub transition: Argument,
    pub clock_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetClockUncertainty {
    pub from: Option<Argument>,
    pub rise_from: Option<Argument>,
    pub fall_from: Option<Argument>,
    pub to: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub rise: bool,
    pub fall: bool,
    pub setup: bool,
    pub hold: bool,
    pub uncertainty: Argument,
    pub object_list: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetDataCheck {
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub rise_from: Option<Argument>,
    pub fall_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub setup: bool,
    pub hold: bool,
    pub clock: Option<Argument>,
    pub value: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetDisableTiming {
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub cell_pin_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetFalsePath {
    pub setup: bool,
    pub hold: bool,
    pub rise: bool,
    pub fall: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Option<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Option<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Option<Argument>,
    pub comment: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetIdealLatency {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub delay: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetIdealNetwork {
    pub no_propagate: bool,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetIdealTransition {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub transition_time: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetInputDelay {
    pub clock: Option<Argument>,
    pub reference_pin: Option<Argument>,
    pub clock_fall: bool,
    pub level_sensitive: bool,
    pub rise: bool,
    pub fall: bool,
    pub max: bool,
    pub min: bool,
    pub add_delay: bool,
    pub network_latency_included: bool,
    pub source_latency_included: bool,
    pub delay_value: Argument,
    pub port_pin_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxDelay {
    pub rise: bool,
    pub fall: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Option<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Option<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Option<Argument>,
    pub ignore_clock_latency: bool,
    pub comment: Option<Argument>,
    pub delay_value: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxTimeBorrow {
    pub delay_value: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMinDelay {
    pub rise: bool,
    pub fall: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Option<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Option<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Option<Argument>,
    pub ignore_clock_latency: bool,
    pub comment: Option<Argument>,
    pub delay_value: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMinPulseWidth {
    pub low: bool,
    pub high: bool,
    pub value: Argument,
    pub object_list: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMulticyclePath {
    pub setup: bool,
    pub hold: bool,
    pub rise: bool,
    pub fall: bool,
    pub start: bool,
    pub end: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Option<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Option<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Option<Argument>,
    pub comment: Option<Argument>,
    pub path_multiplier: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetOutputDelay {
    pub clock: Option<Argument>,
    pub reference_pin: Option<Argument>,
    pub clock_fall: bool,
    pub level_sensitive: bool,
    pub rise: bool,
    pub fall: bool,
    pub max: bool,
    pub min: bool,
    pub add_delay: bool,
    pub network_latency_included: bool,
    pub source_latency_included: bool,
    pub delay_value: Argument,
    pub port_pin_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPropagatedClock {
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetCaseAnalysis {
    pub value: Argument,
    pub port_or_pin_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetDrive {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub resistance: Argument,
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetDrivingCell {
    pub lib_cell: Option<Argument>,
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub library: Option<Argument>,
    pub pin: Option<Argument>,
    pub from_pin: Option<Argument>,
    pub dont_scale: bool,
    pub no_design_rule: bool,
    pub clock: Option<Argument>,
    pub clock_fall: bool,
    pub input_transition_rise: Option<Argument>,
    pub input_transition_fall: Option<Argument>,
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetFanoutLoad {
    pub value: Argument,
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetInputTransition {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub clock: Option<Argument>,
    pub clock_fall: bool,
    pub transition: Argument,
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetLoad {
    pub min: bool,
    pub max: bool,
    pub subtract_pin_load: bool,
    pub pin_load: bool,
    pub wire_load: bool,
    pub value: Argument,
    pub objects: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetLogicDc {
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetLogicOne {
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetLogicZero {
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxArea {
    pub area_value: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxCapacitance {
    pub value: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxFanout {
    pub value: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxTransition {
    pub clock_path: bool,
    pub data_path: bool,
    pub rise: bool,
    pub fall: bool,
    pub value: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMinCapacitance {
    pub value: Argument,
    pub object_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetOperatingConditions {
    pub library: Option<Argument>,
    pub analysis_type: Option<Argument>,
    pub max: Option<Argument>,
    pub min: Option<Argument>,
    pub max_library: Option<Argument>,
    pub min_library: Option<Argument>,
    pub object_list: Option<Argument>,
    pub condition: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetPortFanoutNumber {
    pub value: Argument,
    pub port_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetResistance {
    pub min: bool,
    pub max: bool,
    pub value: Argument,
    pub net_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetTimingDerate {
    pub cell_delay: bool,
    pub cell_check: bool,
    pub net_delay: bool,
    pub data: bool,
    pub clock: bool,
    pub early: bool,
    pub late: bool,
    pub rise: bool,
    pub fall: bool,
    pub r#static: bool,
    pub dynamic: bool,
    pub increment: bool,
    pub derate_value: Argument,
    pub object_list: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetVoltage {
    pub min: Option<Argument>,
    pub object_list: Option<Argument>,
    pub max_case_voltage: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetWireLoadMinBlockSize {
    pub size: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetWireLoadMode {
    pub mode_name: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetWireLoadModel {
    pub name: Argument,
    pub library: Option<Argument>,
    pub min: bool,
    pub max: bool,
    pub object_list: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetWireLoadSelectionGroup {
    pub library: Option<Argument>,
    pub min: bool,
    pub max: bool,
    pub group_name: Argument,
    pub object_list: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateVoltageArea {
    pub name: Argument,
    pub coordinate: Option<Argument>,
    pub guard_band_x: Option<Argument>,
    pub guard_band_y: Option<Argument>,
    pub cell_list: Argument,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetLevelShifterStrategy {
    pub rule: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetLevelShifterThreshold {
    pub voltage: Option<Argument>,
    pub percent: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxDynamicPower {
    pub power: Argument,
    pub unit: Option<Argument>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SetMaxLeakagePower {
    pub power: Argument,
    pub unit: Option<Argument>,
}

impl TryFrom<&grammar::Command<'_>> for Command {
    type Error = SdcError;

    fn try_from(value: &grammar::Command<'_>) -> Result<Self, SdcError> {
        let command = value.token_word.term_word.term_word.text();
        let mut args = vec![];
        for arg in &value.command_list {
            args.push(arg.argument.as_ref().try_into()?);
        }
        match command {
            "current_instance" => current_instance(args),
            "expr" => expr(args),
            "list" => list(args),
            "set" => set(args),
            "set_hierarchy_separator" => set_hierarchy_separator(args),
            "set_units" => set_units(args),
            "all_clocks" => all_clocks(args),
            "all_inputs" => all_inputs(args),
            "all_outputs" => all_outputs(args),
            "all_registers" => all_registers(args),
            "current_design" => current_design(args),
            "get_cells" => get_cells(args),
            "get_clocks" => get_clocks(args),
            "get_lib_cells" => get_lib_cells(args),
            "get_lib_pins" => get_lib_pins(args),
            "get_libs" => get_libs(args),
            "get_nets" => get_nets(args),
            "get_pins" => get_pins(args),
            "get_ports" => get_ports(args),
            "create_clock" => create_clock(args),
            "create_generated_clock" => create_generated_clock(args),
            "group_path" => group_path(args),
            "set_clock_gating_check" => set_clock_gating_check(args),
            "set_clock_groups" => set_clock_groups(args),
            "set_clock_latency" => set_clock_latency(args),
            "set_sense" => set_sense(args),
            "set_clock_transition" => set_clock_transition(args),
            "set_clock_uncertainty" => set_clock_uncertainty(args),
            "set_data_check" => set_data_check(args),
            "set_disable_timing" => set_disable_timing(args),
            "set_false_path" => set_false_path(args),
            "set_ideal_latency" => set_ideal_latency(args),
            "set_ideal_network" => set_ideal_network(args),
            "set_ideal_transition" => set_ideal_transition(args),
            "set_input_delay" => set_input_delay(args),
            "set_max_delay" => set_max_delay(args),
            "set_max_time_borrow" => set_max_time_borrow(args),
            "set_min_delay" => set_min_delay(args),
            "set_min_pulse_width" => set_min_pulse_width(args),
            "set_multicycle_path" => set_multicycle_path(args),
            "set_output_delay" => set_output_delay(args),
            "set_propagated_clock" => set_propagated_clock(args),
            "set_case_analysis" => set_case_analysis(args),
            "set_drive" => set_drive(args),
            "set_driving_cell" => set_driving_cell(args),
            "set_fanout_load" => set_fanout_load(args),
            "set_input_transition" => set_input_transition(args),
            "set_load" => set_load(args),
            "set_logic_dc" => set_logic_dc(args),
            "set_logic_one" => set_logic_one(args),
            "set_logic_zero" => set_logic_zero(args),
            "set_max_area" => set_max_area(args),
            "set_max_capacitance" => set_max_capacitance(args),
            "set_max_fanout" => set_max_fanout(args),
            "set_max_transition" => set_max_transition(args),
            "set_min_capacitance" => set_min_capacitance(args),
            "set_operating_conditions" => set_operating_conditions(args),
            "set_port_fanout_number" => set_port_fanout_number(args),
            "set_resistance" => set_resistance(args),
            "set_timing_derate" => set_timing_derate(args),
            "set_voltage" => set_voltage(args),
            "set_wire_load_min_block_size" => set_wire_load_min_block_size(args),
            "set_wire_load_mode" => set_wire_load_mode(args),
            "set_wire_load_model" => set_wire_load_model(args),
            "set_wire_load_selection_group" => set_wire_load_selection_group(args),
            "create_voltage_area" => create_voltage_area(args),
            "set_level_shifter_strategy" => set_level_shifter_strategy(args),
            "set_level_shifter_threshold" => set_level_shifter_threshold(args),
            "set_max_dynamic_power" => set_max_dynamic_power(args),
            "set_max_leakage_power" => set_max_leakage_power(args),
            _ => return Err(SdcError::UnknownCommand(command.into())),
        }
    }
}

fn opt_arg(
    name: Argument,
    arg: Option<Argument>,
    tgt: Option<Argument>,
) -> Result<Option<Argument>, SdcError> {
    if arg.is_none() {
        return Err(SdcError::MissingOptArgument(name));
    }
    match tgt {
        Some(_) => Err(SdcError::DuplicatedArgument(name)),
        None => Ok(arg),
    }
}

fn opt_flg(name: Argument, tgt: bool) -> Result<bool, SdcError> {
    match tgt {
        true => Err(SdcError::DuplicatedArgument(name)),
        false => Ok(true),
    }
}

fn pos_args1(arg: Option<Argument>, tgt: Option<Argument>) -> Result<Option<Argument>, SdcError> {
    if arg.is_none() {
        return Err(SdcError::MissingPosArgument);
    }
    match tgt {
        Some(_) => Err(SdcError::TooManyArgument),
        None => Ok(arg),
    }
}

fn pos_args2(
    arg: Option<Argument>,
    tgt: (Option<Argument>, Option<Argument>),
) -> Result<(Option<Argument>, Option<Argument>), SdcError> {
    let (tgt0, tgt1) = tgt;
    if tgt0.is_none() {
        Ok((pos_args1(arg, tgt0)?, None))
    } else if tgt1.is_none() {
        Ok((tgt0, pos_args1(arg, tgt1)?))
    } else {
        Err(SdcError::TooManyArgument)
    }
}

fn mandatory(arg: Option<Argument>, name: &str) -> Result<Argument, SdcError> {
    arg.ok_or(SdcError::MissingMandatoryArgument(name.into()))
}

fn current_instance(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut instance = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        instance = pos_args1(Some(arg), instance)?;
    }

    Ok(Command::CurrentInstance(CurrentInstance { instance }))
}

fn expr(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut ret = vec![];

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        ret.push(arg);
    }

    Ok(Command::Expr(Expr { args: ret }))
}

fn list(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut ret = vec![];

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        ret.push(arg);
    }

    Ok(Command::List(List { args: ret }))
}

fn set_hierarchy_separator(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut separator = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        separator = pos_args1(Some(arg), separator)?;
    }

    let separator = mandatory(separator, "separator")?;

    Ok(Command::SetHierarchySeparator(SetHierarchySeparator {
        separator,
    }))
}

fn set(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut variable_name = None;
    let mut value = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (variable_name, value) = pos_args2(Some(arg), (variable_name, value))?;
    }

    let variable_name = mandatory(variable_name, "variable_name")?;
    let value = mandatory(value, "value")?;

    Ok(Command::Set(Set {
        variable_name,
        value,
    }))
}

fn set_units(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut capacitance = None;
    let mut resistance = None;
    let mut time = None;
    let mut voltage = None;
    let mut current = None;
    let mut power = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-capacitance" => capacitance = opt_arg(arg, iter.next(), capacitance)?,
            "-resistance" => resistance = opt_arg(arg, iter.next(), resistance)?,
            "-time" => time = opt_arg(arg, iter.next(), time)?,
            "-voltage" => voltage = opt_arg(arg, iter.next(), voltage)?,
            "-current" => current = opt_arg(arg, iter.next(), current)?,
            "-power" => power = opt_arg(arg, iter.next(), power)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::SetUnits(SetUnits {
        capacitance,
        resistance,
        time,
        voltage,
        current,
        power,
    }))
}

fn all_clocks(args: Vec<Argument>) -> Result<Command, SdcError> {
    if !args.is_empty() {
        return Err(SdcError::WrongArgument(args));
    }

    Ok(Command::AllClocks)
}

fn all_inputs(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut level_sensitive = false;
    let mut edge_triggered = false;
    let mut clock = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-level_sensitive" => level_sensitive = opt_flg(arg, level_sensitive)?,
            "-edge_triggered" => edge_triggered = opt_flg(arg, edge_triggered)?,
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::AllInputs(AllInputs {
        level_sensitive,
        edge_triggered,
        clock,
    }))
}

fn all_outputs(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut level_sensitive = false;
    let mut edge_triggered = false;
    let mut clock = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-level_sensitive" => level_sensitive = opt_flg(arg, level_sensitive)?,
            "-edge_triggered" => edge_triggered = opt_flg(arg, edge_triggered)?,
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::AllOutputs(AllOutputs {
        level_sensitive,
        edge_triggered,
        clock,
    }))
}

fn all_registers(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut hsc = None;
    let mut clock = None;
    let mut rise_clock = None;
    let mut fall_clock = None;
    let mut cells = false;
    let mut data_pins = false;
    let mut clock_pins = false;
    let mut slave_clock_pins = false;
    let mut async_pins = false;
    let mut output_pins = false;
    let mut level_sensitive = false;
    let mut edge_triggered = false;
    let mut master_slave = false;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-hsc" => hsc = opt_arg(arg, iter.next(), hsc)?,
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            "-rise_clock" => rise_clock = opt_arg(arg, iter.next(), rise_clock)?,
            "-fall_clock" => fall_clock = opt_arg(arg, iter.next(), fall_clock)?,
            "-cells" => cells = opt_flg(arg, cells)?,
            "-data_pins" => data_pins = opt_flg(arg, data_pins)?,
            "-clock_pins" => clock_pins = opt_flg(arg, clock_pins)?,
            "-slave_clock_pins" => slave_clock_pins = opt_flg(arg, slave_clock_pins)?,
            "-async_pins" => async_pins = opt_flg(arg, async_pins)?,
            "-output_pins" => output_pins = opt_flg(arg, output_pins)?,
            "-level_sensitive" => level_sensitive = opt_flg(arg, level_sensitive)?,
            "-edge_triggered" => edge_triggered = opt_flg(arg, edge_triggered)?,
            "-master_slave" => master_slave = opt_flg(arg, master_slave)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::AllRegisters(AllRegisters {
        hsc,
        clock,
        rise_clock,
        fall_clock,
        cells,
        data_pins,
        clock_pins,
        slave_clock_pins,
        async_pins,
        output_pins,
        level_sensitive,
        edge_triggered,
        master_slave,
    }))
}

fn current_design(args: Vec<Argument>) -> Result<Command, SdcError> {
    if !args.is_empty() {
        return Err(SdcError::WrongArgument(args));
    }

    Ok(Command::CurrentDesign)
}

fn get_cells(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut hierarchical = false;
    let mut regexp = false;
    let mut nocase = false;
    let mut of_objects = None;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-hierarchical" => hierarchical = opt_flg(arg, hierarchical)?,
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            "-of_objects" => of_objects = opt_arg(arg, iter.next(), of_objects)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetCells(GetCells {
        hierarchical,
        regexp,
        nocase,
        of_objects,
        patterns,
    }))
}

fn get_clocks(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut nocase = false;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetClocks(GetClocks {
        regexp,
        nocase,
        patterns,
    }))
}

fn get_lib_cells(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut hsc = None;
    let mut nocase = false;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-hsc" => hsc = opt_arg(arg, iter.next(), hsc)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetLibCells(GetLibCells {
        regexp,
        hsc,
        nocase,
        patterns,
    }))
}

fn get_lib_pins(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut nocase = false;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetLibPins(GetLibPins {
        regexp,
        nocase,
        patterns,
    }))
}

fn get_libs(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut nocase = false;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetLibs(GetLibs {
        regexp,
        nocase,
        patterns,
    }))
}

fn get_nets(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut hierarchical = false;
    let mut hsc = None;
    let mut regexp = false;
    let mut nocase = false;
    let mut of_objects = None;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-hierarchical" => hierarchical = opt_flg(arg, hierarchical)?,
            "-hsc" => hsc = opt_arg(arg, iter.next(), hsc)?,
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            "-of_objects" => of_objects = opt_arg(arg, iter.next(), of_objects)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetNets(GetNets {
        hierarchical,
        hsc,
        regexp,
        nocase,
        of_objects,
        patterns,
    }))
}

fn get_pins(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut hierarchical = false;
    let mut hsc = None;
    let mut regexp = false;
    let mut nocase = false;
    let mut of_objects = None;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-hierarchical" => hierarchical = opt_flg(arg, hierarchical)?,
            "-hsc" => hsc = opt_arg(arg, iter.next(), hsc)?,
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            "-of_objects" => of_objects = opt_arg(arg, iter.next(), of_objects)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetPins(GetPins {
        hierarchical,
        hsc,
        regexp,
        nocase,
        of_objects,
        patterns,
    }))
}

fn get_ports(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut nocase = false;
    let mut patterns = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-regexp" => regexp = opt_flg(arg, regexp)?,
            "-nocase" => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns)?,
        }
    }

    let patterns = mandatory(patterns, "patterns")?;

    Ok(Command::GetPorts(GetPorts {
        regexp,
        nocase,
        patterns,
    }))
}

fn create_clock(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut period = None;
    let mut name = None;
    let mut waveform = None;
    let mut add = false;
    let mut comment = None;
    let mut source_objects = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-period" => period = opt_arg(arg, iter.next(), period)?,
            "-name" => name = opt_arg(arg, iter.next(), name)?,
            "-waveform" => waveform = opt_arg(arg, iter.next(), waveform)?,
            "-add" => add = opt_flg(arg, add)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => source_objects = pos_args1(Some(arg), source_objects)?,
        }
    }

    let period = mandatory(period, "-period")?;
    let source_objects = mandatory(source_objects, "source_objects")?;

    Ok(Command::CreateClock(CreateClock {
        period,
        name,
        waveform,
        add,
        comment,
        source_objects,
    }))
}

fn create_generated_clock(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut name = None;
    let mut source = None;
    let mut edges = None;
    let mut divide_by = None;
    let mut multiply_by = None;
    let mut duty_cycle = None;
    let mut invert = false;
    let mut edge_shift = None;
    let mut add = false;
    let mut master_clock = None;
    let mut combinational = false;
    let mut comment = None;
    let mut source_objects = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-name" => name = opt_arg(arg, iter.next(), name)?,
            "-source" => source = opt_arg(arg, iter.next(), source)?,
            "-edges" => edges = opt_arg(arg, iter.next(), edges)?,
            "-divide_by" => divide_by = opt_arg(arg, iter.next(), divide_by)?,
            "-multiply_by" => multiply_by = opt_arg(arg, iter.next(), multiply_by)?,
            "-duty_cycle" => duty_cycle = opt_arg(arg, iter.next(), duty_cycle)?,
            "-invert" => invert = opt_flg(arg, invert)?,
            "-edge_shift" => edge_shift = opt_arg(arg, iter.next(), edge_shift)?,
            "-add" => add = opt_flg(arg, add)?,
            "-master_clock" => master_clock = opt_arg(arg, iter.next(), master_clock)?,
            "-combinational" => combinational = opt_flg(arg, combinational)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => source_objects = pos_args1(Some(arg), source_objects)?,
        }
    }

    let source = mandatory(source, "-source")?;
    let source_objects = mandatory(source_objects, "source_objects")?;

    Ok(Command::CreateGeneratedClock(CreateGeneratedClock {
        name,
        source,
        edges,
        divide_by,
        multiply_by,
        duty_cycle,
        invert,
        edge_shift,
        add,
        master_clock,
        combinational,
        comment,
        source_objects,
    }))
}

fn group_path(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut name = None;
    let mut default = false;
    let mut weight = None;
    let mut from = None;
    let mut rise_from = None;
    let mut fall_from = None;
    let mut to = None;
    let mut rise_to = None;
    let mut fall_to = None;
    let mut through = None;
    let mut rise_through = None;
    let mut fall_through = None;
    let mut comment = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-name" => name = opt_arg(arg, iter.next(), name)?,
            "-default" => default = opt_flg(arg, default)?,
            "-weight" => weight = opt_arg(arg, iter.next(), weight)?,
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-rise_from" => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            "-fall_from" => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            "-rise_to" => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            "-fall_to" => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            "-through" => through = opt_arg(arg, iter.next(), through)?,
            "-rise_through" => rise_through = opt_arg(arg, iter.next(), rise_through)?,
            "-fall_through" => fall_through = opt_arg(arg, iter.next(), fall_through)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::GroupPath(GroupPath {
        name,
        default,
        weight,
        from,
        rise_from,
        fall_from,
        to,
        rise_to,
        fall_to,
        through,
        rise_through,
        fall_through,
        comment,
    }))
}

fn set_clock_gating_check(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut setup = None;
    let mut hold = None;
    let mut rise = false;
    let mut fall = false;
    let mut high = false;
    let mut low = false;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-setup" => setup = opt_arg(arg, iter.next(), setup)?,
            "-hold" => hold = opt_arg(arg, iter.next(), hold)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-high" => high = opt_flg(arg, high)?,
            "-low" => low = opt_flg(arg, low)?,
            _ => object_list = pos_args1(Some(arg), object_list)?,
        }
    }

    Ok(Command::SetClockGatingCheck(SetClockGatingCheck {
        setup,
        hold,
        rise,
        fall,
        high,
        low,
        object_list,
    }))
}

fn set_clock_groups(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut group = None;
    let mut logically_exclusive = false;
    let mut phisically_exclusive = false;
    let mut asynchronous = false;
    let mut allow_paths = false;
    let mut name = None;
    let mut comment = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-group" => group = opt_arg(arg, iter.next(), group)?,
            "-logically_exclusive" => logically_exclusive = opt_flg(arg, logically_exclusive)?,
            "-phisically_exclusive" => phisically_exclusive = opt_flg(arg, phisically_exclusive)?,
            "-asynchronous" => asynchronous = opt_flg(arg, asynchronous)?,
            "-allow_paths" => allow_paths = opt_flg(arg, allow_paths)?,
            "-name" => name = opt_arg(arg, iter.next(), name)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    let group = mandatory(group, "-group")?;

    Ok(Command::SetClockGroups(SetClockGroups {
        group,
        logically_exclusive,
        phisically_exclusive,
        asynchronous,
        allow_paths,
        name,
        comment,
    }))
}

fn set_clock_latency(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut source = false;
    let mut dynamic = false;
    let mut late = false;
    let mut early = false;
    let mut clock = None;
    let mut delay = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            "-source" => source = opt_flg(arg, source)?,
            "-dynamic" => dynamic = opt_flg(arg, dynamic)?,
            "-late" => late = opt_flg(arg, late)?,
            "-early" => early = opt_flg(arg, early)?,
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            _ => (delay, object_list) = pos_args2(Some(arg), (delay, object_list))?,
        }
    }

    let delay = mandatory(delay, "delay")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetClockLatency(SetClockLatency {
        rise,
        fall,
        min,
        max,
        source,
        dynamic,
        late,
        early,
        clock,
        delay,
        object_list,
    }))
}

fn set_sense(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut r#type = None;
    let mut non_unate = false;
    let mut positive = false;
    let mut negative = false;
    let mut clock_leaf = false;
    let mut stop_propagation = false;
    let mut pulse = None;
    let mut clocks = None;
    let mut pin_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-type" => r#type = opt_arg(arg, iter.next(), r#type)?,
            "-non_unate" => non_unate = opt_flg(arg, non_unate)?,
            "-positive" => positive = opt_flg(arg, positive)?,
            "-negative" => negative = opt_flg(arg, negative)?,
            "-clock_leaf" => clock_leaf = opt_flg(arg, clock_leaf)?,
            "-stop_propagation" => stop_propagation = opt_flg(arg, stop_propagation)?,
            "-pulse" => pulse = opt_arg(arg, iter.next(), pulse)?,
            "-clocks" => clocks = opt_arg(arg, iter.next(), clocks)?,
            _ => pin_list = pos_args1(Some(arg), pin_list)?,
        }
    }

    let pin_list = mandatory(pin_list, "pin_list")?;

    Ok(Command::SetSense(SetSense {
        r#type,
        non_unate,
        positive,
        negative,
        clock_leaf,
        stop_propagation,
        pulse,
        clocks,
        pin_list,
    }))
}

fn set_clock_transition(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut transition = None;
    let mut clock_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            _ => (transition, clock_list) = pos_args2(Some(arg), (transition, clock_list))?,
        }
    }

    let transition = mandatory(transition, "transition")?;
    let clock_list = mandatory(clock_list, "clock_list")?;

    Ok(Command::SetClockTransition(SetClockTransition {
        rise,
        fall,
        min,
        max,
        transition,
        clock_list,
    }))
}

fn set_clock_uncertainty(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut from = None;
    let mut rise_from = None;
    let mut fall_from = None;
    let mut to = None;
    let mut rise_to = None;
    let mut fall_to = None;
    let mut rise = false;
    let mut fall = false;
    let mut setup = false;
    let mut hold = false;
    let mut uncertainty = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-rise_from" => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            "-fall_from" => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            "-rise_to" => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            "-fall_to" => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-setup" => setup = opt_flg(arg, setup)?,
            "-hold" => hold = opt_flg(arg, hold)?,
            _ => (uncertainty, object_list) = pos_args2(Some(arg), (uncertainty, object_list))?,
        }
    }

    let uncertainty = mandatory(uncertainty, "uncertainty")?;

    Ok(Command::SetClockUncertainty(SetClockUncertainty {
        from,
        rise_from,
        fall_from,
        to,
        rise_to,
        fall_to,
        rise,
        fall,
        setup,
        hold,
        uncertainty,
        object_list,
    }))
}

fn set_data_check(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut from = None;
    let mut to = None;
    let mut rise_from = None;
    let mut fall_from = None;
    let mut rise_to = None;
    let mut fall_to = None;
    let mut setup = false;
    let mut hold = false;
    let mut clock = None;
    let mut value = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            "-rise_from" => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            "-fall_from" => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            "-rise_to" => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            "-fall_to" => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            "-setup" => setup = opt_flg(arg, setup)?,
            "-hold" => hold = opt_flg(arg, hold)?,
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            _ => value = pos_args1(Some(arg), value)?,
        }
    }

    let value = mandatory(value, "value")?;

    Ok(Command::SetDataCheck(SetDataCheck {
        from,
        to,
        rise_from,
        fall_from,
        rise_to,
        fall_to,
        setup,
        hold,
        clock,
        value,
    }))
}

fn set_disable_timing(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut from = None;
    let mut to = None;
    let mut cell_pin_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            _ => cell_pin_list = pos_args1(Some(arg), cell_pin_list)?,
        }
    }

    let cell_pin_list = mandatory(cell_pin_list, "cell_pin_list")?;

    Ok(Command::SetDisableTiming(SetDisableTiming {
        from,
        to,
        cell_pin_list,
    }))
}

fn set_false_path(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut setup = false;
    let mut hold = false;
    let mut rise = false;
    let mut fall = false;
    let mut from = None;
    let mut to = None;
    let mut through = None;
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = None;
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = None;
    let mut comment = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-setup" => setup = opt_flg(arg, setup)?,
            "-hold" => hold = opt_flg(arg, hold)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            "-through" => through = opt_arg(arg, iter.next(), through)?,
            "-rise_from" => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            "-rise_to" => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            "-rise_through" => rise_through = opt_arg(arg, iter.next(), rise_through)?,
            "-fall_from" => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            "-fall_to" => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            "-fall_through" => fall_through = opt_arg(arg, iter.next(), fall_through)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::SetFalsePath(SetFalsePath {
        setup,
        hold,
        rise,
        fall,
        from,
        to,
        through,
        rise_from,
        rise_to,
        rise_through,
        fall_from,
        fall_to,
        fall_through,
        comment,
    }))
}

fn set_ideal_latency(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut delay = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            _ => (delay, object_list) = pos_args2(Some(arg), (delay, object_list))?,
        }
    }

    let delay = mandatory(delay, "delay")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetIdealLatency(SetIdealLatency {
        rise,
        fall,
        min,
        max,
        delay,
        object_list,
    }))
}

fn set_ideal_network(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut no_propagate = false;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-no_propagate" => no_propagate = opt_flg(arg, no_propagate)?,
            _ => object_list = pos_args1(Some(arg), object_list)?,
        }
    }

    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetIdealNetwork(SetIdealNetwork {
        no_propagate,
        object_list,
    }))
}

fn set_ideal_transition(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut transition_time = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            _ => {
                (transition_time, object_list) =
                    pos_args2(Some(arg), (transition_time, object_list))?
            }
        }
    }

    let transition_time = mandatory(transition_time, "transition_time")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetIdealTransition(SetIdealTransition {
        rise,
        fall,
        min,
        max,
        transition_time,
        object_list,
    }))
}

fn set_input_delay(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut clock = None;
    let mut reference_pin = None;
    let mut clock_fall = false;
    let mut level_sensitive = false;
    let mut rise = false;
    let mut fall = false;
    let mut max = false;
    let mut min = false;
    let mut add_delay = false;
    let mut network_latency_included = false;
    let mut source_latency_included = false;
    let mut delay_value = None;
    let mut port_pin_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            "-reference_pin" => reference_pin = opt_arg(arg, iter.next(), reference_pin)?,
            "-clock_fall" => clock_fall = opt_flg(arg, clock_fall)?,
            "-level_sensitive" => level_sensitive = opt_flg(arg, level_sensitive)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-max" => max = opt_flg(arg, max)?,
            "-min" => min = opt_flg(arg, min)?,
            "-add_delay" => add_delay = opt_flg(arg, add_delay)?,
            "-network_latency_included" => {
                network_latency_included = opt_flg(arg, network_latency_included)?
            }
            "-source_latency_included" => {
                source_latency_included = opt_flg(arg, source_latency_included)?
            }
            _ => (delay_value, port_pin_list) = pos_args2(Some(arg), (delay_value, port_pin_list))?,
        }
    }

    let delay_value = mandatory(delay_value, "delay_value")?;
    let port_pin_list = mandatory(port_pin_list, "port_pin_list")?;

    Ok(Command::SetInputDelay(SetInputDelay {
        clock,
        reference_pin,
        clock_fall,
        level_sensitive,
        rise,
        fall,
        max,
        min,
        add_delay,
        network_latency_included,
        source_latency_included,
        delay_value,
        port_pin_list,
    }))
}

fn set_max_delay(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut from = None;
    let mut to = None;
    let mut through = None;
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = None;
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = None;
    let mut ignore_clock_latency = false;
    let mut comment = None;
    let mut delay_value = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            "-through" => through = opt_arg(arg, iter.next(), through)?,
            "-rise_from" => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            "-rise_to" => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            "-rise_through" => rise_through = opt_arg(arg, iter.next(), rise_through)?,
            "-fall_from" => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            "-fall_to" => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            "-fall_through" => fall_through = opt_arg(arg, iter.next(), fall_through)?,
            "-ignore_clock_latency" => ignore_clock_latency = opt_flg(arg, ignore_clock_latency)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => delay_value = pos_args1(Some(arg), delay_value)?,
        }
    }

    let delay_value = mandatory(delay_value, "delay_value")?;

    Ok(Command::SetMaxDelay(SetMaxDelay {
        rise,
        fall,
        from,
        to,
        through,
        rise_from,
        rise_to,
        rise_through,
        fall_from,
        fall_to,
        fall_through,
        ignore_clock_latency,
        comment,
        delay_value,
    }))
}

fn set_max_time_borrow(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut delay_value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (delay_value, object_list) = pos_args2(Some(arg), (delay_value, object_list))?,
        }
    }

    let delay_value = mandatory(delay_value, "delay_value")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetMaxTimeBorrow(SetMaxTimeBorrow {
        delay_value,
        object_list,
    }))
}

fn set_min_delay(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut from = None;
    let mut to = None;
    let mut through = None;
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = None;
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = None;
    let mut ignore_clock_latency = false;
    let mut comment = None;
    let mut delay_value = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            "-through" => through = opt_arg(arg, iter.next(), through)?,
            "-rise_from" => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            "-rise_to" => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            "-rise_through" => rise_through = opt_arg(arg, iter.next(), rise_through)?,
            "-fall_from" => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            "-fall_to" => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            "-fall_through" => fall_through = opt_arg(arg, iter.next(), fall_through)?,
            "-ignore_clock_latency" => ignore_clock_latency = opt_flg(arg, ignore_clock_latency)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => delay_value = pos_args1(Some(arg), delay_value)?,
        }
    }

    let delay_value = mandatory(delay_value, "delay_value")?;

    Ok(Command::SetMinDelay(SetMinDelay {
        rise,
        fall,
        from,
        to,
        through,
        rise_from,
        rise_to,
        rise_through,
        fall_from,
        fall_to,
        fall_through,
        ignore_clock_latency,
        comment,
        delay_value,
    }))
}

fn set_min_pulse_width(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut low = false;
    let mut high = false;
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-low" => low = opt_flg(arg, low)?,
            "-high" => high = opt_flg(arg, high)?,
            _ => (value, object_list) = pos_args2(Some(arg), (value, object_list))?,
        }
    }

    let value = mandatory(value, "value")?;

    Ok(Command::SetMinPulseWidth(SetMinPulseWidth {
        low,
        high,
        value,
        object_list,
    }))
}

fn set_multicycle_path(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut setup = false;
    let mut hold = false;
    let mut rise = false;
    let mut fall = false;
    let mut start = false;
    let mut end = false;
    let mut from = None;
    let mut to = None;
    let mut through = None;
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = None;
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = None;
    let mut comment = None;
    let mut path_multiplier = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-setup" => setup = opt_flg(arg, setup)?,
            "-hold" => hold = opt_flg(arg, hold)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-start" => start = opt_flg(arg, start)?,
            "-end" => end = opt_flg(arg, end)?,
            "-from" => from = opt_arg(arg, iter.next(), from)?,
            "-to" => to = opt_arg(arg, iter.next(), to)?,
            "-through" => through = opt_arg(arg, iter.next(), through)?,
            "-rise_from" => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            "-rise_to" => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            "-rise_through" => rise_through = opt_arg(arg, iter.next(), rise_through)?,
            "-fall_from" => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            "-fall_to" => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            "-fall_through" => fall_through = opt_arg(arg, iter.next(), fall_through)?,
            "-comment" => comment = opt_arg(arg, iter.next(), comment)?,
            _ => path_multiplier = pos_args1(Some(arg), path_multiplier)?,
        }
    }

    let path_multiplier = mandatory(path_multiplier, "path_multiplier")?;

    Ok(Command::SetMulticyclePath(SetMulticyclePath {
        setup,
        hold,
        rise,
        fall,
        start,
        end,
        from,
        to,
        through,
        rise_from,
        rise_to,
        rise_through,
        fall_from,
        fall_to,
        fall_through,
        comment,
        path_multiplier,
    }))
}

fn set_output_delay(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut clock = None;
    let mut reference_pin = None;
    let mut clock_fall = false;
    let mut level_sensitive = false;
    let mut rise = false;
    let mut fall = false;
    let mut max = false;
    let mut min = false;
    let mut add_delay = false;
    let mut network_latency_included = false;
    let mut source_latency_included = false;
    let mut delay_value = None;
    let mut port_pin_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            "-reference_pin" => reference_pin = opt_arg(arg, iter.next(), reference_pin)?,
            "-clock_fall" => clock_fall = opt_flg(arg, clock_fall)?,
            "-level_sensitive" => level_sensitive = opt_flg(arg, level_sensitive)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-max" => max = opt_flg(arg, max)?,
            "-min" => min = opt_flg(arg, min)?,
            "-add_delay" => add_delay = opt_flg(arg, add_delay)?,
            "-network_latency_included" => {
                network_latency_included = opt_flg(arg, network_latency_included)?
            }
            "-source_latency_included" => {
                source_latency_included = opt_flg(arg, source_latency_included)?
            }
            _ => (delay_value, port_pin_list) = pos_args2(Some(arg), (delay_value, port_pin_list))?,
        }
    }

    let delay_value = mandatory(delay_value, "delay_value")?;
    let port_pin_list = mandatory(port_pin_list, "port_pin_list")?;

    Ok(Command::SetOutputDelay(SetOutputDelay {
        clock,
        reference_pin,
        clock_fall,
        level_sensitive,
        rise,
        fall,
        max,
        min,
        add_delay,
        network_latency_included,
        source_latency_included,
        delay_value,
        port_pin_list,
    }))
}

fn set_propagated_clock(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => object_list = pos_args1(Some(arg), object_list)?,
        }
    }

    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetPropagatedClock(SetPropagatedClock {
        object_list,
    }))
}

fn set_case_analysis(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut value = None;
    let mut port_or_pin_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (value, port_or_pin_list) = pos_args2(Some(arg), (value, port_or_pin_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let port_or_pin_list = mandatory(port_or_pin_list, "port_or_pin_list")?;

    Ok(Command::SetCaseAnalysis(SetCaseAnalysis {
        value,
        port_or_pin_list,
    }))
}

fn set_drive(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut resistance = None;
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            _ => (resistance, port_list) = pos_args2(Some(arg), (resistance, port_list))?,
        }
    }

    let resistance = mandatory(resistance, "resistance")?;
    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetDrive(SetDrive {
        rise,
        fall,
        min,
        max,
        resistance,
        port_list,
    }))
}

fn set_driving_cell(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut lib_cell = None;
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut library = None;
    let mut pin = None;
    let mut from_pin = None;
    let mut dont_scale = false;
    let mut no_design_rule = false;
    let mut clock = None;
    let mut clock_fall = false;
    let mut input_transition_rise = None;
    let mut input_transition_fall = None;
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-lib_cell" => lib_cell = opt_arg(arg, iter.next(), lib_cell)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            "-library" => library = opt_arg(arg, iter.next(), library)?,
            "-pin" => pin = opt_arg(arg, iter.next(), pin)?,
            "-from_pin" => from_pin = opt_arg(arg, iter.next(), from_pin)?,
            "-dont_scale" => dont_scale = opt_flg(arg, dont_scale)?,
            "-no_design_rule" => no_design_rule = opt_flg(arg, no_design_rule)?,
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            "-clock_fall" => clock_fall = opt_flg(arg, clock_fall)?,
            "-input_transition_rise" => {
                input_transition_rise = opt_arg(arg, iter.next(), input_transition_rise)?
            }
            "-input_transition_fall" => {
                input_transition_fall = opt_arg(arg, iter.next(), input_transition_fall)?
            }
            _ => port_list = pos_args1(Some(arg), port_list)?,
        }
    }

    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetDrivingCell(SetDrivingCell {
        lib_cell,
        rise,
        fall,
        min,
        max,
        library,
        pin,
        from_pin,
        dont_scale,
        no_design_rule,
        clock,
        clock_fall,
        input_transition_rise,
        input_transition_fall,
        port_list,
    }))
}

fn set_fanout_load(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut value = None;
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (value, port_list) = pos_args2(Some(arg), (value, port_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetFanoutLoad(SetFanoutLoad { value, port_list }))
}

fn set_input_transition(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut clock = None;
    let mut clock_fall = false;
    let mut transition = None;
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            "-clock" => clock = opt_arg(arg, iter.next(), clock)?,
            "-clock_fall" => clock_fall = opt_flg(arg, clock_fall)?,
            _ => (transition, port_list) = pos_args2(Some(arg), (transition, port_list))?,
        }
    }

    let transition = mandatory(transition, "transition")?;
    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetInputTransition(SetInputTransition {
        rise,
        fall,
        min,
        max,
        clock,
        clock_fall,
        transition,
        port_list,
    }))
}

fn set_load(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut min = false;
    let mut max = false;
    let mut subtract_pin_load = false;
    let mut pin_load = false;
    let mut wire_load = false;
    let mut value = None;
    let mut objects = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            "-subtract_pin_load" => subtract_pin_load = opt_flg(arg, subtract_pin_load)?,
            "-pin_load" => pin_load = opt_flg(arg, pin_load)?,
            "-wire_load" => wire_load = opt_flg(arg, wire_load)?,
            _ => (value, objects) = pos_args2(Some(arg), (value, objects))?,
        }
    }

    let value = mandatory(value, "value")?;
    let objects = mandatory(objects, "objects")?;

    Ok(Command::SetLoad(SetLoad {
        min,
        max,
        subtract_pin_load,
        pin_load,
        wire_load,
        value,
        objects,
    }))
}

fn set_logic_dc(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => port_list = pos_args1(Some(arg), port_list)?,
        }
    }

    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetLogicDc(SetLogicDc { port_list }))
}

fn set_logic_one(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => port_list = pos_args1(Some(arg), port_list)?,
        }
    }

    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetLogicOne(SetLogicOne { port_list }))
}

fn set_logic_zero(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => port_list = pos_args1(Some(arg), port_list)?,
        }
    }

    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetLogicZero(SetLogicZero { port_list }))
}

fn set_max_area(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut area_value = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => area_value = pos_args1(Some(arg), area_value)?,
        }
    }

    let area_value = mandatory(area_value, "area_value")?;

    Ok(Command::SetMaxArea(SetMaxArea { area_value }))
}

fn set_max_capacitance(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (value, object_list) = pos_args2(Some(arg), (value, object_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetMaxCapacitance(SetMaxCapacitance {
        value,
        object_list,
    }))
}

fn set_max_fanout(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (value, object_list) = pos_args2(Some(arg), (value, object_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetMaxFanout(SetMaxFanout { value, object_list }))
}

fn set_max_transition(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut clock_path = false;
    let mut data_path = false;
    let mut rise = false;
    let mut fall = false;
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-clock_path" => clock_path = opt_flg(arg, clock_path)?,
            "-data_path" => data_path = opt_flg(arg, data_path)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            _ => (value, object_list) = pos_args2(Some(arg), (value, object_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetMaxTransition(SetMaxTransition {
        clock_path,
        data_path,
        rise,
        fall,
        value,
        object_list,
    }))
}

fn set_min_capacitance(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (value, object_list) = pos_args2(Some(arg), (value, object_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let object_list = mandatory(object_list, "object_list")?;

    Ok(Command::SetMinCapacitance(SetMinCapacitance {
        value,
        object_list,
    }))
}

fn set_operating_conditions(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut library = None;
    let mut analysis_type = None;
    let mut max = None;
    let mut min = None;
    let mut max_library = None;
    let mut min_library = None;
    let mut object_list = None;
    let mut condition = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-library" => library = opt_arg(arg, iter.next(), library)?,
            "-analysis_type" => analysis_type = opt_arg(arg, iter.next(), analysis_type)?,
            "-max" => max = opt_arg(arg, iter.next(), max)?,
            "-min" => min = opt_arg(arg, iter.next(), min)?,
            "-max_library" => max_library = opt_arg(arg, iter.next(), max_library)?,
            "-min_library" => min_library = opt_arg(arg, iter.next(), min_library)?,
            "-object_list" => object_list = opt_arg(arg, iter.next(), object_list)?,
            _ => condition = pos_args1(Some(arg), condition)?,
        }
    }

    Ok(Command::SetOperatingConditions(SetOperatingConditions {
        library,
        analysis_type,
        max,
        min,
        max_library,
        min_library,
        object_list,
        condition,
    }))
}

fn set_port_fanout_number(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut value = None;
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (value, port_list) = pos_args2(Some(arg), (value, port_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let port_list = mandatory(port_list, "port_list")?;

    Ok(Command::SetPortFanoutNumber(SetPortFanoutNumber {
        value,
        port_list,
    }))
}

fn set_resistance(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut min = false;
    let mut max = false;
    let mut value = None;
    let mut net_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            _ => (value, net_list) = pos_args2(Some(arg), (value, net_list))?,
        }
    }

    let value = mandatory(value, "value")?;
    let net_list = mandatory(net_list, "net_list")?;

    Ok(Command::SetResistance(SetResistance {
        min,
        max,
        value,
        net_list,
    }))
}

fn set_timing_derate(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut cell_delay = false;
    let mut cell_check = false;
    let mut net_delay = false;
    let mut data = false;
    let mut clock = false;
    let mut early = false;
    let mut late = false;
    let mut rise = false;
    let mut fall = false;
    let mut r#static = false;
    let mut dynamic = false;
    let mut increment = false;
    let mut derate_value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-cell_delay" => cell_delay = opt_flg(arg, cell_delay)?,
            "-cell_check" => cell_check = opt_flg(arg, cell_check)?,
            "-net_delay" => net_delay = opt_flg(arg, net_delay)?,
            "-data" => data = opt_flg(arg, data)?,
            "-clock" => clock = opt_flg(arg, clock)?,
            "-early" => early = opt_flg(arg, early)?,
            "-late" => late = opt_flg(arg, late)?,
            "-rise" => rise = opt_flg(arg, rise)?,
            "-fall" => fall = opt_flg(arg, fall)?,
            "-static" => r#static = opt_flg(arg, r#static)?,
            "-dynamic" => dynamic = opt_flg(arg, dynamic)?,
            "-increment" => increment = opt_flg(arg, increment)?,
            _ => (derate_value, object_list) = pos_args2(Some(arg), (derate_value, object_list))?,
        }
    }

    let derate_value = mandatory(derate_value, "derate_value")?;

    Ok(Command::SetTimingDerate(SetTimingDerate {
        cell_delay,
        cell_check,
        net_delay,
        data,
        clock,
        early,
        late,
        rise,
        fall,
        r#static,
        dynamic,
        increment,
        derate_value,
        object_list,
    }))
}

fn set_voltage(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut min = None;
    let mut object_list = None;
    let mut max_case_voltage = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-min" => min = opt_arg(arg, iter.next(), min)?,
            "-object_list" => object_list = opt_arg(arg, iter.next(), object_list)?,
            _ => max_case_voltage = pos_args1(Some(arg), max_case_voltage)?,
        }
    }

    let max_case_voltage = mandatory(max_case_voltage, "max_case_voltage")?;

    Ok(Command::SetVoltage(SetVoltage {
        min,
        object_list,
        max_case_voltage,
    }))
}

fn set_wire_load_min_block_size(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut size = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => size = pos_args1(Some(arg), size)?,
        }
    }

    let size = mandatory(size, "size")?;

    Ok(Command::SetWireLoadMinBlockSize(SetWireLoadMinBlockSize {
        size,
    }))
}

fn set_wire_load_mode(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut mode_name = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => mode_name = pos_args1(Some(arg), mode_name)?,
        }
    }

    let mode_name = mandatory(mode_name, "mode_name")?;

    Ok(Command::SetWireLoadMode(SetWireLoadMode { mode_name }))
}

fn set_wire_load_model(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut name = None;
    let mut library = None;
    let mut min = false;
    let mut max = false;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-name" => name = opt_arg(arg, iter.next(), name)?,
            "-library" => library = opt_arg(arg, iter.next(), library)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            _ => object_list = pos_args1(Some(arg), object_list)?,
        }
    }

    let name = mandatory(name, "name")?;

    Ok(Command::SetWireLoadModel(SetWireLoadModel {
        name,
        library,
        min,
        max,
        object_list,
    }))
}

fn set_wire_load_selection_group(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut library = None;
    let mut min = false;
    let mut max = false;
    let mut group_name = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-library" => library = opt_arg(arg, iter.next(), library)?,
            "-min" => min = opt_flg(arg, min)?,
            "-max" => max = opt_flg(arg, max)?,
            _ => (group_name, object_list) = pos_args2(Some(arg), (group_name, object_list))?,
        }
    }

    let group_name = mandatory(group_name, "group_name")?;

    Ok(Command::SetWireLoadSelectionGroup(
        SetWireLoadSelectionGroup {
            library,
            min,
            max,
            group_name,
            object_list,
        },
    ))
}

fn create_voltage_area(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut name = None;
    let mut coordinate = None;
    let mut guard_band_x = None;
    let mut guard_band_y = None;
    let mut cell_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-name" => name = opt_arg(arg, iter.next(), name)?,
            "-coordinate" => coordinate = opt_arg(arg, iter.next(), coordinate)?,
            "-guard_band_x" => guard_band_x = opt_arg(arg, iter.next(), guard_band_x)?,
            "-guard_band_y" => guard_band_y = opt_arg(arg, iter.next(), guard_band_y)?,
            _ => cell_list = pos_args1(Some(arg), cell_list)?,
        }
    }

    let name = mandatory(name, "name")?;
    let cell_list = mandatory(cell_list, "cell_list")?;

    Ok(Command::CreateVoltageArea(CreateVoltageArea {
        name,
        coordinate,
        guard_band_x,
        guard_band_y,
        cell_list,
    }))
}

fn set_level_shifter_strategy(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut rule = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-rule" => rule = opt_arg(arg, iter.next(), rule)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::SetLevelShifterStrategy(SetLevelShifterStrategy {
        rule,
    }))
}

fn set_level_shifter_threshold(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut voltage = None;
    let mut percent = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-voltage" => voltage = opt_arg(arg, iter.next(), voltage)?,
            "-percent" => percent = opt_arg(arg, iter.next(), percent)?,
            _ => return Err(SdcError::WrongArgument(vec![arg])),
        }
    }

    Ok(Command::SetLevelShifterThreshold(
        SetLevelShifterThreshold { voltage, percent },
    ))
}

fn set_max_dynamic_power(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut power = None;
    let mut unit = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (power, unit) = pos_args2(Some(arg), (power, unit))?,
        }
    }

    let power = mandatory(power, "power")?;

    Ok(Command::SetMaxDynamicPower(SetMaxDynamicPower {
        power,
        unit,
    }))
}

fn set_max_leakage_power(args: Vec<Argument>) -> Result<Command, SdcError> {
    let mut power = None;
    let mut unit = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            _ => (power, unit) = pos_args2(Some(arg), (power, unit))?,
        }
    }

    let power = mandatory(power, "power")?;

    Ok(Command::SetMaxLeakagePower(SetMaxLeakagePower {
        power,
        unit,
    }))
}
