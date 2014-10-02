use std::io::process::Command;
use std::io::process::ExitStatus;
use std::str;

static PROGNAME: &'static str = "./pathchk";


#[test]
fn test_argument_required() {
    let po = match Command::new(PROGNAME).output() {
        Ok(p) => p,
        Err(err) => fail!("{}", err),
    };

    let out = str::from_utf8(po.output.as_slice()).unwrap();
    let err = str::from_utf8(po.error.as_slice()).unwrap();
    let status = po.status;

    assert_eq!(out, "");
    assert_eq!(err, "pathchk: error: missing operand\n");
    assert_eq!(status, ExitStatus(1));
}
