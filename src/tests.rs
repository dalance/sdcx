use crate::sdc::*;
use crate::Parser;

fn success(code: &str) {
    let code = format!("{code}\n");
    let sdc = Parser::parse(&code, &"");
    dbg!(&code);
    dbg!(&sdc);
    assert!(sdc.is_ok());
}

fn check_header(code: &str, headers: Vec<String>) {
    let code = format!("{code}\n");
    let sdc = Parser::parse(&code, &"").unwrap();
    assert_eq!(sdc.header.len(), headers.len());
    for (i, h) in headers.iter().enumerate() {
        assert_eq!(sdc.header[i], *h);
    }
}

fn check_version(code: &str, version: Option<SdcVersion>) {
    let code = format!("{code}\n");
    let sdc = Parser::parse(&code, &"").unwrap();
    assert_eq!(sdc.version, version);
}

fn check_command(code: &str, command: Command) {
    let code = format!("{code}\n");
    let sdc = Parser::parse(&code, &"").unwrap();
    assert_eq!(sdc.commands[0], command);
}

#[test]
fn comment() {
    success("#comment");
    success("#comment\n#comment\n\n");
}

#[test]
fn header() {
    check_header(
        r##"#--------------
# header
#--------------
set sdc_version 2.1

# comment
"##,
        vec![
            "#--------------\n".to_string(),
            "# header\n".to_string(),
            "#--------------\n".to_string(),
        ],
    );
}

#[test]
fn version() {
    check_version("set sdc_version 1.1", Some(SdcVersion::SDC1_1));
    check_version("set sdc_version 1.2", Some(SdcVersion::SDC1_2));
    check_version("set sdc_version 1.3", Some(SdcVersion::SDC1_3));
    check_version("set sdc_version 1.4", Some(SdcVersion::SDC1_4));
    check_version("set sdc_version 1.5", Some(SdcVersion::SDC1_5));
    check_version("set sdc_version 1.6", Some(SdcVersion::SDC1_6));
    check_version("set sdc_version 1.7", Some(SdcVersion::SDC1_7));
    check_version("set sdc_version 1.8", Some(SdcVersion::SDC1_8));
    check_version("set sdc_version 1.9", Some(SdcVersion::SDC1_9));
    check_version("set sdc_version 2.0", Some(SdcVersion::SDC2_0));
    check_version("set sdc_version 2.1", Some(SdcVersion::SDC2_1));
}

#[test]
fn command() {
    check_command(
        "current_instance A",
        Command::CurrentInstance(CurrentInstance {
            instance: Some("A".into()),
        }),
    );
    check_command(
        "expr A B C",
        Command::Expr(Expr {
            args: vec!["A".into(), "B".into(), "C".into()],
        }),
    );
    check_command(
        "list A B C",
        Command::List(List {
            args: vec!["A".into(), "B".into(), "C".into()],
        }),
    );
    check_command(
        "set A B",
        Command::Set(Set {
            variable_name: "A".into(),
            value: "B".into(),
        }),
    );
    check_command(
        "set_hierarchy_separator A",
        Command::SetHierarchySeparator(SetHierarchySeparator {
            separator: "A".into(),
        }),
    );
    check_command(
        "set_units -capacitance A -resistance B -time C -voltage D -current E -power F",
        Command::SetUnits(SetUnits {
            capacitance: Some("A".into()),
            resistance: Some("B".into()),
            time: Some("C".into()),
            voltage: Some("D".into()),
            current: Some("E".into()),
            power: Some("F".into()),
        }),
    );
    check_command("all_clocks", Command::AllClocks(AllClocks));
    check_command(
        "all_inputs -level_sensitive -edge_triggered -clock A",
        Command::AllInputs(AllInputs {
            level_sensitive: true,
            edge_triggered: true,
            clock: Some("A".into()),
        }),
    );
    check_command(
        "all_outputs -level_sensitive -edge_triggered -clock A",
        Command::AllOutputs(AllOutputs {
            level_sensitive: true,
            edge_triggered: true,
            clock: Some("A".into()),
        }),
    );
}
