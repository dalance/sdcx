# SDCx

[![Actions Status](https://github.com/dalance/sdcx/workflows/Regression/badge.svg)](https://github.com/dalance/sdcx/actions)
[![Crates.io](https://img.shields.io/crates/v/sdcx.svg)](https://crates.io/crates/sdcx)

**SDCx** is SDC (Synopsys Design Constraints) toolkit.
It provides a cli tool to handle SDC files and a SDC parsing library of Rust.

# About SDC

SDC (Synopsys Design Constraints) is a de facto standard format to describe constraints for ASIC/FPGA design.
The specification of SDC is published through [Technology Access Program (TAP-in)](https://www.synopsys.com/community/interoperability-programs/tap-in.html).
SDCx supports SDC1.1 ~ SDC2.1 defined at the specification.

# CLI tool

`sdcx` is a CLI frontend of SDCx. This provides the following features.

- Check : Validate an input SDC file.
- Format: Format an input SDC file.

`sdcx` supports gzip-ed sdc file. If the extension of the specified file name is `.gz`, it is treated as gzip-ed file.

## Check

`sdcx check` validates an input SDC file.
By default, `sdcx` uses SDC version according to `set sdc_version` command.
If version is not specified, SDC 2.1 will be assumed.
If `--force-version` option is provided, the version can be overrided.

```console
$ cat test.sdc
set sdc_version 2.1
set_sense -positive pin; # set_sense is supported on SDC2.1 only
$ sdcx check test.sdc
$ sdcx check --force-version 2.0 test.sdc
error[sdcx::sdc::SdcError]: Unsupported command at SDC 2.0
  ┌─ test.sdc:2:1
  │
2 │ set_sense -positive pin; # set_sense is supported on SDC2.1 only
  │ ^^^^^^^^^^^^^^^^^^^^^^^ Found
```

## Format

`sdcx fmt` formats an input SDC file.
If `--output` option is provided, the result will be written to the specified file.
Otherwise it will be outputted to stdout.

The formatted result will be:

- Commands are sorted by alphabet-order.
- Arguments of each command are ordered by the pre-defined order.
- All unnecessary whitespaces are removed.
- All comments except file header are removed.

Some EDA tools output randomly ordered commands in SDC file.
In the case, formatting can be used to get the stable result.
Additionally, the differences between EDA tool vendors can be removed too.

```console
$ cat test.sdc
set_units    -time    ns    -capaticance    pF
create_clock  -name    CLK    -period    10
$ sdcx fmt test.sdc
create_clock -period 10 -name CLK
set_units -capacitance pF -time ns
```

# Library

[![Crates.io](https://img.shields.io/crates/v/sdcx.svg)](https://crates.io/crates/sdcx)
[![Docs.rs](https://docs.rs/sdcx/badge.svg)](https://docs.rs/sdcx)

## Usage

```console
$ cargo add sdcx
```

## Example

```rust
    let code = format!("create_clock -period 10 -name CLK\n");
    let sdc = sdcx::Parser::parse(&code, &"");
    dbg!(sdc);
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
