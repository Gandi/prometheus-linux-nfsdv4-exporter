/*-
 * SPDX-License-Identifier: BSD-2-Clause
 *
 * BSD 2-Clause License
 *
 * Copyright (c) 2021-2023, Gandi S.A.S.
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

use crate::utils::helper::{path_exists, wrapper_read, PROC_NFSDV4, VAR_NFSDV4};
use std::fs::read_dir;

#[derive(Debug)]
pub struct Nfsv4Client {
    pub clientid: String,
    pub address: String,
    pub ops_count: Nfsv4ClientOps,
}

#[derive(Debug)]
pub struct Nfsv4ClientOps {
    pub t_open: i64,
    pub t_lock: i64,
    pub t_deleg: i64,
    pub t_layout: i64,
}

pub fn number_of_clients() -> i64 {
    let mut _proc_nfsdv4 = PROC_NFSDV4.to_owned();
    _proc_nfsdv4.push_str("clients/");
    if path_exists(&_proc_nfsdv4) {
        let paths = read_dir(&_proc_nfsdv4).unwrap();
        return paths.count() as i64;
    }
    0
}

pub fn number_of_exports() -> i64 {
    let etab = VAR_NFSDV4.to_owned() + "/etab";
    if path_exists(&etab) {
        let content = wrapper_read(etab);
        return content.len() as i64;
    }
    0
}

fn clients_ops_information(path: &str) -> Nfsv4ClientOps {
    let clt_states = path.to_owned() + "/states";
    let (mut t_open, mut t_lock, mut t_deleg, mut t_layout): (i64, i64, i64, i64) = (0, 0, 0, 0);

    if path_exists(&clt_states) {
        let content = wrapper_read(clt_states);

        for line in content.iter() {
            if line.contains("type: open") {
                t_open += 1;
            } else if line.contains("type: lock") {
                t_lock += 1;
            } else if line.contains("type: deleg") {
                t_deleg += 1;
            } else if line.contains("type: layout") {
                t_layout += 1;
            }
        }
    }

    let nfsv4_client_ops = Nfsv4ClientOps {
        t_open: t_open,
        t_lock: t_lock,
        t_deleg: t_deleg,
        t_layout: t_layout,
    };

    nfsv4_client_ops
}

pub fn clients_information() -> Vec<Nfsv4Client> {
    let mut nfsv4_client: Vec<Nfsv4Client> = Vec::new();
    let mut _proc_nfsdv4_clients = PROC_NFSDV4.to_owned();

    _proc_nfsdv4_clients.push_str("clients/");
    if path_exists(&_proc_nfsdv4_clients) {
        let paths = read_dir(&_proc_nfsdv4_clients).unwrap();
        for path in paths {
            let mut clientid = String::new();
            let mut address = String::new();
            let _path = path.unwrap().path();
            let info = _path.to_str().unwrap().to_owned() + "/info";

            let content = wrapper_read(info);

            for line in content.iter() {
                let mut line = line.to_owned();
                if line.contains("clientid") {
                    clientid = line.replace("clientid:", "").trim().to_string();
                }
                if line.contains("address") && !line.contains("callback") {
                    line.retain(|x| !['\"'].contains(&x));
                    address = line.replace("address:", "").trim().to_string();
                }
            }
            nfsv4_client.push(Nfsv4Client {
                clientid: clientid,
                address: address,
                ops_count: clients_ops_information(&_path.to_str().unwrap()),
            });
        }
    }

    nfsv4_client
}
