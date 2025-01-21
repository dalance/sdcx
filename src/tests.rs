use crate::sdc::*;
use crate::Parser;
use std::fs::File;
use std::io::Read;

fn check_parse(code: &str) {
    let code = format!("{code}\n");
    let sdc = Parser::parse(&code, &"");
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

fn check_format(code: &str, format: &str) {
    let code = format!("{code}\n");
    let sdc = Parser::parse(&code, &"").unwrap();
    assert_eq!(&format!("{}", sdc.commands[0]), format);
}

fn check_testcase(path: &str, validatable: bool) {
    let mut f = File::open(path).unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);
    let sdc = Parser::parse(&code, &"");
    //dbg!(&sdc);
    assert!(sdc.is_ok());
    if validatable {
        assert!(sdc.unwrap().validate(None).is_empty())
    } else {
        assert!(!sdc.unwrap().validate(None).is_empty())
    }
}

#[test]
fn comment() {
    check_parse("#comment");
    check_parse("#comment\n#comment\n\n");
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
fn format() {
    check_format("all_clocks", "all_clocks");
    check_format(
        "all_inputs -clock A -level_sensitive",
        "all_inputs -level_sensitive -clock A",
    );
    check_format(
        "all_outputs -clock A -edge_triggered",
        "all_outputs -edge_triggered -clock A",
    );
    check_format(
        "all_registers -clock A -edge_triggered",
        "all_registers -clock A -edge_triggered",
    );
    check_format("current_instance   A  ", "current_instance A");
    check_format("expr A B C ", "expr A B C");
    check_format("list A B C ", "list A B C");
    check_format("set A B", "set A B");
    check_format("set A B", "set A B");
    check_format("get_pins   a[10]", "get_pins a[10]");
    check_format("get_pins   a[0x10]", "get_pins a[0x10]");
    check_format("get_pins   a[*]", "get_pins a[*]");
    check_format("get_pins   a[3:2]", "get_pins a[3:2]");
}

#[test]
fn testcase() {
    check_testcase("testcase/cdctl.sdc", false);
    check_testcase("testcase/ctu_can_fd.sdc", true);
    check_testcase("testcase/logic_clock_domain_crossing.sdc", false);
    check_testcase("testcase/peridot_cam.sdc", false);
    check_testcase("testcase/peridot_hostbridge.sdc", false);
    check_testcase("testcase/peridot_wsg.sdc", false);
    check_testcase("testcase/scan.sdc", true);
    // TODO linebreak of the end of file is missing
    //check_testcase("testcase/signoff.sdc", true);
    check_testcase("testcase/soc_system_hdmi_i2c_only.sdc", true);
    check_testcase("testcase/soc_system_mandelbrot_timing.sdc", false);
    check_testcase("testcase/spi_slave_mm.sdc", false);
    check_testcase("testcase/tiles_base.sdc", true);
    check_testcase("testcase/timing_constraints.sdc", true);
    check_testcase("testcase/tinyODIN.sdc", true);
}
