use crate::sdc::{Argument, SdcError, SdcVersion};

pub(crate) fn opt_arg(
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

pub(crate) fn opt_flg(name: Argument, tgt: bool) -> Result<bool, SdcError> {
    match tgt {
        true => Err(SdcError::DuplicatedArgument(name)),
        false => Ok(true),
    }
}

pub(crate) fn vec_arg(
    name: Argument,
    arg: Option<Argument>,
    mut tgt: Vec<Argument>,
) -> Result<Vec<Argument>, SdcError> {
    if let Some(arg) = arg {
        tgt.push(arg);
        Ok(tgt)
    } else {
        Err(SdcError::MissingOptArgument(name))
    }
}

pub(crate) fn pos_args1(
    arg: Option<Argument>,
    tgt: Option<Argument>,
) -> Result<Option<Argument>, SdcError> {
    if arg.is_none() {
        return Err(SdcError::MissingPosArgument);
    }
    match tgt {
        Some(_) => Err(SdcError::TooManyArgument),
        None => Ok(arg),
    }
}

pub(crate) fn pos_args2(
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

pub(crate) fn mandatory(arg: Option<Argument>, name: &str) -> Result<Argument, SdcError> {
    arg.ok_or(SdcError::MissingMandatoryArgument(name.into()))
}

pub(crate) fn fmt_arg(x: &Argument) -> String {
    format!(" {}", x)
}

pub(crate) fn fmt_opt_arg(x: &Option<Argument>) -> String {
    if let Some(x) = x {
        format!(" {}", x)
    } else {
        "".into()
    }
}

pub(crate) fn fmt_named_arg(x: &Argument, name: &str) -> String {
    format!(" -{} {}", name, x)
}

pub(crate) fn fmt_named_opt_arg(x: &Option<Argument>, name: &str) -> String {
    if let Some(x) = x {
        format!(" -{} {}", name, x)
    } else {
        "".into()
    }
}

pub(crate) fn fmt_named_vec_arg(x: &Vec<Argument>, name: &str) -> String {
    let mut ret = "".to_string();
    for x in x {
        ret.push_str(&format!(" -{} {}", name, x))
    }
    ret
}

pub(crate) fn fmt_named_flg(x: bool, name: &str) -> String {
    if x {
        format!(" -{}", name)
    } else {
        "".into()
    }
}

pub(crate) fn minimum_supported_version(tgt: SdcVersion, msv: SdcVersion) -> bool {
    tgt >= msv
}

pub(crate) trait Exist {
    fn exist(&self) -> bool;
}

impl Exist for bool {
    fn exist(&self) -> bool {
        *self
    }
}

impl<T> Exist for Option<T> {
    fn exist(&self) -> bool {
        self.is_some()
    }
}

impl<T> Exist for Vec<T> {
    fn exist(&self) -> bool {
        !self.is_empty()
    }
}

pub(crate) fn check2<A: Exist, B: Exist, T: Fn(bool, bool) -> bool>(a: &A, b: &B, func: T) -> bool {
    func(a.exist(), b.exist())
}

pub(crate) fn check3<A: Exist, B: Exist, C: Exist, T: Fn(bool, bool, bool) -> bool>(
    a: &A,
    b: &B,
    c: &C,
    func: T,
) -> bool {
    func(a.exist(), b.exist(), c.exist())
}

pub(crate) fn check4<
    A: Exist,
    B: Exist,
    C: Exist,
    D: Exist,
    T: Fn(bool, bool, bool, bool) -> bool,
>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    func: T,
) -> bool {
    func(a.exist(), b.exist(), c.exist(), d.exist())
}

pub(crate) fn check5<
    A: Exist,
    B: Exist,
    C: Exist,
    D: Exist,
    E: Exist,
    T: Fn(bool, bool, bool, bool, bool) -> bool,
>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    func: T,
) -> bool {
    func(a.exist(), b.exist(), c.exist(), d.exist(), e.exist())
}

pub(crate) fn check6<
    A: Exist,
    B: Exist,
    C: Exist,
    D: Exist,
    E: Exist,
    F: Exist,
    T: Fn(bool, bool, bool, bool, bool, bool) -> bool,
>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    func: T,
) -> bool {
    func(
        a.exist(),
        b.exist(),
        c.exist(),
        d.exist(),
        e.exist(),
        f.exist(),
    )
}

pub(crate) fn check7<
    A: Exist,
    B: Exist,
    C: Exist,
    D: Exist,
    E: Exist,
    F: Exist,
    G: Exist,
    T: Fn(bool, bool, bool, bool, bool, bool, bool) -> bool,
>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    g: &G,
    func: T,
) -> bool {
    func(
        a.exist(),
        b.exist(),
        c.exist(),
        d.exist(),
        e.exist(),
        f.exist(),
        g.exist(),
    )
}
