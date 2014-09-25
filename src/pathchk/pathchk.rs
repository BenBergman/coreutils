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
    0
}
