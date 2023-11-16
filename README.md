# SDCx

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

## Check

`sdcx check` validates an input SDC file.
By default, `sdcx` uses SDC version according to `set sdc_version` command.
If version is not specified, SDC 2.1 will be assumed.
If `--force-version` option is provided, the version can be overrided.

```
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

# Library

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
