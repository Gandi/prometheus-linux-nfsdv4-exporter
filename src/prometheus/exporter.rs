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

use clap::ArgMatches;
use prometheus::{
    IntGauge, IntGaugeVec, Opts, Registry,
};
use std::net::SocketAddr;
use std::result::Result;
use std::sync::Mutex;

use warp::{Filter, Rejection, Reply};

use crate::nfs::nfs_generic::rpc_nfsd_metrics;
use crate::nfs::nfsv4::{clients_information, number_of_clients, number_of_exports};

// Enable or Disable metrics, disable by default nfsv4 ops per clients
#[derive(Debug, Clone)]
pub struct ExporterOptions {
    nfsv4_ops_clients: bool,
}

lazy_static! {
    // Export options
    static ref EXPORTEROPTS: Mutex<ExporterOptions> = Mutex::new(ExporterOptions { nfsv4_ops_clients: false });

    pub static ref REGISTRY: Registry = Registry::new();
    // Number of clients connected
    pub static ref NUMBER_OF_NFSV4_CLIENTS: IntGauge =
        IntGauge::new("number_of_nfsv4_clients", "Number of NFSv4 clients")
            .expect("metric can be created");

    // Number of exports
    pub static ref NUMBER_OF_NFSV4_EXPORTS: IntGauge =
        IntGauge::new("nfsv4_exports_total", "Number of NFSv4 exports")
            .expect("metric can be created");

    // Number of FS OPS per client
    pub static ref OPEN_PER_NFSV4_CLIENT: IntGaugeVec =
        IntGaugeVec::new(Opts::new("nfsv4_op_open_per_client", "Number of open operations per NFSv4 client"),
        &["client"])
            .expect("metric can be created");
    pub static ref LOCK_PER_NFSV4_CLIENT: IntGaugeVec =
        IntGaugeVec::new(Opts::new("nfsv4_op_lock_per_client", "Number of lock operations per NFSv4 client"),
        &["client"])
            .expect("metric can be created");
    pub static ref DELEG_PER_NFSV4_CLIENT: IntGaugeVec =
        IntGaugeVec::new(Opts::new("nfsv4_op_deleg_per_client", "Number of deleg operations per NFSv4 client"),
        &["client"])
            .expect("metric can be created");
    pub static ref LAYOUT_PER_NFSV4_CLIENT: IntGaugeVec =
        IntGaugeVec::new(Opts::new("nfsv4_op_layout_per_client", "Number of layout operations per NFSv4 client"),
        &["client"])
            .expect("metric can be created");

    // Cache
    pub static ref REPLY_CACHE_HITS: IntGauge =
        IntGauge::new("nfs_reply_cache_hits", "Number of cache hits")
            .expect("metric can be created");
    pub static ref REPLY_CACHE_MISSES: IntGauge =
        IntGauge::new("nfs_reply_cache_misses", "Number of cache misses")
            .expect("metric can be created");
    pub static ref REPLY_CACHE_NOCACHE: IntGauge =
        IntGauge::new("nfs_reply_cache_nocache", "Number of nocache")
            .expect("metric can be created");

    // IOBytes read and write
    pub static ref IOBYTES_READ: IntGauge =
        IntGauge::new("nfs_iobytes_read", "Total of bytes read")
            .expect("metric can be created");
    pub static ref IOBYTES_WRITE: IntGauge =
        IntGauge::new("nfs_iobytes_write", "Total of bytes write")
            .expect("metric can be created");

    // Network
    pub static ref NETWORK_NETCOUNT: IntGauge =
        IntGauge::new("nfs_network_netcount", "Total amount of packets")
            .expect("metric can be created");
    pub static ref NETWORK_UDPCOUNT: IntGauge =
        IntGauge::new("nfs_network_udpcount", "Total amount of UDP packets")
            .expect("metric can be created");
    pub static ref NETWORK_TCPCOUNT: IntGauge =
        IntGauge::new("nfs_network_tcpcount", "Total amount of TCP packets")
            .expect("metric can be created");
    pub static ref NETWORK_CONNECTIONS: IntGauge =
        IntGauge::new("nfs_network_connections", "Total amount of network connections")
            .expect("metric can be created");
}

fn register_metrics() {
    REGISTRY.register(Box::new(NUMBER_OF_NFSV4_CLIENTS.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(NUMBER_OF_NFSV4_EXPORTS.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(OPEN_PER_NFSV4_CLIENT.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(LOCK_PER_NFSV4_CLIENT.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(DELEG_PER_NFSV4_CLIENT.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(LAYOUT_PER_NFSV4_CLIENT.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(REPLY_CACHE_HITS.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(REPLY_CACHE_MISSES.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(REPLY_CACHE_NOCACHE.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(IOBYTES_READ.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(IOBYTES_WRITE.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(NETWORK_NETCOUNT.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(NETWORK_UDPCOUNT.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(NETWORK_TCPCOUNT.clone()))
        .expect("collector can be registered");
    REGISTRY.register(Box::new(NETWORK_CONNECTIONS.clone()))
        .expect("collector can be registered");
}

// Index handler.
async fn index_handler() -> Result<impl Reply, Rejection> {
    Ok("")
}

async fn metrics_handler() -> Result<impl Reply, Rejection> {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let opts = EXPORTEROPTS.try_lock().unwrap().clone();

    // Number of clients connected.
    let number_of_clients = number_of_clients();
    NUMBER_OF_NFSV4_CLIENTS.set(number_of_clients);

    // Number of exports.
    let number_of_exports = number_of_exports();
    NUMBER_OF_NFSV4_EXPORTS.set(number_of_exports);

    // Number of NFSv4 ops per client.
    // It is disabled by default as it can be CPU intensive
    if opts.nfsv4_ops_clients {
    let ops_per_client = clients_information();
        for client in ops_per_client.iter() {
            OPEN_PER_NFSV4_CLIENT.with_label_values(&[&client.address])
                .set(client.ops_count.t_open);
            LOCK_PER_NFSV4_CLIENT.with_label_values(&[&client.address])
                .set(client.ops_count.t_lock);
            DELEG_PER_NFSV4_CLIENT.with_label_values(&[&client.address])
                .set(client.ops_count.t_deleg);
            LAYOUT_PER_NFSV4_CLIENT.with_label_values(&[&client.address])
                .set(client.ops_count.t_layout);
        }
    }

    // NFS Cache information.
    let nfs_stats = rpc_nfsd_metrics();
    REPLY_CACHE_HITS.set(nfs_stats.reply_cache.hits);
    REPLY_CACHE_MISSES.set(nfs_stats.reply_cache.misses);
    REPLY_CACHE_NOCACHE.set(nfs_stats.reply_cache.nocache);

    // IOBytes
    IOBYTES_READ.set(nfs_stats.io_bytes.read);
    IOBYTES_WRITE.set(nfs_stats.io_bytes.write);

    // Network
    NETWORK_NETCOUNT.set(nfs_stats.network_usage.netcount);
    NETWORK_UDPCOUNT.set(nfs_stats.network_usage.UDPcount);
    NETWORK_TCPCOUNT.set(nfs_stats.network_usage.TCPcount);
    NETWORK_CONNECTIONS.set(nfs_stats.network_usage.TCPconnect);

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };

    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8: {}", e);
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };
    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_custom);
    Ok(res)
}

#[tokio::main]
pub async fn start_prometheus(options: &ArgMatches) -> Result<(), ()> {
    let mut default_port = "9944";
    let mut default_address = "0.0.0.0";

    let expopts: ExporterOptions = ExporterOptions {
        nfsv4_ops_clients: options.is_present("nfsv4opsclients"),
     };

    // XXX: It is safe to use unwrap() here
    EXPORTEROPTS.try_lock().unwrap().clone_from(&expopts);

    if let Some(port) = options.value_of("port") {
        default_port = port;
    }

    if let Some(ip) = options.value_of("ip-address") {
        default_address = ip;
    }

    let addr: String = default_address.to_owned() + ":" + &default_port;
    let addr_convert: SocketAddr = addr.parse().expect("Could not parse SocketAddr");

    register_metrics();

    let metrics_route = warp::path!("metrics").and_then(metrics_handler);
    let route = warp::path::end().and_then(index_handler);

    println!("Exporter started on IP: {}, Port: {}", default_address, default_port);
    warp::serve(metrics_route.or(route))
        .run(addr_convert)
        .await;

    Ok(())
}
