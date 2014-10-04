#![feature(macro_rules)]
#![crate_name = "pathchk"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Benjamin Bergman <ben@benbergman.ca>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate getopts;

use std::io::{print};

#[path = "../common/util.rs"]
mod util;

static NAME: &'static str = "pathchk";
static VERSION: &'static str = "1.0.0";

pub fn uumain(args: Vec<String>) -> int  {
    let program = args[0].as_slice();
    let opts = [
        getopts::optflag("p", "", "check for most POSIX systems"),
        getopts::optflag("P", "", "check for empty names and leading \"-\""),
        getopts::optflag("", "portability", "check for all POSIX systems (equivalent to -p -P)"),
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!("Invalid options\n{}", f)
    };

    if matches.opt_present("help") {
        println!("Usage:");
        println!("  {0:s} [OPTION]... NAME...", program);
        println!("");
        print(getopts::usage("Diagnose invalid or unportable file names.", opts).as_slice());
        return 0
    }

    if matches.opt_present("version") {
        println!("{} {}", program, VERSION);
        return 0;
    }

    let mut check_basic_portability = false;
    if matches.opt_present("p") {
        check_basic_portability = true;
    }

    let mut check_extra_portability = false;
    if matches.opt_present("p") {
        check_extra_portability = true;
    }

    if matches.opt_present("portability") {
        check_basic_portability = true;
        check_extra_portability = true;
    }

    let names = matches.free;
    if names.is_empty() {
        crash!(1, "missing operand");
    }

    let mut okay = 0;
    for name in names.iter() {
        if ! validate_file_name(name, check_basic_portability, check_extra_portability) {
            okay = 1;
        }
    }

    okay
}


fn validate_file_name(name: &String, check_basic_portability: bool, check_extra_portability: bool) -> bool {
    if check_extra_portability && leading_hyphen(name.as_slice()) {
        return false;
    }

    if (check_extra_portability || check_basic_portability) && name.len() == 0 {
        println!("{}: empty file name", NAME);
        return false;
    }

    if check_basic_portability {
        if ! portable_chars_only(name.as_slice()) {
            return false;
        }
    }

    true
}


fn leading_hyphen(name: &str) -> bool {
    for (i, c) in name.char_indices() {
        if c == '-' && (i == 0 || name.char_at(i - 1) == '/') {
            println!("{}: leading '-' in a component of file name '{}'", NAME, name);
            return true;
        }
    }
    false
}


fn portable_chars_only(name: &str) -> bool {
    let approved_chars =  "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ/._-";
    for c in name.chars() {
        if ! approved_chars.contains_char(c) {
            println!("{}: nonportable character '{}' in file name '{}'", NAME, c, name);
            return false;
        }
    }
    true
}


#[test]
fn test_leading_hyphen() {
    assert!(leading_hyphen("aoue/-snth"));
    assert!(leading_hyphen("-aoue/snth"));
    assert!(!leading_hyphen("aoue/snth"));
}


#[test]
fn test_portable_chars_only() {
    assert!(portable_chars_only("aoesunth"));
    assert!(portable_chars_only("nt/923874/._-"));
    assert!(!portable_chars_only(")(*^%$*&^$)"));
}
