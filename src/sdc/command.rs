#![allow(clippy::while_let_on_iterator)]

use crate::parser::sdc_grammar_trait as grammar;
use crate::sdc::util::*;
use crate::sdc::SdcVersion::*;
use crate::sdc::{Argument, Location, SdcError, SdcVersion};
use std::fmt;
use std::sync::OnceLock;

/// SDC command
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    AllClocks(AllClocks),
    AllInputs(AllInputs),
    AllOutputs(AllOutputs),
    AllRegisters(AllRegisters),
    CreateClock(CreateClock),
    CreateGeneratedClock(CreateGeneratedClock),
    CreateVoltageArea(CreateVoltageArea),
    CurrentDesign(CurrentDesign),
    CurrentInstance(CurrentInstance),
    Expr(Expr),
    GetCells(GetCells),
    GetClocks(GetClocks),
    GetLibCells(GetLibCells),
    GetLibPins(GetLibPins),
    GetLibs(GetLibs),
    GetNets(GetNets),
    GetPins(GetPins),
    GetPorts(GetPorts),
    GroupPath(GroupPath),
    List(List),
    Set(Set),
    SetCaseAnalysis(SetCaseAnalysis),
    SetClockGatingCheck(SetClockGatingCheck),
    SetClockGroups(SetClockGroups),
    SetClockLatency(SetClockLatency),
    SetClockSense(SetClockSense),
    SetClockTransition(SetClockTransition),
    SetClockUncertainty(SetClockUncertainty),
    SetDataCheck(SetDataCheck),
    SetDisableTiming(SetDisableTiming),
    SetDrive(SetDrive),
    SetDrivingCell(SetDrivingCell),
    SetFalsePath(SetFalsePath),
    SetFanoutLoad(SetFanoutLoad),
    SetHierarchySeparator(SetHierarchySeparator),
    SetIdealLatency(SetIdealLatency),
    SetIdealNetwork(SetIdealNetwork),
    SetIdealTransition(SetIdealTransition),
    SetInputDelay(SetInputDelay),
    SetInputTransition(SetInputTransition),
    SetLevelShifterStrategy(SetLevelShifterStrategy),
    SetLevelShifterThreshold(SetLevelShifterThreshold),
    SetLoad(SetLoad),
    SetLogicDc(SetLogicDc),
    SetLogicOne(SetLogicOne),
    SetLogicZero(SetLogicZero),
    SetMaxArea(SetMaxArea),
    SetMaxCapacitance(SetMaxCapacitance),
    SetMaxDelay(SetMaxDelay),
    SetMaxDynamicPower(SetMaxDynamicPower),
    SetMaxFanout(SetMaxFanout),
    SetMaxLeakagePower(SetMaxLeakagePower),
    SetMaxTimeBorrow(SetMaxTimeBorrow),
    SetMaxTransition(SetMaxTransition),
    SetMinCapacitance(SetMinCapacitance),
    SetMinDelay(SetMinDelay),
    SetMinPorosity(SetMinPorosity),
    SetMinPulseWidth(SetMinPulseWidth),
    SetMulticyclePath(SetMulticyclePath),
    SetOperatingConditions(SetOperatingConditions),
    SetOutputDelay(SetOutputDelay),
    SetPortFanoutNumber(SetPortFanoutNumber),
    SetPropagatedClock(SetPropagatedClock),
    SetResistance(SetResistance),
    SetSense(SetSense),
    SetTimingDerate(SetTimingDerate),
    SetUnits(SetUnits),
    SetVoltage(SetVoltage),
    SetWireLoadMinBlockSize(SetWireLoadMinBlockSize),
    SetWireLoadMode(SetWireLoadMode),
    SetWireLoadModel(SetWireLoadModel),
    SetWireLoadSelectionGroup(SetWireLoadSelectionGroup),
    Unknown(Unknown),
}

impl Command {
    pub fn location(&self) -> &Location {
        match self {
            Command::AllClocks(x) => &x.location,
            Command::AllInputs(x) => &x.location,
            Command::AllOutputs(x) => &x.location,
            Command::AllRegisters(x) => &x.location,
            Command::CreateClock(x) => &x.location,
            Command::CreateGeneratedClock(x) => &x.location,
            Command::CreateVoltageArea(x) => &x.location,
            Command::CurrentDesign(x) => &x.location,
            Command::CurrentInstance(x) => &x.location,
            Command::Expr(x) => &x.location,
            Command::GetCells(x) => &x.location,
            Command::GetClocks(x) => &x.location,
            Command::GetLibCells(x) => &x.location,
            Command::GetLibPins(x) => &x.location,
            Command::GetLibs(x) => &x.location,
            Command::GetNets(x) => &x.location,
            Command::GetPins(x) => &x.location,
            Command::GetPorts(x) => &x.location,
            Command::GroupPath(x) => &x.location,
            Command::List(x) => &x.location,
            Command::Set(x) => &x.location,
            Command::SetCaseAnalysis(x) => &x.location,
            Command::SetClockGatingCheck(x) => &x.location,
            Command::SetClockGroups(x) => &x.location,
            Command::SetClockLatency(x) => &x.location,
            Command::SetClockSense(x) => &x.location,
            Command::SetClockTransition(x) => &x.location,
            Command::SetClockUncertainty(x) => &x.location,
            Command::SetDataCheck(x) => &x.location,
            Command::SetDisableTiming(x) => &x.location,
            Command::SetDrive(x) => &x.location,
            Command::SetDrivingCell(x) => &x.location,
            Command::SetFalsePath(x) => &x.location,
            Command::SetFanoutLoad(x) => &x.location,
            Command::SetHierarchySeparator(x) => &x.location,
            Command::SetIdealLatency(x) => &x.location,
            Command::SetIdealNetwork(x) => &x.location,
            Command::SetIdealTransition(x) => &x.location,
            Command::SetInputDelay(x) => &x.location,
            Command::SetInputTransition(x) => &x.location,
            Command::SetLevelShifterStrategy(x) => &x.location,
            Command::SetLevelShifterThreshold(x) => &x.location,
            Command::SetLoad(x) => &x.location,
            Command::SetLogicDc(x) => &x.location,
            Command::SetLogicOne(x) => &x.location,
            Command::SetLogicZero(x) => &x.location,
            Command::SetMaxArea(x) => &x.location,
            Command::SetMaxCapacitance(x) => &x.location,
            Command::SetMaxDelay(x) => &x.location,
            Command::SetMaxDynamicPower(x) => &x.location,
            Command::SetMaxFanout(x) => &x.location,
            Command::SetMaxLeakagePower(x) => &x.location,
            Command::SetMaxTimeBorrow(x) => &x.location,
            Command::SetMaxTransition(x) => &x.location,
            Command::SetMinCapacitance(x) => &x.location,
            Command::SetMinDelay(x) => &x.location,
            Command::SetMinPorosity(x) => &x.location,
            Command::SetMinPulseWidth(x) => &x.location,
            Command::SetMulticyclePath(x) => &x.location,
            Command::SetOperatingConditions(x) => &x.location,
            Command::SetOutputDelay(x) => &x.location,
            Command::SetPortFanoutNumber(x) => &x.location,
            Command::SetPropagatedClock(x) => &x.location,
            Command::SetResistance(x) => &x.location,
            Command::SetSense(x) => &x.location,
            Command::SetTimingDerate(x) => &x.location,
            Command::SetUnits(x) => &x.location,
            Command::SetVoltage(x) => &x.location,
            Command::SetWireLoadMinBlockSize(x) => &x.location,
            Command::SetWireLoadMode(x) => &x.location,
            Command::SetWireLoadModel(x) => &x.location,
            Command::SetWireLoadSelectionGroup(x) => &x.location,
            Command::Unknown(x) => &x.location,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::AllClocks(x) => x.fmt(f),
            Command::AllInputs(x) => x.fmt(f),
            Command::AllOutputs(x) => x.fmt(f),
            Command::AllRegisters(x) => x.fmt(f),
            Command::CreateClock(x) => x.fmt(f),
            Command::CreateGeneratedClock(x) => x.fmt(f),
            Command::CreateVoltageArea(x) => x.fmt(f),
            Command::CurrentDesign(x) => x.fmt(f),
            Command::CurrentInstance(x) => x.fmt(f),
            Command::Expr(x) => x.fmt(f),
            Command::GetCells(x) => x.fmt(f),
            Command::GetClocks(x) => x.fmt(f),
            Command::GetLibCells(x) => x.fmt(f),
            Command::GetLibPins(x) => x.fmt(f),
            Command::GetLibs(x) => x.fmt(f),
            Command::GetNets(x) => x.fmt(f),
            Command::GetPins(x) => x.fmt(f),
            Command::GetPorts(x) => x.fmt(f),
            Command::GroupPath(x) => x.fmt(f),
            Command::List(x) => x.fmt(f),
            Command::Set(x) => x.fmt(f),
            Command::SetCaseAnalysis(x) => x.fmt(f),
            Command::SetClockGatingCheck(x) => x.fmt(f),
            Command::SetClockGroups(x) => x.fmt(f),
            Command::SetClockLatency(x) => x.fmt(f),
            Command::SetClockSense(x) => x.fmt(f),
            Command::SetClockTransition(x) => x.fmt(f),
            Command::SetClockUncertainty(x) => x.fmt(f),
            Command::SetDataCheck(x) => x.fmt(f),
            Command::SetDisableTiming(x) => x.fmt(f),
            Command::SetDrive(x) => x.fmt(f),
            Command::SetDrivingCell(x) => x.fmt(f),
            Command::SetFalsePath(x) => x.fmt(f),
            Command::SetFanoutLoad(x) => x.fmt(f),
            Command::SetHierarchySeparator(x) => x.fmt(f),
            Command::SetIdealLatency(x) => x.fmt(f),
            Command::SetIdealNetwork(x) => x.fmt(f),
            Command::SetIdealTransition(x) => x.fmt(f),
            Command::SetInputDelay(x) => x.fmt(f),
            Command::SetInputTransition(x) => x.fmt(f),
            Command::SetLevelShifterStrategy(x) => x.fmt(f),
            Command::SetLevelShifterThreshold(x) => x.fmt(f),
            Command::SetLoad(x) => x.fmt(f),
            Command::SetLogicDc(x) => x.fmt(f),
            Command::SetLogicOne(x) => x.fmt(f),
            Command::SetLogicZero(x) => x.fmt(f),
            Command::SetMaxArea(x) => x.fmt(f),
            Command::SetMaxCapacitance(x) => x.fmt(f),
            Command::SetMaxDelay(x) => x.fmt(f),
            Command::SetMaxDynamicPower(x) => x.fmt(f),
            Command::SetMaxFanout(x) => x.fmt(f),
            Command::SetMaxLeakagePower(x) => x.fmt(f),
            Command::SetMaxTimeBorrow(x) => x.fmt(f),
            Command::SetMaxTransition(x) => x.fmt(f),
            Command::SetMinCapacitance(x) => x.fmt(f),
            Command::SetMinDelay(x) => x.fmt(f),
            Command::SetMinPorosity(x) => x.fmt(f),
            Command::SetMinPulseWidth(x) => x.fmt(f),
            Command::SetMulticyclePath(x) => x.fmt(f),
            Command::SetOperatingConditions(x) => x.fmt(f),
            Command::SetOutputDelay(x) => x.fmt(f),
            Command::SetPortFanoutNumber(x) => x.fmt(f),
            Command::SetPropagatedClock(x) => x.fmt(f),
            Command::SetResistance(x) => x.fmt(f),
            Command::SetSense(x) => x.fmt(f),
            Command::SetTimingDerate(x) => x.fmt(f),
            Command::SetUnits(x) => x.fmt(f),
            Command::SetVoltage(x) => x.fmt(f),
            Command::SetWireLoadMinBlockSize(x) => x.fmt(f),
            Command::SetWireLoadMode(x) => x.fmt(f),
            Command::SetWireLoadModel(x) => x.fmt(f),
            Command::SetWireLoadSelectionGroup(x) => x.fmt(f),
            Command::Unknown(x) => x.fmt(f),
        }
    }
}

impl Validate for Command {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        match self {
            Command::AllClocks(x) => x.validate(version),
            Command::AllInputs(x) => x.validate(version),
            Command::AllOutputs(x) => x.validate(version),
            Command::AllRegisters(x) => x.validate(version),
            Command::CreateClock(x) => x.validate(version),
            Command::CreateGeneratedClock(x) => x.validate(version),
            Command::CreateVoltageArea(x) => x.validate(version),
            Command::CurrentDesign(x) => x.validate(version),
            Command::CurrentInstance(x) => x.validate(version),
            Command::Expr(x) => x.validate(version),
            Command::GetCells(x) => x.validate(version),
            Command::GetClocks(x) => x.validate(version),
            Command::GetLibCells(x) => x.validate(version),
            Command::GetLibPins(x) => x.validate(version),
            Command::GetLibs(x) => x.validate(version),
            Command::GetNets(x) => x.validate(version),
            Command::GetPins(x) => x.validate(version),
            Command::GetPorts(x) => x.validate(version),
            Command::GroupPath(x) => x.validate(version),
            Command::List(x) => x.validate(version),
            Command::Set(x) => x.validate(version),
            Command::SetCaseAnalysis(x) => x.validate(version),
            Command::SetClockGatingCheck(x) => x.validate(version),
            Command::SetClockGroups(x) => x.validate(version),
            Command::SetClockLatency(x) => x.validate(version),
            Command::SetClockSense(x) => x.validate(version),
            Command::SetClockTransition(x) => x.validate(version),
            Command::SetClockUncertainty(x) => x.validate(version),
            Command::SetDataCheck(x) => x.validate(version),
            Command::SetDisableTiming(x) => x.validate(version),
            Command::SetDrive(x) => x.validate(version),
            Command::SetDrivingCell(x) => x.validate(version),
            Command::SetFalsePath(x) => x.validate(version),
            Command::SetFanoutLoad(x) => x.validate(version),
            Command::SetHierarchySeparator(x) => x.validate(version),
            Command::SetIdealLatency(x) => x.validate(version),
            Command::SetIdealNetwork(x) => x.validate(version),
            Command::SetIdealTransition(x) => x.validate(version),
            Command::SetInputDelay(x) => x.validate(version),
            Command::SetInputTransition(x) => x.validate(version),
            Command::SetLevelShifterStrategy(x) => x.validate(version),
            Command::SetLevelShifterThreshold(x) => x.validate(version),
            Command::SetLoad(x) => x.validate(version),
            Command::SetLogicDc(x) => x.validate(version),
            Command::SetLogicOne(x) => x.validate(version),
            Command::SetLogicZero(x) => x.validate(version),
            Command::SetMaxArea(x) => x.validate(version),
            Command::SetMaxCapacitance(x) => x.validate(version),
            Command::SetMaxDelay(x) => x.validate(version),
            Command::SetMaxDynamicPower(x) => x.validate(version),
            Command::SetMaxFanout(x) => x.validate(version),
            Command::SetMaxLeakagePower(x) => x.validate(version),
            Command::SetMaxTimeBorrow(x) => x.validate(version),
            Command::SetMaxTransition(x) => x.validate(version),
            Command::SetMinCapacitance(x) => x.validate(version),
            Command::SetMinDelay(x) => x.validate(version),
            Command::SetMinPorosity(x) => x.validate(version),
            Command::SetMinPulseWidth(x) => x.validate(version),
            Command::SetMulticyclePath(x) => x.validate(version),
            Command::SetOperatingConditions(x) => x.validate(version),
            Command::SetOutputDelay(x) => x.validate(version),
            Command::SetPortFanoutNumber(x) => x.validate(version),
            Command::SetPropagatedClock(x) => x.validate(version),
            Command::SetResistance(x) => x.validate(version),
            Command::SetSense(x) => x.validate(version),
            Command::SetTimingDerate(x) => x.validate(version),
            Command::SetUnits(x) => x.validate(version),
            Command::SetVoltage(x) => x.validate(version),
            Command::SetWireLoadMinBlockSize(x) => x.validate(version),
            Command::SetWireLoadMode(x) => x.validate(version),
            Command::SetWireLoadModel(x) => x.validate(version),
            Command::SetWireLoadSelectionGroup(x) => x.validate(version),
            Command::Unknown(x) => x.validate(version),
        }
    }

    fn location(&self) -> &Location {
        match self {
            Command::AllClocks(x) => x.location(),
            Command::AllInputs(x) => x.location(),
            Command::AllOutputs(x) => x.location(),
            Command::AllRegisters(x) => x.location(),
            Command::CreateClock(x) => x.location(),
            Command::CreateGeneratedClock(x) => x.location(),
            Command::CreateVoltageArea(x) => x.location(),
            Command::CurrentDesign(x) => x.location(),
            Command::CurrentInstance(x) => x.location(),
            Command::Expr(x) => x.location(),
            Command::GetCells(x) => x.location(),
            Command::GetClocks(x) => x.location(),
            Command::GetLibCells(x) => x.location(),
            Command::GetLibPins(x) => x.location(),
            Command::GetLibs(x) => x.location(),
            Command::GetNets(x) => x.location(),
            Command::GetPins(x) => x.location(),
            Command::GetPorts(x) => x.location(),
            Command::GroupPath(x) => x.location(),
            Command::List(x) => x.location(),
            Command::Set(x) => x.location(),
            Command::SetCaseAnalysis(x) => x.location(),
            Command::SetClockGatingCheck(x) => x.location(),
            Command::SetClockGroups(x) => x.location(),
            Command::SetClockLatency(x) => x.location(),
            Command::SetClockSense(x) => x.location(),
            Command::SetClockTransition(x) => x.location(),
            Command::SetClockUncertainty(x) => x.location(),
            Command::SetDataCheck(x) => x.location(),
            Command::SetDisableTiming(x) => x.location(),
            Command::SetDrive(x) => x.location(),
            Command::SetDrivingCell(x) => x.location(),
            Command::SetFalsePath(x) => x.location(),
            Command::SetFanoutLoad(x) => x.location(),
            Command::SetHierarchySeparator(x) => x.location(),
            Command::SetIdealLatency(x) => x.location(),
            Command::SetIdealNetwork(x) => x.location(),
            Command::SetIdealTransition(x) => x.location(),
            Command::SetInputDelay(x) => x.location(),
            Command::SetInputTransition(x) => x.location(),
            Command::SetLevelShifterStrategy(x) => x.location(),
            Command::SetLevelShifterThreshold(x) => x.location(),
            Command::SetLoad(x) => x.location(),
            Command::SetLogicDc(x) => x.location(),
            Command::SetLogicOne(x) => x.location(),
            Command::SetLogicZero(x) => x.location(),
            Command::SetMaxArea(x) => x.location(),
            Command::SetMaxCapacitance(x) => x.location(),
            Command::SetMaxDelay(x) => x.location(),
            Command::SetMaxDynamicPower(x) => x.location(),
            Command::SetMaxFanout(x) => x.location(),
            Command::SetMaxLeakagePower(x) => x.location(),
            Command::SetMaxTimeBorrow(x) => x.location(),
            Command::SetMaxTransition(x) => x.location(),
            Command::SetMinCapacitance(x) => x.location(),
            Command::SetMinDelay(x) => x.location(),
            Command::SetMinPorosity(x) => x.location(),
            Command::SetMinPulseWidth(x) => x.location(),
            Command::SetMulticyclePath(x) => x.location(),
            Command::SetOperatingConditions(x) => x.location(),
            Command::SetOutputDelay(x) => x.location(),
            Command::SetPortFanoutNumber(x) => x.location(),
            Command::SetPropagatedClock(x) => x.location(),
            Command::SetResistance(x) => x.location(),
            Command::SetSense(x) => x.location(),
            Command::SetTimingDerate(x) => x.location(),
            Command::SetUnits(x) => x.location(),
            Command::SetVoltage(x) => x.location(),
            Command::SetWireLoadMinBlockSize(x) => x.location(),
            Command::SetWireLoadMode(x) => x.location(),
            Command::SetWireLoadModel(x) => x.location(),
            Command::SetWireLoadSelectionGroup(x) => x.location(),
            Command::Unknown(x) => x.location(),
        }
    }
}

impl TryFrom<&grammar::Command<'_>> for Command {
    type Error = SdcError;

    fn try_from(value: &grammar::Command<'_>) -> Result<Self, SdcError> {
        let command = value.token_word.term_word.term_word.text();
        let start: Location = (&value.token_word.term_word.term_word.location).into();

        let mut args: Vec<Argument> = vec![];
        for arg in &value.command_list {
            args.push(arg.argument.as_ref().try_into()?);
        }

        let loc = if args.is_empty() {
            start
        } else {
            Location::from_to(&start, args.last().unwrap().location())
        };

        match command {
            "all_clocks" => all_clocks(args, loc),
            "all_inputs" => all_inputs(args, loc),
            "all_outputs" => all_outputs(args, loc),
            "all_registers" => all_registers(args, loc),
            "create_clock" => create_clock(args, loc),
            "create_generated_clock" => create_generated_clock(args, loc),
            "create_voltage_area" => create_voltage_area(args, loc),
            "current_design" => current_design(args, loc),
            "current_instance" => current_instance(args, loc),
            "expr" => expr(args, loc),
            "get_cell" => get_cells(args, loc, true),
            "get_cells" => get_cells(args, loc, false),
            "get_clocks" => get_clocks(args, loc),
            "get_lib_cell" => get_lib_cells(args, loc, true),
            "get_lib_cells" => get_lib_cells(args, loc, false),
            "get_lib_pin" => get_lib_pins(args, loc, true),
            "get_lib_pins" => get_lib_pins(args, loc, false),
            "get_libs" => get_libs(args, loc),
            "get_net" => get_nets(args, loc, true),
            "get_nets" => get_nets(args, loc, false),
            "get_pin" => get_pins(args, loc, true),
            "get_pins" => get_pins(args, loc, false),
            "get_port" => get_ports(args, loc, true),
            "get_ports" => get_ports(args, loc, false),
            "group_path" => group_path(args, loc),
            "list" => list(args, loc),
            "set" => set(args, loc),
            "set_case_analysis" => set_case_analysis(args, loc),
            "set_clock_gating_check" => set_clock_gating_check(args, loc),
            "set_clock_group" => set_clock_groups(args, loc, true),
            "set_clock_groups" => set_clock_groups(args, loc, false),
            "set_clock_latency" => set_clock_latency(args, loc),
            "set_clock_sense" => set_clock_sense(args, loc),
            "set_clock_transition" => set_clock_transition(args, loc),
            "set_clock_uncertainty" => set_clock_uncertainty(args, loc),
            "set_data_check" => set_data_check(args, loc),
            "set_disable_timing" => set_disable_timing(args, loc),
            "set_drive" => set_drive(args, loc),
            "set_driving_cell" => set_driving_cell(args, loc),
            "set_false_path" => set_false_path(args, loc),
            "set_fanout_load" => set_fanout_load(args, loc),
            "set_hierarchy_separator" => set_hierarchy_separator(args, loc),
            "set_ideal_latency" => set_ideal_latency(args, loc),
            "set_ideal_network" => set_ideal_network(args, loc),
            "set_ideal_transition" => set_ideal_transition(args, loc),
            "set_input_delay" => set_input_delay(args, loc),
            "set_input_transition" => set_input_transition(args, loc),
            "set_level_shifter_strategy" => set_level_shifter_strategy(args, loc),
            "set_level_shifter_threshold" => set_level_shifter_threshold(args, loc),
            "set_load" => set_load(args, loc),
            "set_logic_dc" => set_logic_dc(args, loc),
            "set_logic_one" => set_logic_one(args, loc),
            "set_logic_zero" => set_logic_zero(args, loc),
            "set_max_area" => set_max_area(args, loc),
            "set_max_capacitance" => set_max_capacitance(args, loc),
            "set_max_delay" => set_max_delay(args, loc),
            "set_max_dynamic_power" => set_max_dynamic_power(args, loc),
            "set_max_fanout" => set_max_fanout(args, loc),
            "set_max_leakage_power" => set_max_leakage_power(args, loc),
            "set_max_time_borrow" => set_max_time_borrow(args, loc),
            "set_max_transition" => set_max_transition(args, loc),
            "set_min_capacitance" => set_min_capacitance(args, loc),
            "set_min_delay" => set_min_delay(args, loc),
            "set_min_porosity" => set_min_porosity(args, loc),
            "set_min_pulse_width" => set_min_pulse_width(args, loc),
            "set_multicycle_path" => set_multicycle_path(args, loc),
            "set_operating_conditions" => set_operating_conditions(args, loc),
            "set_output_delay" => set_output_delay(args, loc),
            "set_port_fanout_number" => set_port_fanout_number(args, loc),
            "set_propagated_clock" => set_propagated_clock(args, loc),
            "set_resistance" => set_resistance(args, loc),
            "set_sense" => set_sense(args, loc),
            "set_timing_derate" => set_timing_derate(args, loc),
            "set_unit" => set_units(args, loc, true),
            "set_units" => set_units(args, loc, false),
            "set_voltage" => set_voltage(args, loc),
            "set_wire_load_min_block_size" => set_wire_load_min_block_size(args, loc),
            "set_wire_load_mode" => set_wire_load_mode(args, loc),
            "set_wire_load_model" => set_wire_load_model(args, loc),
            "set_wire_load_selection_group" => set_wire_load_selection_group(args, loc),
            x => unknown(x, args, loc),
        }
    }
}

/// all_clocks
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllClocks {
    location: Location,
}

impl fmt::Display for AllClocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = "all_clocks".to_string();
        text.fmt(f)
    }
}

impl Validate for AllClocks {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn all_clocks(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    if !args.is_empty() {
        return Err(SdcError::WrongArgument(args[0].clone()));
    }

    Ok(Command::AllClocks(AllClocks { location }))
}

/// all_inputs
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllInputs {
    pub level_sensitive: bool,
    pub edge_triggered: bool,
    pub clock: Option<Argument>,
    location: Location,
}

impl fmt::Display for AllInputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "all_inputs".to_string();
        text.push_str(&fmt_named_flg(self.level_sensitive, "level_sensitive"));
        text.push_str(&fmt_named_flg(self.edge_triggered, "edge_triggered"));
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.fmt(f)
    }
}

impl Validate for AllInputs {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_comb2(
            version.within(SDC1_1, SDC2_1),
            &self.level_sensitive,
            &self.edge_triggered,
            |a, b| !(a & b),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn all_inputs(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut level_sensitive = false;
    let mut edge_triggered = false;
    let mut clock = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict =
        DICT.get_or_init(|| LazyDict::new(&["-level_sensitive", "-edge_triggered", "-clock"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-level_sensitive") => level_sensitive = opt_flg(arg, level_sensitive)?,
            x if x.m("-edge_triggered") => edge_triggered = opt_flg(arg, edge_triggered)?,
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            _ => return Err(SdcError::WrongArgument(arg)),
        }
    }

    Ok(Command::AllInputs(AllInputs {
        level_sensitive,
        edge_triggered,
        clock,
        location,
    }))
}

/// all_outputs
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllOutputs {
    pub level_sensitive: bool,
    pub edge_triggered: bool,
    pub clock: Option<Argument>,
    location: Location,
}

impl fmt::Display for AllOutputs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "all_outputs".to_string();
        text.push_str(&fmt_named_flg(self.level_sensitive, "level_sensitive"));
        text.push_str(&fmt_named_flg(self.edge_triggered, "edge_triggered"));
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.fmt(f)
    }
}

impl Validate for AllOutputs {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_comb2(
            version.within(SDC1_2, SDC2_1),
            &self.level_sensitive,
            &self.edge_triggered,
            |a, b| !(a & b),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn all_outputs(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut level_sensitive = false;
    let mut edge_triggered = false;
    let mut clock = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict =
        DICT.get_or_init(|| LazyDict::new(&["-level_sensitive", "-edge_triggered", "-clock"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-level_sensitive") => level_sensitive = opt_flg(arg, level_sensitive)?,
            x if x.m("-edge_triggered") => edge_triggered = opt_flg(arg, edge_triggered)?,
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            _ => return Err(SdcError::WrongArgument(arg)),
        }
    }

    Ok(Command::AllOutputs(AllOutputs {
        level_sensitive,
        edge_triggered,
        clock,
        location,
    }))
}

/// all_registers
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AllRegisters {
    pub no_hierarchy: bool,
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
    location: Location,
}

impl fmt::Display for AllRegisters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "all_registers".to_string();
        text.push_str(&fmt_named_flg(self.no_hierarchy, "no_hierarchy"));
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.push_str(&fmt_named_opt_arg(&self.rise_clock, "rise_clock"));
        text.push_str(&fmt_named_opt_arg(&self.fall_clock, "fall_clock"));
        text.push_str(&fmt_named_flg(self.cells, "cells"));
        text.push_str(&fmt_named_flg(self.data_pins, "data_pins"));
        text.push_str(&fmt_named_flg(self.clock_pins, "clock_pins"));
        text.push_str(&fmt_named_flg(self.slave_clock_pins, "slave_clock_pins"));
        text.push_str(&fmt_named_flg(self.output_pins, "output_pins"));
        text.push_str(&fmt_named_flg(self.level_sensitive, "level_sensitive"));
        text.push_str(&fmt_named_flg(self.edge_triggered, "edge_triggered"));
        text.push_str(&fmt_named_flg(self.master_slave, "master_slave"));
        text.fmt(f)
    }
}

impl Validate for AllRegisters {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn all_registers(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut no_hierarchy = false;
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-no_hierarchy",
            "-clock",
            "-rise_clock",
            "-fall_clock",
            "-cells",
            "-data_pins",
            "-clock_pins",
            "-slave_clock_pins",
            "-async_pins",
            "-output_pins",
            "-level_sensitive",
            "-edge_triggered",
            "-master_slave",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-no_hierarchy") => no_hierarchy = opt_flg(arg, no_hierarchy)?,
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            x if x.m("-rise_clock") => rise_clock = opt_arg(arg, iter.next(), rise_clock)?,
            x if x.m("-fall_clock") => fall_clock = opt_arg(arg, iter.next(), fall_clock)?,
            x if x.m("-cells") => cells = opt_flg(arg, cells)?,
            x if x.m("-data_pins") => data_pins = opt_flg(arg, data_pins)?,
            x if x.m("-clock_pins") => clock_pins = opt_flg(arg, clock_pins)?,
            x if x.m("-slave_clock_pins") => slave_clock_pins = opt_flg(arg, slave_clock_pins)?,
            x if x.m("-async_pins") => async_pins = opt_flg(arg, async_pins)?,
            x if x.m("-output_pins") => output_pins = opt_flg(arg, output_pins)?,
            x if x.m("-level_sensitive") => level_sensitive = opt_flg(arg, level_sensitive)?,
            x if x.m("-edge_triggered") => edge_triggered = opt_flg(arg, edge_triggered)?,
            x if x.m("-master_slave") => master_slave = opt_flg(arg, master_slave)?,
            _ => return Err(SdcError::WrongArgument(arg)),
        }
    }

    Ok(Command::AllRegisters(AllRegisters {
        no_hierarchy,
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
        location,
    }))
}

/// create_clock
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateClock {
    pub period: Argument,
    pub name: Option<Argument>,
    pub waveform: Option<Argument>,
    pub add: bool,
    pub comment: Option<Argument>,
    pub source_objects: Option<Argument>,
    location: Location,
}

impl fmt::Display for CreateClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "create_clock".to_string();
        text.push_str(&fmt_named_arg(&self.period, "period"));
        text.push_str(&fmt_named_opt_arg(&self.name, "name"));
        text.push_str(&fmt_named_opt_arg(&self.waveform, "waveform"));
        text.push_str(&fmt_named_flg(self.add, "add"));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.push_str(&fmt_opt_arg(&self.source_objects));
        text.fmt(f)
    }
}

impl Validate for CreateClock {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_4, SDC2_1), &self.add, "add")?;
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")?;
        self.arg_comb2(
            version.within(SDC1_2, SDC2_1),
            &self.name,
            &self.source_objects,
            |a, b| a | b,
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn create_clock(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut period = None;
    let mut name = None;
    let mut waveform = None;
    let mut add = false;
    let mut comment = None;
    let mut source_objects = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict =
        DICT.get_or_init(|| LazyDict::new(&["-period", "-name", "-waveform", "-add", "-comment"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-period") => period = opt_arg(arg, iter.next(), period)?,
            x if x.m("-name") => name = opt_arg(arg, iter.next(), name)?,
            x if x.m("-waveform") => waveform = opt_arg(arg, iter.next(), waveform)?,
            x if x.m("-add") => add = opt_flg(arg, add)?,
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => source_objects = pos_args1(Some(arg), source_objects, &location)?,
        }
    }

    let period = mandatory(period, "-period", &location)?;

    Ok(Command::CreateClock(CreateClock {
        period,
        name,
        waveform,
        add,
        comment,
        source_objects,
        location,
    }))
}

/// create_generated_clock
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for CreateGeneratedClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "create_generated_clock".to_string();
        text.push_str(&fmt_named_opt_arg(&self.name, "name"));
        text.push_str(&fmt_named_arg(&self.source, "source"));
        text.push_str(&fmt_named_opt_arg(&self.edges, "edges"));
        text.push_str(&fmt_named_opt_arg(&self.divide_by, "divide_by"));
        text.push_str(&fmt_named_opt_arg(&self.multiply_by, "multiply_by"));
        text.push_str(&fmt_named_opt_arg(&self.duty_cycle, "duty_cycle"));
        text.push_str(&fmt_named_flg(self.invert, "invert"));
        text.push_str(&fmt_named_opt_arg(&self.edge_shift, "edge_shift"));
        text.push_str(&fmt_named_flg(self.add, "add"));
        text.push_str(&fmt_named_opt_arg(&self.master_clock, "master_clock"));
        text.push_str(&fmt_named_flg(self.combinational, "combinational"));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.push_str(&fmt_arg(&self.source_objects));
        text.fmt(f)
    }
}

impl Validate for CreateGeneratedClock {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_3, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_4, SDC2_1), &self.add, "add")?;
        self.arg_supported_version(
            version.within(SDC1_4, SDC2_1),
            &self.master_clock,
            "master_clock",
        )?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_0),
            &self.combinational,
            "combinational",
        )?;
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")?;
        self.arg_comb2(
            version.within(SDC1_3, SDC2_1),
            &self.multiply_by,
            &self.divide_by,
            |a, b| !(a & b),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn create_generated_clock(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-name",
            "-source",
            "-edges",
            "-divide_by",
            "-multiply_by",
            "-duty_cycle",
            "-invert",
            "-edge_shift",
            "-add",
            "-master_clock",
            "-combinational",
            "-comment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-name") => name = opt_arg(arg, iter.next(), name)?,
            x if x.m("-source") => source = opt_arg(arg, iter.next(), source)?,
            x if x.m("-edges") => edges = opt_arg(arg, iter.next(), edges)?,
            x if x.m("-divide_by") => divide_by = opt_arg(arg, iter.next(), divide_by)?,
            x if x.m("-multiply_by") => multiply_by = opt_arg(arg, iter.next(), multiply_by)?,
            x if x.m("-duty_cycle") => duty_cycle = opt_arg(arg, iter.next(), duty_cycle)?,
            x if x.m("-invert") => invert = opt_flg(arg, invert)?,
            x if x.m("-edge_shift") => edge_shift = opt_arg(arg, iter.next(), edge_shift)?,
            x if x.m("-add") => add = opt_flg(arg, add)?,
            x if x.m("-master_clock") => master_clock = opt_arg(arg, iter.next(), master_clock)?,
            x if x.m("-combinational") => combinational = opt_flg(arg, combinational)?,
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => source_objects = pos_args1(Some(arg), source_objects, &location)?,
        }
    }

    let source = mandatory(source, "-source", &location)?;
    let source_objects = mandatory(source_objects, "source_objects", &location)?;

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
        location,
    }))
}

/// create_voltage_area
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CreateVoltageArea {
    pub name: Argument,
    pub coordinate: Option<Argument>,
    pub guard_band_x: Option<Argument>,
    pub guard_band_y: Option<Argument>,
    pub cell_list: Argument,
    location: Location,
}

impl fmt::Display for CreateVoltageArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "create_voltage_area".to_string();
        text.push_str(&fmt_named_arg(&self.name, "name"));
        text.push_str(&fmt_named_opt_arg(&self.coordinate, "coordinate"));
        text.push_str(&fmt_named_opt_arg(&self.guard_band_x, "guard_band_x"));
        text.push_str(&fmt_named_opt_arg(&self.guard_band_y, "guard_band_y"));
        text.push_str(&fmt_arg(&self.cell_list));
        text.fmt(f)
    }
}

impl Validate for CreateVoltageArea {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_6, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn create_voltage_area(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut name = None;
    let mut coordinate = None;
    let mut guard_band_x = None;
    let mut guard_band_y = None;
    let mut cell_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT
        .get_or_init(|| LazyDict::new(&["-name", "-coordinate", "-guard_band_x", "-guard_band_y"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-name") => name = opt_arg(arg, iter.next(), name)?,
            x if x.m("-coordinate") => coordinate = opt_arg(arg, iter.next(), coordinate)?,
            x if x.m("-guard_band_x") => guard_band_x = opt_arg(arg, iter.next(), guard_band_x)?,
            x if x.m("-guard_band_y") => guard_band_y = opt_arg(arg, iter.next(), guard_band_y)?,
            _ => cell_list = pos_args1(Some(arg), cell_list, &location)?,
        }
    }

    let name = mandatory(name, "name", &location)?;
    let cell_list = mandatory(cell_list, "cell_list", &location)?;

    Ok(Command::CreateVoltageArea(CreateVoltageArea {
        name,
        coordinate,
        guard_band_x,
        guard_band_y,
        cell_list,
        location,
    }))
}

/// current_design
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentDesign {
    location: Location,
}

impl fmt::Display for CurrentDesign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = "current_design".to_string();
        text.fmt(f)
    }
}

impl Validate for CurrentDesign {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn current_design(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    if !args.is_empty() {
        return Err(SdcError::WrongArgument(args[0].clone()));
    }

    Ok(Command::CurrentDesign(CurrentDesign { location }))
}

/// current_instance
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CurrentInstance {
    pub instance: Option<Argument>,
    location: Location,
}

impl fmt::Display for CurrentInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "current_instance".to_string();
        text.push_str(&fmt_opt_arg(&self.instance));
        text.fmt(f)
    }
}

impl Validate for CurrentInstance {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn current_instance(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut instance = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        instance = pos_args1(Some(arg), instance, &location)?;
    }

    Ok(Command::CurrentInstance(CurrentInstance {
        instance,
        location,
    }))
}

/// expr
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expr {
    pub args: Vec<Argument>,
    location: Location,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "expr".to_string();
        for arg in &self.args {
            text.push_str(&fmt_arg(arg));
        }
        text.fmt(f)
    }
}

impl Validate for Expr {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn expr(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut ret = vec![];

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        ret.push(arg);
    }

    Ok(Command::Expr(Expr {
        args: ret,
        location,
    }))
}

/// get_cells
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetCells {
    pub hierarchical: bool,
    pub regexp: bool,
    pub nocase: bool,
    pub hsc: Option<Argument>,
    pub of_objects: Option<Argument>,
    pub patterns: Option<Argument>,
    location: Location,
    alias: bool,
}

impl fmt::Display for GetCells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_cells".to_string();
        text.push_str(&fmt_named_flg(self.hierarchical, "hierarchical"));
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_named_flg(self.nocase, "nocase"));
        text.push_str(&fmt_named_opt_arg(&self.hsc, "hsc"));
        text.push_str(&fmt_named_opt_arg(&self.of_objects, "of_objects"));
        text.push_str(&fmt_opt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetCells {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_5, SDC2_1), self.alias)?;
        self.arg_supported_version(version.within(SDC1_2, SDC2_1), &self.hsc, "hsc")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.nocase, "nocase")?;
        self.arg_comb3(
            version.within(SDC1_1, SDC1_1),
            &self.patterns,
            &self.of_objects,
            &self.hierarchical,
            |a, b, c| (a & !b) | (b & !a & !c),
        )?;
        self.arg_comb2(
            version.within(SDC1_2, SDC1_4),
            &self.patterns,
            &self.of_objects,
            |a, b| (a & !b) | (b & !a),
        )?;
        self.arg_comb2(
            version.within(SDC1_5, SDC2_1),
            &self.patterns,
            &self.of_objects,
            |a, b| (a & !b) | (b & !a) | !a,
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_cells(args: Vec<Argument>, location: Location, alias: bool) -> Result<Command, SdcError> {
    let mut hierarchical = false;
    let mut regexp = false;
    let mut nocase = false;
    let mut hsc = None;
    let mut of_objects = None;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&["-hierarchical", "-regexp", "-nocase", "-hsc", "-of_objects"])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-hierarchical") => hierarchical = opt_flg(arg, hierarchical)?,
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            x if x.m("-nocase") => nocase = opt_flg(arg, nocase)?,
            x if x.m("-hsc") => hsc = opt_arg(arg, iter.next(), hsc)?,
            x if x.m("-of_objects") => of_objects = opt_arg(arg, iter.next(), of_objects)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    Ok(Command::GetCells(GetCells {
        hierarchical,
        regexp,
        nocase,
        hsc,
        of_objects,
        patterns,
        location,
        alias,
    }))
}

/// get_clocks
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetClocks {
    pub regexp: bool,
    pub nocase: bool,
    pub patterns: Option<Argument>,
    location: Location,
}

impl fmt::Display for GetClocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_clocks".to_string();
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_named_flg(self.nocase, "nocase"));
        text.push_str(&fmt_opt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetClocks {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.nocase, "nocase")?;
        self.arg_comb1(version.within(SDC1_1, SDC1_4), &self.patterns, |a| a)
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_clocks(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut nocase = false;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-regexp", "-nocase"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            x if x.m("-nocase") => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    Ok(Command::GetClocks(GetClocks {
        regexp,
        nocase,
        patterns,
        location,
    }))
}

/// get_lib_cells
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetLibCells {
    pub regexp: bool,
    pub hsc: Option<Argument>,
    pub nocase: bool,
    pub patterns: Argument,
    location: Location,
    alias: bool,
}

impl fmt::Display for GetLibCells {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_lib_cells".to_string();
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_named_opt_arg(&self.hsc, "hsc"));
        text.push_str(&fmt_named_flg(self.nocase, "nocase"));
        text.push_str(&fmt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetLibCells {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_5, SDC2_1), self.alias)?;
        self.arg_supported_version(version.within(SDC1_2, SDC2_1), &self.hsc, "hsc")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.nocase, "nocase")
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_lib_cells(
    args: Vec<Argument>,
    location: Location,
    alias: bool,
) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut hsc = None;
    let mut nocase = false;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-regexp", "-hsc", "-nocase"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            x if x.m("-hsc") => hsc = opt_arg(arg, iter.next(), hsc)?,
            x if x.m("-nocase") => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    let patterns = mandatory(patterns, "patterns", &location)?;

    Ok(Command::GetLibCells(GetLibCells {
        regexp,
        hsc,
        nocase,
        patterns,
        location,
        alias,
    }))
}

/// get_lib_pins
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetLibPins {
    pub regexp: bool,
    pub hsc: bool,
    pub nocase: bool,
    pub patterns: Argument,
    location: Location,
    alias: bool,
}

impl fmt::Display for GetLibPins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_lib_pins".to_string();
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_named_flg(self.hsc, "hsc"));
        text.push_str(&fmt_named_flg(self.nocase, "nocase"));
        text.push_str(&fmt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetLibPins {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_5, SDC2_1), self.alias)?;
        self.arg_supported_version(version.within(SDC1_2, SDC2_1), &self.hsc, "hsc")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.nocase, "nocase")
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_lib_pins(args: Vec<Argument>, location: Location, alias: bool) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut hsc = false;
    let mut nocase = false;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-regexp", "-hsc", "-nocase"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            x if x.m("-hsc") => hsc = opt_flg(arg, hsc)?,
            x if x.m("-nocase") => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    let patterns = mandatory(patterns, "patterns", &location)?;

    Ok(Command::GetLibPins(GetLibPins {
        regexp,
        hsc,
        nocase,
        patterns,
        location,
        alias,
    }))
}

/// get_libs
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetLibs {
    pub regexp: bool,
    pub nocase: bool,
    pub patterns: Option<Argument>,
    location: Location,
}

impl fmt::Display for GetLibs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_libs".to_string();
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_named_flg(self.nocase, "nocase"));
        text.push_str(&fmt_opt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetLibs {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.nocase, "nocase")?;
        self.arg_comb1(version.within(SDC1_1, SDC1_4), &self.patterns, |a| a)
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_libs(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut regexp = false;
    let mut nocase = false;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-regexp", "-nocase"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            x if x.m("-nocase") => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    Ok(Command::GetLibs(GetLibs {
        regexp,
        nocase,
        patterns,
        location,
    }))
}

/// get_nets
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetNets {
    pub hierarchical: bool,
    pub hsc: Option<Argument>,
    pub regexp: bool,
    pub nocase: bool,
    pub of_objects: Option<Argument>,
    pub patterns: Option<Argument>,
    location: Location,
    alias: bool,
}

impl fmt::Display for GetNets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_nets".to_string();
        text.push_str(&fmt_named_flg(self.hierarchical, "hierarchical"));
        text.push_str(&fmt_named_opt_arg(&self.hsc, "hsc"));
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_named_flg(self.nocase, "nocase"));
        text.push_str(&fmt_named_opt_arg(&self.of_objects, "of_objects"));
        text.push_str(&fmt_opt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetNets {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_5, SDC2_1), self.alias)?;
        self.arg_supported_version(version.within(SDC1_2, SDC2_1), &self.hsc, "hsc")?;
        self.arg_supported_version(
            version.within(SDC1_5, SDC2_1),
            &self.of_objects,
            "of_objects",
        )?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.nocase, "nocase")?;
        self.arg_comb1(version.within(SDC1_1, SDC1_4), &self.patterns, |a| a)?;
        self.arg_comb2(
            version.within(SDC1_5, SDC2_1),
            &self.patterns,
            &self.of_objects,
            |a, b| (a & !b) | (b & !a) | !a,
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_nets(args: Vec<Argument>, location: Location, alias: bool) -> Result<Command, SdcError> {
    let mut hierarchical = false;
    let mut hsc = None;
    let mut regexp = false;
    let mut nocase = false;
    let mut of_objects = None;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&["-hierarchical", "-hsc", "-regexp", "-nocase", "-of_objects"])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-hierarchical") => hierarchical = opt_flg(arg, hierarchical)?,
            x if x.m("-hsc") => hsc = opt_arg(arg, iter.next(), hsc)?,
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            x if x.m("-nocase") => nocase = opt_flg(arg, nocase)?,
            x if x.m("-of_objects") => of_objects = opt_arg(arg, iter.next(), of_objects)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    Ok(Command::GetNets(GetNets {
        hierarchical,
        hsc,
        regexp,
        nocase,
        of_objects,
        patterns,
        location,
        alias,
    }))
}

/// get_pins
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetPins {
    pub hierarchical: bool,
    pub hsc: Option<Argument>,
    pub regexp: bool,
    pub nocase: bool,
    pub patterns: Option<Argument>,
    location: Location,
    alias: bool,
}

impl fmt::Display for GetPins {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_pins".to_string();
        text.push_str(&fmt_named_flg(self.hierarchical, "hierarchical"));
        text.push_str(&fmt_named_opt_arg(&self.hsc, "hsc"));
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_named_flg(self.nocase, "nocase"));
        text.push_str(&fmt_opt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetPins {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_5, SDC2_1), self.alias)?;
        self.arg_supported_version(version.within(SDC1_2, SDC2_1), &self.hsc, "hsc")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.nocase, "nocase")?;
        self.arg_comb1(version.within(SDC1_1, SDC1_4), &self.patterns, |a| a)
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_pins(args: Vec<Argument>, location: Location, alias: bool) -> Result<Command, SdcError> {
    let mut hierarchical = false;
    let mut hsc = None;
    let mut regexp = false;
    let mut nocase = false;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-hierarchical", "-hsc", "-regexp", "-nocase"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-hierarchical") => hierarchical = opt_flg(arg, hierarchical)?,
            x if x.m("-hsc") => hsc = opt_arg(arg, iter.next(), hsc)?,
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            x if x.m("-nocase") => nocase = opt_flg(arg, nocase)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    Ok(Command::GetPins(GetPins {
        hierarchical,
        hsc,
        regexp,
        nocase,
        patterns,
        location,
        alias,
    }))
}

/// get_ports
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetPorts {
    pub hierarchical: bool,
    pub regexp: bool,
    pub patterns: Option<Argument>,
    location: Location,
    alias: bool,
}

impl fmt::Display for GetPorts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "get_ports".to_string();
        text.push_str(&fmt_named_flg(self.hierarchical, "hierarchical"));
        text.push_str(&fmt_named_flg(self.regexp, "regexp"));
        text.push_str(&fmt_opt_arg(&self.patterns));
        text.fmt(f)
    }
}

impl Validate for GetPorts {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_5, SDC2_1), self.alias)?;
        self.arg_supported_version(
            version.within(SDC1_5, SDC2_1),
            &self.hierarchical,
            "hierarchical",
        )?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.regexp, "regexp")?;
        self.arg_comb1(version.within(SDC1_1, SDC1_4), &self.patterns, |a| a)
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn get_ports(args: Vec<Argument>, location: Location, alias: bool) -> Result<Command, SdcError> {
    let mut hierarchical = false;
    let mut regexp = false;
    let mut patterns = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-hierarchical", "-regexp"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-hierarchical") => hierarchical = opt_flg(arg, hierarchical)?,
            x if x.m("-regexp") => regexp = opt_flg(arg, regexp)?,
            _ => patterns = pos_args1(Some(arg), patterns, &location)?,
        }
    }

    Ok(Command::GetPorts(GetPorts {
        hierarchical,
        regexp,
        patterns,
        location,
        alias,
    }))
}

/// group_path
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    pub through: Vec<Argument>,
    pub rise_through: Vec<Argument>,
    pub fall_through: Vec<Argument>,
    pub comment: Option<Argument>,
    location: Location,
}

impl fmt::Display for GroupPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "group_path".to_string();
        text.push_str(&fmt_named_opt_arg(&self.name, "name"));
        text.push_str(&fmt_named_flg(self.default, "default"));
        text.push_str(&fmt_named_opt_arg(&self.weight, "weight"));
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.rise_from, "rise_from"));
        text.push_str(&fmt_named_opt_arg(&self.fall_from, "fall_from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_named_opt_arg(&self.rise_to, "rise_to"));
        text.push_str(&fmt_named_opt_arg(&self.fall_to, "fall_to"));
        text.push_str(&fmt_named_vec_arg(&self.through, "through"));
        text.push_str(&fmt_named_vec_arg(&self.rise_through, "rise_through"));
        text.push_str(&fmt_named_vec_arg(&self.fall_through, "fall_through"));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.fmt(f)
    }
}

impl Validate for GroupPath {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")?;
        self.arg_comb5(
            version.within(SDC1_7, SDC2_1),
            &self.name,
            &self.default,
            &self.from,
            &self.rise_from,
            &self.fall_from,
            |a, b, c, d, e| (a & !b) | (b & !a) | !a & (c ^ d ^ e),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn group_path(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut name = None;
    let mut default = false;
    let mut weight = None;
    let mut from = None;
    let mut rise_from = None;
    let mut fall_from = None;
    let mut to = None;
    let mut rise_to = None;
    let mut fall_to = None;
    let mut through = vec![];
    let mut rise_through = vec![];
    let mut fall_through = vec![];
    let mut comment = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-name",
            "-default",
            "-weight",
            "-from",
            "-rise_from",
            "-fall_from",
            "-to",
            "-rise_to",
            "-fall_to",
            "-through",
            "-rise_through",
            "-fall_through",
            "-comment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-name") => name = opt_arg(arg, iter.next(), name)?,
            x if x.m("-default") => default = opt_flg(arg, default)?,
            x if x.m("-weight") => weight = opt_arg(arg, iter.next(), weight)?,
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-rise_from") => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            x if x.m("-fall_from") => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            x if x.m("-rise_to") => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            x if x.m("-fall_to") => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            x if x.m("-through") => through = vec_arg(arg, iter.next(), through)?,
            x if x.m("-rise_through") => rise_through = vec_arg(arg, iter.next(), rise_through)?,
            x if x.m("-fall_through") => fall_through = vec_arg(arg, iter.next(), fall_through)?,
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => return Err(SdcError::WrongArgument(arg)),
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
        location,
    }))
}

/// list
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct List {
    pub args: Vec<Argument>,
    location: Location,
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "list".to_string();
        for arg in &self.args {
            text.push_str(&fmt_arg(arg));
        }
        text.fmt(f)
    }
}

impl Validate for List {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn list(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut ret = vec![];

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        ret.push(arg);
    }

    Ok(Command::List(List {
        args: ret,
        location,
    }))
}

/// set
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set {
    pub variable_name: Argument,
    pub value: Argument,
    location: Location,
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set".to_string();
        text.push_str(&fmt_arg(&self.variable_name));
        text.push_str(&fmt_arg(&self.value));
        text.fmt(f)
    }
}

impl Validate for Set {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut variable_name = None;
    let mut value = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (variable_name, value) = pos_args2(Some(arg), (variable_name, value), &location)?;
    }

    let variable_name = mandatory(variable_name, "variable_name", &location)?;
    let value = mandatory(value, "value", &location)?;

    Ok(Command::Set(Set {
        variable_name,
        value,
        location,
    }))
}

/// set_case_analysis
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetCaseAnalysis {
    pub value: Option<Argument>,
    pub port_or_pin_list: Argument,
    location: Location,
}

impl fmt::Display for SetCaseAnalysis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_case_analysis".to_string();
        text.push_str(&fmt_opt_arg(&self.value));
        text.push_str(&fmt_arg(&self.port_or_pin_list));
        text.fmt(f)
    }
}

impl Validate for SetCaseAnalysis {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_comb1(version.within(SDC1_2, SDC2_1), &self.value, |a| a)
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_case_analysis(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut value = None;
    let mut port_or_pin_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (value, port_or_pin_list) = pos_args2(Some(arg), (value, port_or_pin_list), &location)?
    }

    let port_or_pin_list = mandatory(port_or_pin_list, "port_or_pin_list", &location)?;

    Ok(Command::SetCaseAnalysis(SetCaseAnalysis {
        value,
        port_or_pin_list,
        location,
    }))
}

/// set_clock_gating_check
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetClockGatingCheck {
    pub setup: Option<Argument>,
    pub hold: Option<Argument>,
    pub rise: bool,
    pub fall: bool,
    pub high: bool,
    pub low: bool,
    pub object_list: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetClockGatingCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_clock_gating_check".to_string();
        text.push_str(&fmt_named_opt_arg(&self.setup, "setup"));
        text.push_str(&fmt_named_opt_arg(&self.hold, "hold"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.high, "high"));
        text.push_str(&fmt_named_flg(self.low, "low"));
        text.push_str(&fmt_opt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetClockGatingCheck {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_2, SDC2_1))?;
        self.arg_comb4(
            version.within(SDC1_2, SDC2_1),
            &self.setup,
            &self.hold,
            &self.high,
            &self.low,
            |a, b, c, d| (a | b | c | d) & !(c & d),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_clock_gating_check(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut setup = None;
    let mut hold = None;
    let mut rise = false;
    let mut fall = false;
    let mut high = false;
    let mut low = false;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict =
        DICT.get_or_init(|| LazyDict::new(&["-setup", "-hold", "-rise", "-fall", "-high", "-low"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-setup") => setup = opt_arg(arg, iter.next(), setup)?,
            x if x.m("-hold") => hold = opt_arg(arg, iter.next(), hold)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-high") => high = opt_flg(arg, high)?,
            x if x.m("-low") => low = opt_flg(arg, low)?,
            _ => object_list = pos_args1(Some(arg), object_list, &location)?,
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
        location,
    }))
}

/// set_clock_groups
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetClockGroups {
    pub group: Vec<Argument>,
    pub logically_exclusive: bool,
    pub physically_exclusive: bool,
    pub asynchronous: bool,
    pub allow_paths: bool,
    pub name: Option<Argument>,
    pub comment: Option<Argument>,
    location: Location,
    alias: bool,
}

impl fmt::Display for SetClockGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_clock_groups".to_string();
        text.push_str(&fmt_named_vec_arg(&self.group, "group"));
        text.push_str(&fmt_named_flg(
            self.logically_exclusive,
            "logically_exclusive",
        ));
        text.push_str(&fmt_named_flg(
            self.physically_exclusive,
            "physically_exclusive",
        ));
        text.push_str(&fmt_named_flg(self.asynchronous, "asynchronous"));
        text.push_str(&fmt_named_flg(self.allow_paths, "low"));
        text.push_str(&fmt_named_opt_arg(&self.name, "name"));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.fmt(f)
    }
}

impl Validate for SetClockGroups {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_7, SDC2_1), self.alias)?;
        // TODO group to dup at 1.8
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")?;
        self.arg_comb3(
            version.within(SDC1_7, SDC2_1),
            &self.physically_exclusive,
            &self.logically_exclusive,
            &self.asynchronous,
            |a, b, c| a ^ b ^ c,
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_clock_groups(
    args: Vec<Argument>,
    location: Location,
    alias: bool,
) -> Result<Command, SdcError> {
    let mut group = vec![];
    let mut logically_exclusive = false;
    let mut physically_exclusive = false;
    let mut asynchronous = false;
    let mut allow_paths = false;
    let mut name = None;
    let mut comment = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-group",
            "-logically_exclusive",
            "-physically_exclusive",
            "-asynchronous",
            "-allow_paths",
            "-name",
            "-comment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-group") => group = vec_arg(arg, iter.next(), group)?,
            x if x.m("-logically_exclusive") => {
                logically_exclusive = opt_flg(arg, logically_exclusive)?
            }
            x if x.m("-physically_exclusive") => {
                physically_exclusive = opt_flg(arg, physically_exclusive)?
            }
            x if x.m("-asynchronous") => asynchronous = opt_flg(arg, asynchronous)?,
            x if x.m("-allow_paths") => allow_paths = opt_flg(arg, allow_paths)?,
            x if x.m("-name") => name = opt_arg(arg, iter.next(), name)?,
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => return Err(SdcError::WrongArgument(arg)),
        }
    }

    Ok(Command::SetClockGroups(SetClockGroups {
        group,
        logically_exclusive,
        physically_exclusive,
        asynchronous,
        allow_paths,
        name,
        comment,
        location,
        alias,
    }))
}

/// set_clock_latency
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for SetClockLatency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_clock_latency".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_named_flg(self.source, "source"));
        text.push_str(&fmt_named_flg(self.dynamic, "dynamic"));
        text.push_str(&fmt_named_flg(self.late, "late"));
        text.push_str(&fmt_named_flg(self.early, "early"));
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.push_str(&fmt_arg(&self.delay));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetClockLatency {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_2, SDC2_1), &self.early, "early")?;
        self.arg_supported_version(version.within(SDC1_2, SDC2_1), &self.late, "late")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.clock, "clock")?;
        self.arg_supported_version(version.within(SDC2_1, SDC2_1), &self.dynamic, "dynamic")
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_clock_latency(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-rise", "-fall", "-min", "-max", "-source", "-dynamic", "-late", "-early", "-clock",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            x if x.m("-source") => source = opt_flg(arg, source)?,
            x if x.m("-dynamic") => dynamic = opt_flg(arg, dynamic)?,
            x if x.m("-late") => late = opt_flg(arg, late)?,
            x if x.m("-early") => early = opt_flg(arg, early)?,
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            _ => (delay, object_list) = pos_args2(Some(arg), (delay, object_list), &location)?,
        }
    }

    let delay = mandatory(delay, "delay", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

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
        location,
    }))
}

/// set_clock_sense
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetClockSense {
    pub clocks: Option<Argument>,
    pub positive: bool,
    pub negative: bool,
    pub stop_propagation: bool,
    pub pulse: Option<Argument>,
    pub pins: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetClockSense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_clock_sense".to_string();
        text.push_str(&fmt_named_opt_arg(&self.clocks, "clocks"));
        text.push_str(&fmt_named_flg(self.positive, "positive"));
        text.push_str(&fmt_named_flg(self.negative, "negative"));
        text.push_str(&fmt_named_flg(self.stop_propagation, "stop_propagation"));
        text.push_str(&fmt_named_opt_arg(&self.pulse, "pulse"));
        text.push_str(&fmt_opt_arg(&self.pins));
        text.fmt(f)
    }
}

impl Validate for SetClockSense {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_0))?;
        self.arg_comb4(
            version.within(SDC1_7, SDC2_0),
            &self.positive,
            &self.negative,
            &self.stop_propagation,
            &self.pulse,
            |a, b, c, d| a ^ b ^ c ^ d,
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_clock_sense(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut clocks = None;
    let mut positive = false;
    let mut negative = false;
    let mut stop_propagation = false;
    let mut pulse = None;
    let mut pins = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-clocks",
            "-positive",
            "-negative",
            "-stop_propagation",
            "-pulse",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-clocks") => clocks = opt_arg(arg, iter.next(), clocks)?,
            x if x.m("-positive") => positive = opt_flg(arg, positive)?,
            x if x.m("-negative") => negative = opt_flg(arg, negative)?,
            x if x.m("-stop_propagation") => stop_propagation = opt_flg(arg, stop_propagation)?,
            x if x.m("-pulse") => pulse = opt_arg(arg, iter.next(), pulse)?,
            _ => pins = pos_args1(Some(arg), pins, &location)?,
        }
    }

    Ok(Command::SetClockSense(SetClockSense {
        clocks,
        positive,
        negative,
        stop_propagation,
        pulse,
        pins,
        location,
    }))
}

/// set_clock_transition
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetClockTransition {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub transition: Argument,
    pub clock_list: Argument,
    location: Location,
}

impl fmt::Display for SetClockTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_clock_transition".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_arg(&self.transition));
        text.push_str(&fmt_arg(&self.clock_list));
        text.fmt(f)
    }
}

impl Validate for SetClockTransition {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_comb2(
            version.within(SDC1_2, SDC2_1),
            &self.rise,
            &self.fall,
            |a, b| !(a & b),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_clock_transition(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut transition = None;
    let mut clock_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-rise", "-fall", "-min", "-max"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            _ => {
                (transition, clock_list) =
                    pos_args2(Some(arg), (transition, clock_list), &location)?
            }
        }
    }

    let transition = mandatory(transition, "transition", &location)?;
    let clock_list = mandatory(clock_list, "clock_list", &location)?;

    Ok(Command::SetClockTransition(SetClockTransition {
        rise,
        fall,
        min,
        max,
        transition,
        clock_list,
        location,
    }))
}

/// set_clock_uncertainty
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for SetClockUncertainty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_clock_uncertainty".to_string();
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.rise_from, "rise_from"));
        text.push_str(&fmt_named_opt_arg(&self.fall_from, "fall_from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_named_opt_arg(&self.rise_to, "rise_to"));
        text.push_str(&fmt_named_opt_arg(&self.fall_to, "fall_to"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.setup, "setup"));
        text.push_str(&fmt_named_flg(self.hold, "hold"));
        text.push_str(&fmt_arg(&self.uncertainty));
        text.push_str(&fmt_opt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetClockUncertainty {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.rise_from, "rise_from")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.fall_from, "fall_from")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.rise_to, "rise_to")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.fall_to, "fall_to")?;
        self.arg_comb3(
            version.within(SDC1_1, SDC1_2),
            &self.object_list,
            &self.from,
            &self.to,
            |a, b, c| !(a & (b | c)),
        )?;
        self.arg_comb3(
            version.within(SDC1_3, SDC1_4),
            &self.object_list,
            &self.from,
            &self.to,
            |a, b, c| a ^ (b & c),
        )?;
        self.arg_comb7(
            version.within(SDC1_5, SDC2_1),
            &self.object_list,
            &self.from,
            &self.rise_from,
            &self.fall_from,
            &self.to,
            &self.rise_to,
            &self.fall_to,
            |a, b, c, d, e, f, g| a ^ ((b ^ c ^ d) & (e ^ f ^ g)),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_clock_uncertainty(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-from",
            "-rise_from",
            "-fall_from",
            "-to",
            "-rise_to",
            "-fall_to",
            "-rise",
            "-fall",
            "-setup",
            "-hold",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-rise_from") => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            x if x.m("-fall_from") => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            x if x.m("-rise_to") => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            x if x.m("-fall_to") => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-setup") => setup = opt_flg(arg, setup)?,
            x if x.m("-hold") => hold = opt_flg(arg, hold)?,
            _ => {
                (uncertainty, object_list) =
                    pos_args2(Some(arg), (uncertainty, object_list), &location)?
            }
        }
    }

    let uncertainty = mandatory(uncertainty, "uncertainty", &location)?;

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
        location,
    }))
}

/// set_data_check
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for SetDataCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_data_check".to_string();
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_named_opt_arg(&self.rise_from, "rise_from"));
        text.push_str(&fmt_named_opt_arg(&self.fall_from, "fall_from"));
        text.push_str(&fmt_named_opt_arg(&self.rise_to, "rise_to"));
        text.push_str(&fmt_named_opt_arg(&self.fall_to, "fall_to"));
        text.push_str(&fmt_named_flg(self.setup, "setup"));
        text.push_str(&fmt_named_flg(self.hold, "hold"));
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.push_str(&fmt_arg(&self.value));
        text.fmt(f)
    }
}

impl Validate for SetDataCheck {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_4, SDC2_1))?;
        self.arg_comb6(
            version.within(SDC1_2, SDC2_1),
            &self.from,
            &self.rise_from,
            &self.fall_from,
            &self.to,
            &self.rise_to,
            &self.fall_to,
            |a, b, c, d, e, f| (a | b | c) & (d | e | f),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_data_check(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-from",
            "-to",
            "-rise_from",
            "-fall_from",
            "-rise_to",
            "-fall_to",
            "-setup",
            "-hold",
            "-clock",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            x if x.m("-rise_from") => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            x if x.m("-fall_from") => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            x if x.m("-rise_to") => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            x if x.m("-fall_to") => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            x if x.m("-setup") => setup = opt_flg(arg, setup)?,
            x if x.m("-hold") => hold = opt_flg(arg, hold)?,
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            _ => value = pos_args1(Some(arg), value, &location)?,
        }
    }

    let value = mandatory(value, "value", &location)?;

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
        location,
    }))
}

/// set_disable_timing
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetDisableTiming {
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub cell_pin_list: Argument,
    location: Location,
}

impl fmt::Display for SetDisableTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_disable_timing".to_string();
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_arg(&self.cell_pin_list));
        text.fmt(f)
    }
}

impl Validate for SetDisableTiming {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_comb2(
            version.within(SDC1_2, SDC2_1),
            &self.from,
            &self.to,
            |a, b| !(a ^ b),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_disable_timing(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut from = None;
    let mut to = None;
    let mut cell_pin_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-from", "-to"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            _ => cell_pin_list = pos_args1(Some(arg), cell_pin_list, &location)?,
        }
    }

    let cell_pin_list = mandatory(cell_pin_list, "cell_pin_list", &location)?;

    Ok(Command::SetDisableTiming(SetDisableTiming {
        from,
        to,
        cell_pin_list,
        location,
    }))
}

/// set_drive
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetDrive {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub resistance: Argument,
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetDrive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_drive".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_arg(&self.resistance));
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetDrive {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_drive(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut resistance = None;
    let mut port_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-rise", "-fall", "-min", "-max"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            _ => {
                (resistance, port_list) = pos_args2(Some(arg), (resistance, port_list), &location)?
            }
        }
    }

    let resistance = mandatory(resistance, "resistance", &location)?;
    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetDrive(SetDrive {
        rise,
        fall,
        min,
        max,
        resistance,
        port_list,
        location,
    }))
}

/// set_driving_cell
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetDrivingCell {
    pub lib_cell: Argument,
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub library: Option<Argument>,
    pub pin: Option<Argument>,
    pub from_pin: Option<Argument>,
    pub multiply_by: Option<Argument>,
    pub dont_scale: bool,
    pub no_design_rule: bool,
    pub clock: Option<Argument>,
    pub clock_fall: bool,
    pub input_transition_rise: Option<Argument>,
    pub input_transition_fall: Option<Argument>,
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetDrivingCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_driving_cell".to_string();
        text.push_str(&fmt_named_arg(&self.lib_cell, "lib_cell"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_named_opt_arg(&self.library, "library"));
        text.push_str(&fmt_named_opt_arg(&self.pin, "pin"));
        text.push_str(&fmt_named_opt_arg(&self.from_pin, "from_pin"));
        text.push_str(&fmt_named_opt_arg(&self.multiply_by, "multiply_by"));
        text.push_str(&fmt_named_flg(self.dont_scale, "dont_scale"));
        text.push_str(&fmt_named_flg(self.no_design_rule, "no_design_rule"));
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.push_str(&fmt_named_flg(self.clock_fall, "clock_fall"));
        text.push_str(&fmt_named_opt_arg(
            &self.input_transition_rise,
            "input_transition_rise",
        ));
        text.push_str(&fmt_named_opt_arg(
            &self.input_transition_fall,
            "input_transition_fall",
        ));
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetDrivingCell {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_4, SDC2_1), &self.min, "min")?;
        self.arg_supported_version(version.within(SDC1_4, SDC2_1), &self.max, "max")?;
        self.arg_supported_version(version.within(SDC1_4, SDC2_1), &self.clock, "clock")?;
        self.arg_supported_version(
            version.within(SDC1_4, SDC2_1),
            &self.clock_fall,
            "clock_fall",
        )?;
        self.arg_supported_version(
            version.within(SDC1_1, SDC2_0),
            &self.multiply_by,
            "multiply_by",
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_driving_cell(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut lib_cell = None;
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut library = None;
    let mut pin = None;
    let mut from_pin = None;
    let mut multiply_by = None;
    let mut dont_scale = false;
    let mut no_design_rule = false;
    let mut clock = None;
    let mut clock_fall = false;
    let mut input_transition_rise = None;
    let mut input_transition_fall = None;
    let mut port_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-lib_cell",
            "-rise",
            "-fall",
            "-min",
            "-max",
            "-library",
            "-pin",
            "-from_pin",
            "-multiply_by",
            "-dont_scale",
            "-no_design_rule",
            "-clock",
            "-clock_fall",
            "-input_transition_rise",
            "-input_transition_fall",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-lib_cell") => lib_cell = opt_arg(arg, iter.next(), lib_cell)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            x if x.m("-library") => library = opt_arg(arg, iter.next(), library)?,
            x if x.m("-pin") => pin = opt_arg(arg, iter.next(), pin)?,
            x if x.m("-from_pin") => from_pin = opt_arg(arg, iter.next(), from_pin)?,
            x if x.m("-multiply_by") => multiply_by = opt_arg(arg, iter.next(), multiply_by)?,
            x if x.m("-dont_scale") => dont_scale = opt_flg(arg, dont_scale)?,
            x if x.m("-no_design_rule") => no_design_rule = opt_flg(arg, no_design_rule)?,
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            x if x.m("-clock_fall") => clock_fall = opt_flg(arg, clock_fall)?,
            x if x.m("-input_transition_rise") => {
                input_transition_rise = opt_arg(arg, iter.next(), input_transition_rise)?
            }
            x if x.m("-input_transition_fall") => {
                input_transition_fall = opt_arg(arg, iter.next(), input_transition_fall)?
            }
            _ => port_list = pos_args1(Some(arg), port_list, &location)?,
        }
    }

    let lib_cell = mandatory(lib_cell, "lib_cell", &location)?;
    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetDrivingCell(SetDrivingCell {
        lib_cell,
        rise,
        fall,
        min,
        max,
        library,
        pin,
        from_pin,
        multiply_by,
        dont_scale,
        no_design_rule,
        clock,
        clock_fall,
        input_transition_rise,
        input_transition_fall,
        port_list,
        location,
    }))
}

/// set_false_path
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetFalsePath {
    pub setup: bool,
    pub hold: bool,
    pub rise: bool,
    pub fall: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Vec<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Vec<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Vec<Argument>,
    pub comment: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetFalsePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_false_path".to_string();
        text.push_str(&fmt_named_flg(self.setup, "setup"));
        text.push_str(&fmt_named_flg(self.hold, "hold"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_named_vec_arg(&self.through, "through"));
        text.push_str(&fmt_named_opt_arg(&self.rise_from, "rise_from"));
        text.push_str(&fmt_named_opt_arg(&self.rise_to, "rise_to"));
        text.push_str(&fmt_named_vec_arg(&self.rise_through, "rise_through"));
        text.push_str(&fmt_named_opt_arg(&self.fall_from, "fall_from"));
        text.push_str(&fmt_named_opt_arg(&self.fall_to, "fall_to"));
        text.push_str(&fmt_named_vec_arg(&self.fall_through, "fall_through"));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.fmt(f)
    }
}

impl Validate for SetFalsePath {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_from, "rise_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_from, "fall_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_to, "rise_to")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_to, "fall_to")?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.rise_through,
            "rise_through",
        )?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.fall_through,
            "fall_through",
        )?;
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")?;
        self.arg_comb3(
            version.within(SDC1_1, SDC1_1),
            &self.from,
            &self.to,
            &self.through,
            |a, b, c| (a | b | c),
        )?;
        self.arg_comb7(
            version.within(SDC1_2, SDC2_1),
            &self.from,
            &self.to,
            &self.through,
            &self.rise,
            &self.fall,
            &self.setup,
            &self.hold,
            |a, b, c, d, e, f, g| (a | b | c) & !(d & e) & !(f & g),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_false_path(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut setup = false;
    let mut hold = false;
    let mut rise = false;
    let mut fall = false;
    let mut from = None;
    let mut to = None;
    let mut through = vec![];
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = vec![];
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = vec![];
    let mut comment = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-setup",
            "-hold",
            "-rise",
            "-fall",
            "-from",
            "-to",
            "-through",
            "-rise_from",
            "-rise_to",
            "-rise_through",
            "-fall_from",
            "-fall_to",
            "-fall_through",
            "-comment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-setup") => setup = opt_flg(arg, setup)?,
            x if x.m("-hold") => hold = opt_flg(arg, hold)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            x if x.m("-through") => through = vec_arg(arg, iter.next(), through)?,
            x if x.m("-rise_from") => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            x if x.m("-rise_to") => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            x if x.m("-rise_through") => rise_through = vec_arg(arg, iter.next(), rise_through)?,
            x if x.m("-fall_from") => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            x if x.m("-fall_to") => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            x if x.m("-fall_through") => fall_through = vec_arg(arg, iter.next(), fall_through)?,
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => return Err(SdcError::WrongArgument(arg)),
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
        location,
    }))
}

/// set_fanout_load
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetFanoutLoad {
    pub value: Argument,
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetFanoutLoad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_fanout_load".to_string();
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetFanoutLoad {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_fanout_load(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut value = None;
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (value, port_list) = pos_args2(Some(arg), (value, port_list), &location)?;
    }

    let value = mandatory(value, "value", &location)?;
    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetFanoutLoad(SetFanoutLoad {
        value,
        port_list,
        location,
    }))
}

/// set_hierarchy_separator
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetHierarchySeparator {
    pub separator: Argument,
    location: Location,
}

impl fmt::Display for SetHierarchySeparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_hierarchy_separator".to_string();
        text.push_str(&fmt_arg(&self.separator));
        text.fmt(f)
    }
}

impl Validate for SetHierarchySeparator {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_2, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_hierarchy_separator(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut separator = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        separator = pos_args1(Some(arg), separator, &location)?;
    }

    let separator = mandatory(separator, "separator", &location)?;

    Ok(Command::SetHierarchySeparator(SetHierarchySeparator {
        separator,
        location,
    }))
}

/// set_ideal_latency
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetIdealLatency {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub delay: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetIdealLatency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_ideal_latency".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_arg(&self.delay));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetIdealLatency {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_ideal_latency(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut delay = None;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-rise", "-fall", "-min", "-max"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            _ => (delay, object_list) = pos_args2(Some(arg), (delay, object_list), &location)?,
        }
    }

    let delay = mandatory(delay, "delay", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetIdealLatency(SetIdealLatency {
        rise,
        fall,
        min,
        max,
        delay,
        object_list,
        location,
    }))
}

/// set_ideal_network
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetIdealNetwork {
    pub no_propagate: bool,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetIdealNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_ideal_network".to_string();
        text.push_str(&fmt_named_flg(self.no_propagate, "no_propagate"));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetIdealNetwork {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_ideal_network(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut no_propagate = false;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-no_propagate"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-no_propagate") => no_propagate = opt_flg(arg, no_propagate)?,
            _ => object_list = pos_args1(Some(arg), object_list, &location)?,
        }
    }

    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetIdealNetwork(SetIdealNetwork {
        no_propagate,
        object_list,
        location,
    }))
}

/// set_ideal_transition
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetIdealTransition {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub transition_time: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetIdealTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_ideal_transition".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_arg(&self.transition_time));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetIdealTransition {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_ideal_transition(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut transition_time = None;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-rise", "-fall", "-min", "-max"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            _ => {
                (transition_time, object_list) =
                    pos_args2(Some(arg), (transition_time, object_list), &location)?
            }
        }
    }

    let transition_time = mandatory(transition_time, "transition_time", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetIdealTransition(SetIdealTransition {
        rise,
        fall,
        min,
        max,
        transition_time,
        object_list,
        location,
    }))
}

/// set_input_delay
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for SetInputDelay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_input_delay".to_string();
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.push_str(&fmt_named_opt_arg(&self.reference_pin, "reference_pin"));
        text.push_str(&fmt_named_flg(self.clock_fall, "clock_fall"));
        text.push_str(&fmt_named_flg(self.level_sensitive, "level_sensitive"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_named_flg(self.add_delay, "add_delay"));
        text.push_str(&fmt_named_flg(
            self.network_latency_included,
            "network_latency_included",
        ));
        text.push_str(&fmt_named_flg(
            self.source_latency_included,
            "source_latency_included",
        ));
        text.push_str(&fmt_arg(&self.delay_value));
        text.push_str(&fmt_arg(&self.port_pin_list));
        text.fmt(f)
    }
}

impl Validate for SetInputDelay {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(
            version.within(SDC1_4, SDC2_1),
            &self.network_latency_included,
            "network_latency_included",
        )?;
        self.arg_supported_version(
            version.within(SDC1_4, SDC2_1),
            &self.source_latency_included,
            "source_latency_included",
        )?;
        self.arg_supported_version(
            version.within(SDC2_0, SDC2_1),
            &self.reference_pin,
            "reference_pin",
        )?;
        self.arg_comb3(
            version.within(SDC1_2, SDC2_1),
            &self.clock_fall,
            &self.level_sensitive,
            &self.clock,
            |a, b, c| !((a | b) & !c),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_input_delay(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-clock",
            "-reference_pin",
            "-clock_fall",
            "-level_sensitive",
            "-rise",
            "-fall",
            "-max",
            "-min",
            "-add_delay",
            "-network_latency_included",
            "-source_latency_included",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            x if x.m("-reference_pin") => reference_pin = opt_arg(arg, iter.next(), reference_pin)?,
            x if x.m("-clock_fall") => clock_fall = opt_flg(arg, clock_fall)?,
            x if x.m("-level_sensitive") => level_sensitive = opt_flg(arg, level_sensitive)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-add_delay") => add_delay = opt_flg(arg, add_delay)?,
            x if x.m("-network_latency_included") => {
                network_latency_included = opt_flg(arg, network_latency_included)?
            }
            x if x.m("-source_latency_included") => {
                source_latency_included = opt_flg(arg, source_latency_included)?
            }
            _ => {
                (delay_value, port_pin_list) =
                    pos_args2(Some(arg), (delay_value, port_pin_list), &location)?
            }
        }
    }

    let delay_value = mandatory(delay_value, "delay_value", &location)?;
    let port_pin_list = mandatory(port_pin_list, "port_pin_list", &location)?;

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
        location,
    }))
}

/// set_input_transition
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetInputTransition {
    pub rise: bool,
    pub fall: bool,
    pub min: bool,
    pub max: bool,
    pub clock: Option<Argument>,
    pub clock_fall: bool,
    pub transition: Argument,
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetInputTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_input_transition".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.push_str(&fmt_named_flg(self.clock_fall, "clock_fall"));
        text.push_str(&fmt_arg(&self.transition));
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetInputTransition {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_4, SDC2_1), &self.clock, "clock")?;
        self.arg_supported_version(
            version.within(SDC1_4, SDC2_1),
            &self.clock_fall,
            "clock_fall",
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_input_transition(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut min = false;
    let mut max = false;
    let mut clock = None;
    let mut clock_fall = false;
    let mut transition = None;
    let mut port_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&["-rise", "-fall", "-min", "-max", "-clock", "-clock_fall"])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            x if x.m("-clock_fall") => clock_fall = opt_flg(arg, clock_fall)?,
            _ => {
                (transition, port_list) = pos_args2(Some(arg), (transition, port_list), &location)?
            }
        }
    }

    let transition = mandatory(transition, "transition", &location)?;
    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetInputTransition(SetInputTransition {
        rise,
        fall,
        min,
        max,
        clock,
        clock_fall,
        transition,
        port_list,
        location,
    }))
}

/// set_level_shifter_strategy
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetLevelShifterStrategy {
    pub rule: Argument,
    location: Location,
}

impl fmt::Display for SetLevelShifterStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_level_shifter_strategy".to_string();
        text.push_str(&fmt_named_arg(&self.rule, "rule"));
        text.fmt(f)
    }
}

impl Validate for SetLevelShifterStrategy {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_6, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_level_shifter_strategy(
    args: Vec<Argument>,
    location: Location,
) -> Result<Command, SdcError> {
    let mut rule = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-rule"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rule") => rule = opt_arg(arg, iter.next(), rule)?,
            _ => return Err(SdcError::WrongArgument(arg)),
        }
    }

    let rule = mandatory(rule, "rule", &location)?;

    Ok(Command::SetLevelShifterStrategy(SetLevelShifterStrategy {
        rule,
        location,
    }))
}

/// set_level_shifter_threshold
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetLevelShifterThreshold {
    pub voltage: Argument,
    pub percent: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetLevelShifterThreshold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_level_shifter_threshold".to_string();
        text.push_str(&fmt_named_arg(&self.voltage, "voltage"));
        text.push_str(&fmt_named_opt_arg(&self.percent, "percent"));
        text.fmt(f)
    }
}

impl Validate for SetLevelShifterThreshold {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_6, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_level_shifter_threshold(
    args: Vec<Argument>,
    location: Location,
) -> Result<Command, SdcError> {
    let mut voltage = None;
    let mut percent = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-voltage", "-percent"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-voltage") => voltage = opt_arg(arg, iter.next(), voltage)?,
            x if x.m("-percent") => percent = opt_arg(arg, iter.next(), percent)?,
            _ => return Err(SdcError::WrongArgument(arg)),
        }
    }

    let voltage = mandatory(voltage, "voltage", &location)?;

    Ok(Command::SetLevelShifterThreshold(
        SetLevelShifterThreshold {
            voltage,
            percent,
            location,
        },
    ))
}

/// set_load
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetLoad {
    pub min: bool,
    pub max: bool,
    pub subtract_pin_load: bool,
    pub pin_load: bool,
    pub wire_load: bool,
    pub value: Argument,
    pub objects: Argument,
    location: Location,
}

impl fmt::Display for SetLoad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_load".to_string();
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_named_flg(self.subtract_pin_load, "subtract_pin_load"));
        text.push_str(&fmt_named_flg(self.pin_load, "pin_load"));
        text.push_str(&fmt_named_flg(self.wire_load, "wire_load"));
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.objects));
        text.fmt(f)
    }
}

impl Validate for SetLoad {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_load(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut min = false;
    let mut max = false;
    let mut subtract_pin_load = false;
    let mut pin_load = false;
    let mut wire_load = false;
    let mut value = None;
    let mut objects = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-min",
            "-max",
            "-subtract_pin_load",
            "-pin_load",
            "-wire_load",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            x if x.m("-subtract_pin_load") => subtract_pin_load = opt_flg(arg, subtract_pin_load)?,
            x if x.m("-pin_load") => pin_load = opt_flg(arg, pin_load)?,
            x if x.m("-wire_load") => wire_load = opt_flg(arg, wire_load)?,
            _ => (value, objects) = pos_args2(Some(arg), (value, objects), &location)?,
        }
    }

    let value = mandatory(value, "value", &location)?;
    let objects = mandatory(objects, "objects", &location)?;

    Ok(Command::SetLoad(SetLoad {
        min,
        max,
        subtract_pin_load,
        pin_load,
        wire_load,
        value,
        objects,
        location,
    }))
}

/// set_logic_dc
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetLogicDc {
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetLogicDc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_logic_dc".to_string();
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetLogicDc {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_logic_dc(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        port_list = pos_args1(Some(arg), port_list, &location)?;
    }

    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetLogicDc(SetLogicDc {
        port_list,
        location,
    }))
}

/// set_logic_one
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetLogicOne {
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetLogicOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_logic_one".to_string();
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetLogicOne {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_logic_one(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        port_list = pos_args1(Some(arg), port_list, &location)?;
    }

    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetLogicOne(SetLogicOne {
        port_list,
        location,
    }))
}

/// set_logic_zero
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetLogicZero {
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetLogicZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_logic_zero".to_string();
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetLogicZero {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_logic_zero(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        port_list = pos_args1(Some(arg), port_list, &location)?;
    }

    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetLogicZero(SetLogicZero {
        port_list,
        location,
    }))
}

/// set_max_area
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxArea {
    pub area_value: Argument,
    location: Location,
}

impl fmt::Display for SetMaxArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_area".to_string();
        text.push_str(&fmt_arg(&self.area_value));
        text.fmt(f)
    }
}

impl Validate for SetMaxArea {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_area(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut area_value = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        area_value = pos_args1(Some(arg), area_value, &location)?;
    }

    let area_value = mandatory(area_value, "area_value", &location)?;

    Ok(Command::SetMaxArea(SetMaxArea {
        area_value,
        location,
    }))
}

/// set_max_capacitance
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxCapacitance {
    pub value: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetMaxCapacitance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_capacitance".to_string();
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetMaxCapacitance {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_capacitance(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (value, object_list) = pos_args2(Some(arg), (value, object_list), &location)?;
    }

    let value = mandatory(value, "value", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetMaxCapacitance(SetMaxCapacitance {
        value,
        object_list,
        location,
    }))
}

/// set_max_delay
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxDelay {
    pub rise: bool,
    pub fall: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Vec<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Vec<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Vec<Argument>,
    pub ignore_clock_latency: bool,
    pub comment: Option<Argument>,
    pub delay_value: Argument,
    location: Location,
}

impl fmt::Display for SetMaxDelay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_delay".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_named_vec_arg(&self.through, "through"));
        text.push_str(&fmt_named_opt_arg(&self.rise_from, "rise_from"));
        text.push_str(&fmt_named_opt_arg(&self.rise_to, "rise_to"));
        text.push_str(&fmt_named_vec_arg(&self.rise_through, "rise_through"));
        text.push_str(&fmt_named_opt_arg(&self.fall_from, "fall_from"));
        text.push_str(&fmt_named_opt_arg(&self.fall_to, "fall_to"));
        text.push_str(&fmt_named_vec_arg(&self.fall_through, "fall_through"));
        text.push_str(&fmt_named_flg(
            self.ignore_clock_latency,
            "ignore_clock_latency",
        ));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.push_str(&fmt_arg(&self.delay_value));
        text.fmt(f)
    }
}

impl Validate for SetMaxDelay {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_from, "rise_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_from, "fall_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_to, "rise_to")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_to, "fall_to")?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.rise_through,
            "rise_through",
        )?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.fall_through,
            "fall_through",
        )?;
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")?;
        self.arg_supported_version(
            version.within(SDC2_1, SDC2_1),
            &self.ignore_clock_latency,
            "ignore_clock_latency",
        )?;
        self.arg_comb2(
            version.within(SDC1_2, SDC2_1),
            &self.rise,
            &self.fall,
            |a, b| !(a & b),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_delay(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut from = None;
    let mut to = None;
    let mut through = vec![];
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = vec![];
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = vec![];
    let mut ignore_clock_latency = false;
    let mut comment = None;
    let mut delay_value = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-rise",
            "-fall",
            "-from",
            "-to",
            "-through",
            "-rise_from",
            "-rise_to",
            "-rise_through",
            "-fall_from",
            "-fall_to",
            "-fall_through",
            "-ignore_clock_latency",
            "-comment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            x if x.m("-through") => through = vec_arg(arg, iter.next(), through)?,
            x if x.m("-rise_from") => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            x if x.m("-rise_to") => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            x if x.m("-rise_through") => rise_through = vec_arg(arg, iter.next(), rise_through)?,
            x if x.m("-fall_from") => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            x if x.m("-fall_to") => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            x if x.m("-fall_through") => fall_through = vec_arg(arg, iter.next(), fall_through)?,
            x if x.m("-ignore_clock_latency") => {
                ignore_clock_latency = opt_flg(arg, ignore_clock_latency)?
            }
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => delay_value = pos_args1(Some(arg), delay_value, &location)?,
        }
    }

    let delay_value = mandatory(delay_value, "delay_value", &location)?;

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
        location,
    }))
}

/// set_max_dynamic_power
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxDynamicPower {
    pub power: Argument,
    pub unit: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetMaxDynamicPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_dynamic_power".to_string();
        text.push_str(&fmt_arg(&self.power));
        text.push_str(&fmt_opt_arg(&self.unit));
        text.fmt(f)
    }
}

impl Validate for SetMaxDynamicPower {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_4, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_dynamic_power(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut power = None;
    let mut unit = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (power, unit) = pos_args2(Some(arg), (power, unit), &location)?;
    }

    let power = mandatory(power, "power", &location)?;

    Ok(Command::SetMaxDynamicPower(SetMaxDynamicPower {
        power,
        unit,
        location,
    }))
}

/// set_max_fanout
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxFanout {
    pub value: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetMaxFanout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_fanout".to_string();
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetMaxFanout {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_fanout(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (value, object_list) = pos_args2(Some(arg), (value, object_list), &location)?;
    }

    let value = mandatory(value, "value", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetMaxFanout(SetMaxFanout {
        value,
        object_list,
        location,
    }))
}

/// set_max_leakage_power
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxLeakagePower {
    pub power: Argument,
    pub unit: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetMaxLeakagePower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_leakage_power".to_string();
        text.push_str(&fmt_arg(&self.power));
        text.push_str(&fmt_opt_arg(&self.unit));
        text.fmt(f)
    }
}

impl Validate for SetMaxLeakagePower {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_4, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_leakage_power(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut power = None;
    let mut unit = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (power, unit) = pos_args2(Some(arg), (power, unit), &location)?;
    }

    let power = mandatory(power, "power", &location)?;

    Ok(Command::SetMaxLeakagePower(SetMaxLeakagePower {
        power,
        unit,
        location,
    }))
}

/// set_max_time_borrow
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxTimeBorrow {
    pub delay_value: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetMaxTimeBorrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_time_borrow".to_string();
        text.push_str(&fmt_arg(&self.delay_value));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetMaxTimeBorrow {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_time_borrow(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut delay_value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (delay_value, object_list) = pos_args2(Some(arg), (delay_value, object_list), &location)?;
    }

    let delay_value = mandatory(delay_value, "delay_value", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetMaxTimeBorrow(SetMaxTimeBorrow {
        delay_value,
        object_list,
        location,
    }))
}

/// set_max_transition
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMaxTransition {
    pub clock_path: bool,
    pub rise: bool,
    pub fall: bool,
    pub value: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetMaxTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_max_transition".to_string();
        text.push_str(&fmt_named_flg(self.clock_path, "clock_path"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetMaxTransition {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(
            version.within(SDC1_5, SDC2_1),
            &self.clock_path,
            "clock_path",
        )?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.rise, "rise")?;
        self.arg_supported_version(version.within(SDC1_5, SDC2_1), &self.fall, "fall")
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_max_transition(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut clock_path = false;
    let mut rise = false;
    let mut fall = false;
    let mut value = None;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-clock_path", "-rise", "-fall"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-clock_path") => clock_path = opt_flg(arg, clock_path)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            _ => (value, object_list) = pos_args2(Some(arg), (value, object_list), &location)?,
        }
    }

    let value = mandatory(value, "value", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetMaxTransition(SetMaxTransition {
        clock_path,
        rise,
        fall,
        value,
        object_list,
        location,
    }))
}

/// set_min_capacitance
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMinCapacitance {
    pub value: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetMinCapacitance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_min_capacitance".to_string();
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetMinCapacitance {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_min_capacitance(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (value, object_list) = pos_args2(Some(arg), (value, object_list), &location)?;
    }

    let value = mandatory(value, "value", &location)?;
    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetMinCapacitance(SetMinCapacitance {
        value,
        object_list,
        location,
    }))
}

/// set_min_delay
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMinDelay {
    pub rise: bool,
    pub fall: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Vec<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Vec<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Vec<Argument>,
    pub ignore_clock_latency: bool,
    pub comment: Option<Argument>,
    pub delay_value: Argument,
    location: Location,
}

impl fmt::Display for SetMinDelay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_min_delay".to_string();
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_named_vec_arg(&self.through, "through"));
        text.push_str(&fmt_named_opt_arg(&self.rise_from, "rise_from"));
        text.push_str(&fmt_named_opt_arg(&self.rise_to, "rise_to"));
        text.push_str(&fmt_named_vec_arg(&self.rise_through, "rise_through"));
        text.push_str(&fmt_named_opt_arg(&self.fall_from, "fall_from"));
        text.push_str(&fmt_named_opt_arg(&self.fall_to, "fall_to"));
        text.push_str(&fmt_named_vec_arg(&self.fall_through, "fall_through"));
        text.push_str(&fmt_named_flg(
            self.ignore_clock_latency,
            "ignore_clock_latency",
        ));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.push_str(&fmt_arg(&self.delay_value));
        text.fmt(f)
    }
}

impl Validate for SetMinDelay {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_from, "rise_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_from, "fall_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_to, "rise_to")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_to, "fall_to")?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.rise_through,
            "rise_through",
        )?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.fall_through,
            "fall_through",
        )?;
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")?;
        self.arg_supported_version(
            version.within(SDC2_1, SDC2_1),
            &self.ignore_clock_latency,
            "ignore_clock_latency",
        )?;
        self.arg_comb2(
            version.within(SDC1_2, SDC2_1),
            &self.rise,
            &self.fall,
            |a, b| !(a & b),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_min_delay(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut rise = false;
    let mut fall = false;
    let mut from = None;
    let mut to = None;
    let mut through = vec![];
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = vec![];
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = vec![];
    let mut ignore_clock_latency = false;
    let mut comment = None;
    let mut delay_value = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-rise",
            "-fall",
            "-from",
            "-to",
            "-through",
            "-rise_from",
            "-rise_to",
            "-rise_through",
            "-fall_from",
            "-fall_to",
            "-fall_through",
            "-ignore_clock_latency",
            "-comment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            x if x.m("-through") => through = vec_arg(arg, iter.next(), through)?,
            x if x.m("-rise_from") => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            x if x.m("-rise_to") => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            x if x.m("-rise_through") => rise_through = vec_arg(arg, iter.next(), rise_through)?,
            x if x.m("-fall_from") => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            x if x.m("-fall_to") => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            x if x.m("-fall_through") => fall_through = vec_arg(arg, iter.next(), fall_through)?,
            x if x.m("-ignore_clock_latency") => {
                ignore_clock_latency = opt_flg(arg, ignore_clock_latency)?
            }
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => delay_value = pos_args1(Some(arg), delay_value, &location)?,
        }
    }

    let delay_value = mandatory(delay_value, "delay_value", &location)?;

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
        location,
    }))
}

/// set_min_porosity
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMinPorosity {
    pub porosity_value: Argument,
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetMinPorosity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_min_porosity".to_string();
        text.push_str(&fmt_arg(&self.porosity_value));
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetMinPorosity {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_4, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_min_porosity(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut porosity_value = None;
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (porosity_value, object_list) =
            pos_args2(Some(arg), (porosity_value, object_list), &location)?;
    }

    let porosity_value = mandatory(porosity_value, "porosity_value", &location)?;
    let object_list = mandatory(object_list, "value", &location)?;

    Ok(Command::SetMinPorosity(SetMinPorosity {
        porosity_value,
        object_list,
        location,
    }))
}

/// set_min_pulse_width
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMinPulseWidth {
    pub low: bool,
    pub high: bool,
    pub value: Argument,
    pub object_list: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetMinPulseWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_min_pulse_width".to_string();
        text.push_str(&fmt_named_flg(self.low, "low"));
        text.push_str(&fmt_named_flg(self.high, "high"));
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_opt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetMinPulseWidth {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC2_0, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_min_pulse_width(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut low = false;
    let mut high = false;
    let mut value = None;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-low", "-high"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-low") => low = opt_flg(arg, low)?,
            x if x.m("-high") => high = opt_flg(arg, high)?,
            _ => (value, object_list) = pos_args2(Some(arg), (value, object_list), &location)?,
        }
    }

    let value = mandatory(value, "value", &location)?;

    Ok(Command::SetMinPulseWidth(SetMinPulseWidth {
        low,
        high,
        value,
        object_list,
        location,
    }))
}

/// set_multicycle_path
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMulticyclePath {
    pub setup: bool,
    pub hold: bool,
    pub rise: bool,
    pub fall: bool,
    pub start: bool,
    pub end: bool,
    pub from: Option<Argument>,
    pub to: Option<Argument>,
    pub through: Vec<Argument>,
    pub rise_from: Option<Argument>,
    pub rise_to: Option<Argument>,
    pub rise_through: Vec<Argument>,
    pub fall_from: Option<Argument>,
    pub fall_to: Option<Argument>,
    pub fall_through: Vec<Argument>,
    pub comment: Option<Argument>,
    pub path_multiplier: Argument,
    location: Location,
}

impl fmt::Display for SetMulticyclePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_multicycle_path".to_string();
        text.push_str(&fmt_named_flg(self.setup, "setup"));
        text.push_str(&fmt_named_flg(self.hold, "hold"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.start, "start"));
        text.push_str(&fmt_named_flg(self.end, "end"));
        text.push_str(&fmt_named_opt_arg(&self.from, "from"));
        text.push_str(&fmt_named_opt_arg(&self.to, "to"));
        text.push_str(&fmt_named_vec_arg(&self.through, "through"));
        text.push_str(&fmt_named_opt_arg(&self.rise_from, "rise_from"));
        text.push_str(&fmt_named_opt_arg(&self.rise_to, "rise_to"));
        text.push_str(&fmt_named_vec_arg(&self.rise_through, "rise_through"));
        text.push_str(&fmt_named_opt_arg(&self.fall_from, "fall_from"));
        text.push_str(&fmt_named_opt_arg(&self.fall_to, "fall_to"));
        text.push_str(&fmt_named_vec_arg(&self.fall_through, "fall_through"));
        text.push_str(&fmt_named_opt_arg(&self.comment, "comment"));
        text.push_str(&fmt_arg(&self.path_multiplier));
        text.fmt(f)
    }
}

impl Validate for SetMulticyclePath {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_from, "rise_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_from, "fall_from")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.rise_to, "rise_to")?;
        self.arg_supported_version(version.within(SDC1_7, SDC2_1), &self.fall_to, "fall_to")?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.rise_through,
            "rise_through",
        )?;
        self.arg_supported_version(
            version.within(SDC1_7, SDC2_1),
            &self.fall_through,
            "fall_through",
        )?;
        self.arg_supported_version(version.within(SDC1_9, SDC2_1), &self.comment, "comment")
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_multicycle_path(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut setup = false;
    let mut hold = false;
    let mut rise = false;
    let mut fall = false;
    let mut start = false;
    let mut end = false;
    let mut from = None;
    let mut to = None;
    let mut through = vec![];
    let mut rise_from = None;
    let mut rise_to = None;
    let mut rise_through = vec![];
    let mut fall_from = None;
    let mut fall_to = None;
    let mut fall_through = vec![];
    let mut comment = None;
    let mut path_multiplier = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-setup",
            "-hold",
            "-rise",
            "-fall",
            "-start",
            "-end",
            "-from",
            "-to",
            "-through",
            "-rise_from",
            "-rise_to",
            "-rise_through",
            "-fall_from",
            "-fall_to",
            "-fall_through",
            "-comment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-setup") => setup = opt_flg(arg, setup)?,
            x if x.m("-hold") => hold = opt_flg(arg, hold)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-start") => start = opt_flg(arg, start)?,
            x if x.m("-end") => end = opt_flg(arg, end)?,
            x if x.m("-from") => from = opt_arg(arg, iter.next(), from)?,
            x if x.m("-to") => to = opt_arg(arg, iter.next(), to)?,
            x if x.m("-through") => through = vec_arg(arg, iter.next(), through)?,
            x if x.m("-rise_from") => rise_from = opt_arg(arg, iter.next(), rise_from)?,
            x if x.m("-rise_to") => rise_to = opt_arg(arg, iter.next(), rise_to)?,
            x if x.m("-rise_through") => rise_through = vec_arg(arg, iter.next(), rise_through)?,
            x if x.m("-fall_from") => fall_from = opt_arg(arg, iter.next(), fall_from)?,
            x if x.m("-fall_to") => fall_to = opt_arg(arg, iter.next(), fall_to)?,
            x if x.m("-fall_through") => fall_through = vec_arg(arg, iter.next(), fall_through)?,
            x if x.m("-comment") => comment = opt_arg(arg, iter.next(), comment)?,
            _ => path_multiplier = pos_args1(Some(arg), path_multiplier, &location)?,
        }
    }

    let path_multiplier = mandatory(path_multiplier, "path_multiplier", &location)?;

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
        location,
    }))
}

/// set_operating_conditions
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetOperatingConditions {
    pub library: Option<Argument>,
    pub analysis_type: Option<Argument>,
    pub max: Option<Argument>,
    pub min: Option<Argument>,
    pub max_library: Option<Argument>,
    pub min_library: Option<Argument>,
    pub object_list: Option<Argument>,
    pub condition: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetOperatingConditions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_operating_conditions".to_string();
        text.push_str(&fmt_named_opt_arg(&self.library, "library"));
        text.push_str(&fmt_named_opt_arg(&self.analysis_type, "analysis_type"));
        text.push_str(&fmt_named_opt_arg(&self.max, "max"));
        text.push_str(&fmt_named_opt_arg(&self.min, "min"));
        text.push_str(&fmt_named_opt_arg(&self.max_library, "max_library"));
        text.push_str(&fmt_named_opt_arg(&self.min_library, "min_library"));
        text.push_str(&fmt_named_opt_arg(&self.object_list, "object_list"));
        text.push_str(&fmt_opt_arg(&self.condition));
        text.fmt(f)
    }
}

impl Validate for SetOperatingConditions {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(
            version.within(SDC1_5, SDC2_1),
            &self.object_list,
            "object_list",
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_operating_conditions(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut library = None;
    let mut analysis_type = None;
    let mut max = None;
    let mut min = None;
    let mut max_library = None;
    let mut min_library = None;
    let mut object_list = None;
    let mut condition = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-library",
            "-analysis_type",
            "-max",
            "-min",
            "-max_library",
            "-min_library",
            "-object_list",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-library") => library = opt_arg(arg, iter.next(), library)?,
            x if x.m("-analysis_type") => analysis_type = opt_arg(arg, iter.next(), analysis_type)?,
            x if x.m("-max") => max = opt_arg(arg, iter.next(), max)?,
            x if x.m("-min") => min = opt_arg(arg, iter.next(), min)?,
            x if x.m("-max_library") => max_library = opt_arg(arg, iter.next(), max_library)?,
            x if x.m("-min_library") => min_library = opt_arg(arg, iter.next(), min_library)?,
            x if x.m("-object_list") => object_list = opt_arg(arg, iter.next(), object_list)?,
            _ => condition = pos_args1(Some(arg), condition, &location)?,
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
        location,
    }))
}

/// set_output_delay
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for SetOutputDelay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_output_delay".to_string();
        text.push_str(&fmt_named_opt_arg(&self.clock, "clock"));
        text.push_str(&fmt_named_opt_arg(&self.reference_pin, "reference_pin"));
        text.push_str(&fmt_named_flg(self.clock_fall, "clock_fall"));
        text.push_str(&fmt_named_flg(self.level_sensitive, "level_sensitive"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_named_flg(self.add_delay, "add_delay"));
        text.push_str(&fmt_named_flg(
            self.network_latency_included,
            "network_latency_included",
        ));
        text.push_str(&fmt_named_flg(
            self.source_latency_included,
            "source_latency_included",
        ));
        text.push_str(&fmt_arg(&self.delay_value));
        text.push_str(&fmt_arg(&self.port_pin_list));
        text.fmt(f)
    }
}

impl Validate for SetOutputDelay {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))?;
        self.arg_supported_version(
            version.within(SDC1_4, SDC2_1),
            &self.network_latency_included,
            "network_latency_included",
        )?;
        self.arg_supported_version(
            version.within(SDC1_4, SDC2_1),
            &self.source_latency_included,
            "source_latency_included",
        )?;
        self.arg_supported_version(
            version.within(SDC2_0, SDC2_1),
            &self.reference_pin,
            "reference_pin",
        )?;
        self.arg_comb3(
            version.within(SDC1_2, SDC2_1),
            &self.clock_fall,
            &self.level_sensitive,
            &self.clock,
            |a, b, c| !((a | b) & !c),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_output_delay(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-clock",
            "-reference_pin",
            "-clock_fall",
            "-level_sensitive",
            "-rise",
            "-fall",
            "-max",
            "-min",
            "-add_delay",
            "-network_latency_included",
            "-source_latency_included",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-clock") => clock = opt_arg(arg, iter.next(), clock)?,
            x if x.m("-reference_pin") => reference_pin = opt_arg(arg, iter.next(), reference_pin)?,
            x if x.m("-clock_fall") => clock_fall = opt_flg(arg, clock_fall)?,
            x if x.m("-level_sensitive") => level_sensitive = opt_flg(arg, level_sensitive)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-add_delay") => add_delay = opt_flg(arg, add_delay)?,
            x if x.m("-network_latency_included") => {
                network_latency_included = opt_flg(arg, network_latency_included)?
            }
            x if x.m("-source_latency_included") => {
                source_latency_included = opt_flg(arg, source_latency_included)?
            }
            _ => {
                (delay_value, port_pin_list) =
                    pos_args2(Some(arg), (delay_value, port_pin_list), &location)?
            }
        }
    }

    let delay_value = mandatory(delay_value, "delay_value", &location)?;
    let port_pin_list = mandatory(port_pin_list, "port_pin_list", &location)?;

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
        location,
    }))
}

/// set_port_fanout_number
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetPortFanoutNumber {
    pub value: Argument,
    pub port_list: Argument,
    location: Location,
}

impl fmt::Display for SetPortFanoutNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_port_fanout_number".to_string();
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.port_list));
        text.fmt(f)
    }
}

impl Validate for SetPortFanoutNumber {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_port_fanout_number(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut value = None;
    let mut port_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        (value, port_list) = pos_args2(Some(arg), (value, port_list), &location)?;
    }

    let value = mandatory(value, "value", &location)?;
    let port_list = mandatory(port_list, "port_list", &location)?;

    Ok(Command::SetPortFanoutNumber(SetPortFanoutNumber {
        value,
        port_list,
        location,
    }))
}

/// set_propagated_clock
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetPropagatedClock {
    pub object_list: Argument,
    location: Location,
}

impl fmt::Display for SetPropagatedClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_propagated_clock".to_string();
        text.push_str(&fmt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetPropagatedClock {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_propagated_clock(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut object_list = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        object_list = pos_args1(Some(arg), object_list, &location)?;
    }

    let object_list = mandatory(object_list, "object_list", &location)?;

    Ok(Command::SetPropagatedClock(SetPropagatedClock {
        object_list,
        location,
    }))
}

/// set_resistance
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetResistance {
    pub min: bool,
    pub max: bool,
    pub value: Argument,
    pub net_list: Argument,
    location: Location,
}

impl fmt::Display for SetResistance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_resistance".to_string();
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_arg(&self.value));
        text.push_str(&fmt_arg(&self.net_list));
        text.fmt(f)
    }
}

impl Validate for SetResistance {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_resistance(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut min = false;
    let mut max = false;
    let mut value = None;
    let mut net_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-min", "-max"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            _ => (value, net_list) = pos_args2(Some(arg), (value, net_list), &location)?,
        }
    }

    let value = mandatory(value, "value", &location)?;
    let net_list = mandatory(net_list, "net_list", &location)?;

    Ok(Command::SetResistance(SetResistance {
        min,
        max,
        value,
        net_list,
        location,
    }))
}

/// set_sense
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for SetSense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_sense".to_string();
        text.push_str(&fmt_named_opt_arg(&self.r#type, "type"));
        text.push_str(&fmt_named_flg(self.non_unate, "non_unate"));
        text.push_str(&fmt_named_flg(self.positive, "positive"));
        text.push_str(&fmt_named_flg(self.negative, "negative"));
        text.push_str(&fmt_named_flg(self.clock_leaf, "clock_leaf"));
        text.push_str(&fmt_named_flg(self.stop_propagation, "stop_propagation"));
        text.push_str(&fmt_named_opt_arg(&self.pulse, "pulse"));
        text.push_str(&fmt_named_opt_arg(&self.clocks, "clocks"));
        text.push_str(&fmt_arg(&self.pin_list));
        text.fmt(f)
    }
}

impl Validate for SetSense {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC2_1, SDC2_1))?;
        self.arg_comb6(
            version.within(SDC1_2, SDC2_1),
            &self.positive,
            &self.negative,
            &self.pulse,
            &self.stop_propagation,
            &self.non_unate,
            &self.clocks,
            |a, b, c, d, e, f| a ^ b ^ c ^ d ^ (e & f),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_sense(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut r#type = None;
    let mut non_unate = false;
    let mut positive = false;
    let mut negative = false;
    let mut clock_leaf = false;
    let mut stop_propagation = false;
    let mut pulse = None;
    let mut clocks = None;
    let mut pin_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-type",
            "-non_unate",
            "-positive",
            "-negative",
            "-clock_leaf",
            "-stop_propagation",
            "-pulse",
            "-clocks",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-type") => r#type = opt_arg(arg, iter.next(), r#type)?,
            x if x.m("-non_unate") => non_unate = opt_flg(arg, non_unate)?,
            x if x.m("-positive") => positive = opt_flg(arg, positive)?,
            x if x.m("-negative") => negative = opt_flg(arg, negative)?,
            x if x.m("-clock_leaf") => clock_leaf = opt_flg(arg, clock_leaf)?,
            x if x.m("-stop_propagation") => stop_propagation = opt_flg(arg, stop_propagation)?,
            x if x.m("-pulse") => pulse = opt_arg(arg, iter.next(), pulse)?,
            x if x.m("-clocks") => clocks = opt_arg(arg, iter.next(), clocks)?,
            _ => pin_list = pos_args1(Some(arg), pin_list, &location)?,
        }
    }

    let pin_list = mandatory(pin_list, "pin_list", &location)?;

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
        location,
    }))
}

/// set_timing_derate
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    location: Location,
}

impl fmt::Display for SetTimingDerate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_timing_derate".to_string();
        text.push_str(&fmt_named_flg(self.cell_delay, "cell_delay"));
        text.push_str(&fmt_named_flg(self.cell_check, "cell_check"));
        text.push_str(&fmt_named_flg(self.net_delay, "net_delay"));
        text.push_str(&fmt_named_flg(self.data, "data"));
        text.push_str(&fmt_named_flg(self.clock, "clock"));
        text.push_str(&fmt_named_flg(self.early, "early"));
        text.push_str(&fmt_named_flg(self.late, "late"));
        text.push_str(&fmt_named_flg(self.rise, "rise"));
        text.push_str(&fmt_named_flg(self.fall, "fall"));
        text.push_str(&fmt_named_flg(self.r#static, "static"));
        text.push_str(&fmt_named_flg(self.dynamic, "dynamic"));
        text.push_str(&fmt_named_flg(self.increment, "increment"));
        text.push_str(&fmt_arg(&self.derate_value));
        text.push_str(&fmt_opt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetTimingDerate {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_5, SDC2_1))?;
        self.arg_supported_version(version.within(SDC1_8, SDC2_1), &self.rise, "rise")?;
        self.arg_supported_version(version.within(SDC1_8, SDC2_1), &self.fall, "fall")?;
        self.arg_supported_version(version.within(SDC2_1, SDC2_1), &self.r#static, "static")?;
        self.arg_supported_version(version.within(SDC2_1, SDC2_1), &self.dynamic, "dynamic")?;
        self.arg_supported_version(version.within(SDC2_1, SDC2_1), &self.increment, "increment")?;
        self.arg_comb2(
            version.within(SDC1_5, SDC2_1),
            &self.early,
            &self.late,
            |a, b| (a & !b) | (b & !a),
        )
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_timing_derate(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
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

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-cell_delay",
            "-cell_check",
            "-net_delay",
            "-data",
            "-clock",
            "-early",
            "-late",
            "-rise",
            "-fall",
            "-static",
            "-dynamic",
            "-increment",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-cell_delay") => cell_delay = opt_flg(arg, cell_delay)?,
            x if x.m("-cell_check") => cell_check = opt_flg(arg, cell_check)?,
            x if x.m("-net_delay") => net_delay = opt_flg(arg, net_delay)?,
            x if x.m("-data") => data = opt_flg(arg, data)?,
            x if x.m("-clock") => clock = opt_flg(arg, clock)?,
            x if x.m("-early") => early = opt_flg(arg, early)?,
            x if x.m("-late") => late = opt_flg(arg, late)?,
            x if x.m("-rise") => rise = opt_flg(arg, rise)?,
            x if x.m("-fall") => fall = opt_flg(arg, fall)?,
            x if x.m("-static") => r#static = opt_flg(arg, r#static)?,
            x if x.m("-dynamic") => dynamic = opt_flg(arg, dynamic)?,
            x if x.m("-increment") => increment = opt_flg(arg, increment)?,
            _ => {
                (derate_value, object_list) =
                    pos_args2(Some(arg), (derate_value, object_list), &location)?
            }
        }
    }

    let derate_value = mandatory(derate_value, "derate_value", &location)?;

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
        location,
    }))
}

/// set_units
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetUnits {
    pub capacitance: Option<Argument>,
    pub resistance: Option<Argument>,
    pub time: Option<Argument>,
    pub voltage: Option<Argument>,
    pub current: Option<Argument>,
    pub power: Option<Argument>,
    location: Location,
    alias: bool,
}

impl fmt::Display for SetUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_units".to_string();
        text.push_str(&fmt_named_opt_arg(&self.capacitance, "capacitance"));
        text.push_str(&fmt_named_opt_arg(&self.resistance, "resistance"));
        text.push_str(&fmt_named_opt_arg(&self.time, "time"));
        text.push_str(&fmt_named_opt_arg(&self.voltage, "voltage"));
        text.push_str(&fmt_named_opt_arg(&self.current, "current"));
        text.push_str(&fmt_named_opt_arg(&self.power, "power"));
        text.fmt(f)
    }
}

impl Validate for SetUnits {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_7, SDC2_1))?;
        self.alias_supported_version(version.within(SDC1_7, SDC2_1), self.alias)
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_units(args: Vec<Argument>, location: Location, alias: bool) -> Result<Command, SdcError> {
    let mut capacitance = None;
    let mut resistance = None;
    let mut time = None;
    let mut voltage = None;
    let mut current = None;
    let mut power = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| {
        LazyDict::new(&[
            "-capacitance",
            "-resistance",
            "-time",
            "-voltage",
            "-current",
            "-power",
        ])
    });

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-capacitance") => capacitance = opt_arg(arg, iter.next(), capacitance)?,
            x if x.m("-resistance") => resistance = opt_arg(arg, iter.next(), resistance)?,
            x if x.m("-time") => time = opt_arg(arg, iter.next(), time)?,
            x if x.m("-voltage") => voltage = opt_arg(arg, iter.next(), voltage)?,
            x if x.m("-current") => current = opt_arg(arg, iter.next(), current)?,
            x if x.m("-power") => power = opt_arg(arg, iter.next(), power)?,
            _ => return Err(SdcError::WrongArgument(arg)),
        }
    }

    Ok(Command::SetUnits(SetUnits {
        capacitance,
        resistance,
        time,
        voltage,
        current,
        power,
        location,
        alias,
    }))
}

/// set_voltage
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetVoltage {
    pub min: Option<Argument>,
    pub object_list: Option<Argument>,
    pub max_case_voltage: Argument,
    location: Location,
}

impl fmt::Display for SetVoltage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_voltage".to_string();
        text.push_str(&fmt_named_opt_arg(&self.min, "min"));
        text.push_str(&fmt_named_opt_arg(&self.object_list, "object_list"));
        text.push_str(&fmt_arg(&self.max_case_voltage));
        text.fmt(f)
    }
}

impl Validate for SetVoltage {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_8, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_voltage(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut min = None;
    let mut object_list = None;
    let mut max_case_voltage = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-min", "-object_list"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-min") => min = opt_arg(arg, iter.next(), min)?,
            x if x.m("-object_list") => object_list = opt_arg(arg, iter.next(), object_list)?,
            _ => max_case_voltage = pos_args1(Some(arg), max_case_voltage, &location)?,
        }
    }

    let max_case_voltage = mandatory(max_case_voltage, "max_case_voltage", &location)?;

    Ok(Command::SetVoltage(SetVoltage {
        min,
        object_list,
        max_case_voltage,
        location,
    }))
}

/// set_wire_load_min_block_size
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetWireLoadMinBlockSize {
    pub size: Argument,
    location: Location,
}

impl fmt::Display for SetWireLoadMinBlockSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_wire_load_min_block_size".to_string();
        text.push_str(&fmt_arg(&self.size));
        text.fmt(f)
    }
}

impl Validate for SetWireLoadMinBlockSize {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_wire_load_min_block_size(
    args: Vec<Argument>,
    location: Location,
) -> Result<Command, SdcError> {
    let mut size = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        size = pos_args1(Some(arg), size, &location)?;
    }

    let size = mandatory(size, "size", &location)?;

    Ok(Command::SetWireLoadMinBlockSize(SetWireLoadMinBlockSize {
        size,
        location,
    }))
}

/// set_wire_load_mode
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetWireLoadMode {
    pub mode_name: Argument,
    location: Location,
}

impl fmt::Display for SetWireLoadMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_wire_load_mode".to_string();
        text.push_str(&fmt_arg(&self.mode_name));
        text.fmt(f)
    }
}

impl Validate for SetWireLoadMode {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_wire_load_mode(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut mode_name = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        mode_name = pos_args1(Some(arg), mode_name, &location)?;
    }

    let mode_name = mandatory(mode_name, "mode_name", &location)?;

    Ok(Command::SetWireLoadMode(SetWireLoadMode {
        mode_name,
        location,
    }))
}

/// set_wire_load_model
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetWireLoadModel {
    pub name: Argument,
    pub library: Option<Argument>,
    pub min: bool,
    pub max: bool,
    pub object_list: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetWireLoadModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_wire_load_model".to_string();
        text.push_str(&fmt_named_arg(&self.name, "name"));
        text.push_str(&fmt_named_opt_arg(&self.library, "library"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_opt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetWireLoadModel {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_wire_load_model(args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let mut name = None;
    let mut library = None;
    let mut min = false;
    let mut max = false;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-name", "-library", "-min", "-max"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-name") => name = opt_arg(arg, iter.next(), name)?,
            x if x.m("-library") => library = opt_arg(arg, iter.next(), library)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            _ => object_list = pos_args1(Some(arg), object_list, &location)?,
        }
    }

    let name = mandatory(name, "name", &location)?;

    Ok(Command::SetWireLoadModel(SetWireLoadModel {
        name,
        library,
        min,
        max,
        object_list,
        location,
    }))
}

/// set_wire_load_selection_group
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetWireLoadSelectionGroup {
    pub library: Option<Argument>,
    pub min: bool,
    pub max: bool,
    pub group_name: Argument,
    pub object_list: Option<Argument>,
    location: Location,
}

impl fmt::Display for SetWireLoadSelectionGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "set_wire_load_selection_group".to_string();
        text.push_str(&fmt_named_opt_arg(&self.library, "library"));
        text.push_str(&fmt_named_flg(self.min, "min"));
        text.push_str(&fmt_named_flg(self.max, "max"));
        text.push_str(&fmt_arg(&self.group_name));
        text.push_str(&fmt_opt_arg(&self.object_list));
        text.fmt(f)
    }
}

impl Validate for SetWireLoadSelectionGroup {
    fn validate(&self, version: SdcVersion) -> Result<(), SdcError> {
        self.cmd_supported_version(version.within(SDC1_1, SDC2_1))
        // TODO group_name change from opt_arg to pos_arg at SDC1.3
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn set_wire_load_selection_group(
    args: Vec<Argument>,
    location: Location,
) -> Result<Command, SdcError> {
    let mut library = None;
    let mut min = false;
    let mut max = false;
    let mut group_name = None;
    let mut object_list = None;

    static DICT: OnceLock<LazyDict> = OnceLock::new();
    let dict = DICT.get_or_init(|| LazyDict::new(&["-library", "-min", "-max"]));

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match LazyMatcher::new(arg.as_str(), dict, &location)? {
            x if x.m("-library") => library = opt_arg(arg, iter.next(), library)?,
            x if x.m("-min") => min = opt_flg(arg, min)?,
            x if x.m("-max") => max = opt_flg(arg, max)?,
            _ => {
                (group_name, object_list) =
                    pos_args2(Some(arg), (group_name, object_list), &location)?
            }
        }
    }

    let group_name = mandatory(group_name, "group_name", &location)?;

    Ok(Command::SetWireLoadSelectionGroup(
        SetWireLoadSelectionGroup {
            library,
            min,
            max,
            group_name,
            object_list,
            location,
        },
    ))
}

/// unknowun command
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unknown {
    pub name: String,
    pub args: Vec<Argument>,
    location: Location,
}

impl fmt::Display for Unknown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = self.name.clone();
        for arg in &self.args {
            text.push_str(&fmt_arg(arg));
        }
        text.fmt(f)
    }
}

impl Validate for Unknown {
    fn validate(&self, _: SdcVersion) -> Result<(), SdcError> {
        Err(SdcError::UnknownCommand(
            self.name.clone(),
            self.location.clone(),
        ))
    }

    fn location(&self) -> &Location {
        &self.location
    }
}

fn unknown(name: &str, args: Vec<Argument>, location: Location) -> Result<Command, SdcError> {
    let name = name.to_string();

    Ok(Command::Unknown(Unknown {
        name,
        args,
        location,
    }))
}
