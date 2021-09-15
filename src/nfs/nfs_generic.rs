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

use crate::utils::helper::{path_exists, PROC_RPC};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
pub struct ReplyCache {
    // client did not receive a reply, do a re-transmit request and
    // the reply was cached.
    pub hits: i64,
    // operation that requires caching.
    pub misses: i64,
    // non-idempotent operation like (rename/delete).
    pub nocache: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct IOBytes {
    // total amount of bytes read since the last restart.
    pub read: i64,
    // total amount of bytes write since the last restart.
    pub write: i64,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy)]
pub struct NetworkUsage {
    // total amount of packets.
    pub netcount: i64,
    // total amount of UDP packets.
    pub UDPcount: i64,
    // total amount of TCP packets.
    pub TCPcount: i64,
    // total amount of TCP connections.
    pub TCPconnect: i64,
}

#[derive(Debug)]
pub struct NfsStats {
    pub reply_cache: ReplyCache,
    pub io_bytes: IOBytes,
    pub network_usage: NetworkUsage,
}

fn reply_cache(data: Vec<&str>) -> ReplyCache {
    let (mut hits, mut misses, mut nocache): (i64, i64, i64) = (0, 0, 0);
    if data.len() >= 4 {
        hits = data[1].parse::<i64>().unwrap();
        misses = data[2].parse::<i64>().unwrap();
        nocache = data[3].parse::<i64>().unwrap();
    }

    let reply_cache = ReplyCache {
        hits: hits,
        misses: misses,
        nocache: nocache,
    };

    reply_cache
}

fn io_bytes(data: Vec<&str>) -> IOBytes {
    let (mut read, mut write): (i64, i64) = (0, 0);
    if data.len() >= 3 {
        read = data[1].parse::<i64>().unwrap();
        write = data[2].parse::<i64>().unwrap();
    }

    let io_bytes = IOBytes {
        read: read,
        write: write,
    };

    io_bytes
}

#[allow(non_snake_case)]
fn network_usage(data: Vec<&str>) -> NetworkUsage {
    let (mut netcount, mut UDPcount, mut TCPcount, mut TCPconnect): (i64, i64, i64, i64) = (0, 0, 0, 0);
    if data.len() >= 5 {
        netcount = data[1].parse::<i64>().unwrap();
        UDPcount = data[2].parse::<i64>().unwrap();
        TCPcount = data[3].parse::<i64>().unwrap();
        TCPconnect = data[4].parse::<i64>().unwrap();
    }

    let network_usage = NetworkUsage {
        netcount: netcount,
        UDPcount: UDPcount,
        TCPcount: TCPcount,
        TCPconnect: TCPconnect,
    };

    network_usage
}

pub fn rpc_nfsd_metrics() -> NfsStats {
    let mut reply_cache_s = ReplyCache {hits: 0, misses: 0, nocache: 0};
    let mut io_bytes_s = IOBytes {read: 0, write: 0};
    let mut network_usage_s = NetworkUsage {netcount: 0, UDPcount: 0, TCPcount: 0, TCPconnect: 0};
    let mut nfs_stats = NfsStats {reply_cache: reply_cache_s, io_bytes: io_bytes_s, network_usage: network_usage_s};

    let mut _proc_rpc_nfsd = PROC_RPC.to_owned();
    _proc_rpc_nfsd.push_str("nfsd");

    if path_exists(&_proc_rpc_nfsd) {
        let open_nfsd = File::open(_proc_rpc_nfsd).expect("file not found");
        let reader = BufReader::new(open_nfsd);

        for line in reader.lines() {
            let line = line.unwrap();
            if line.contains("rc") {
                let rc_data: Vec<&str> = line.split(' ').collect();
                reply_cache_s = reply_cache(rc_data);
            }
            if line.contains("io") {
                let io_data: Vec<&str> = line.split(' ').collect();
                io_bytes_s = io_bytes(io_data);
            }
            if line.contains("net") {
                let net_data: Vec<&str> = line.split(' ').collect();
                network_usage_s = network_usage(net_data);
            }
        }
        nfs_stats = NfsStats {
            reply_cache: reply_cache_s,
            io_bytes: io_bytes_s,
            network_usage: network_usage_s,
        };
    }

    nfs_stats
}
