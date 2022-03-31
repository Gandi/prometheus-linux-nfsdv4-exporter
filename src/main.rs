/*-
 * SPDX-License-Identifier: BSD-2-Clause
 *
 * BSD 2-Clause License
 *
 * Copyright (c) 2021, Gandi S.A.S.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#[forbid(unsafe_code)]
#[macro_use]
extern crate lazy_static;

use clap::{App, Arg, SubCommand};

mod nfs;
mod utils;
use crate::utils::helper::{is_kernel_compatible};

mod prometheus;
use crate::prometheus::exporter::{start_prometheus};

const VERSION: &str = "1.0.1";

fn help() {
     println!("Use command with help option");
}

fn main() {
    if !is_kernel_compatible() {
        std::process::exit(69);
    }

    let matches = App::new("prometheus-linux-nfsdv4-exporter")
        .version(VERSION)
        .author("\nAuthor: Marcelo Araujo <marcelo.araujo@gandi.net>")
        .about("prometheus nfsv4 exporter")
        .subcommand(
            SubCommand::with_name("set")
                .about("Set parameters")
                .arg(
                    Arg::with_name("ip-address")
                        .short("ip")
                        .long("ip-address")
                        .required(false)
                        .value_name("IPADDRESS")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .required(false)
                        .value_name("PORT")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(m)) => start_prometheus(m),
        _ => Ok(help()),
    };
}
