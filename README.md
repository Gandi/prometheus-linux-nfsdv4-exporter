# prometheus-linux-nfsdv4-exporter - prometheus exporter for NFSv4 server. 

<b>prometheus-linux-nfsdv4-exporter</b> It is a prometheus-exporter that reports basic metrics of NFSv4 clients from the server side. Since Linux Kernel 5.3, NFSv4 mount points have information reported inside the /proc/fs/nfsd/clients/, it is possible to retrieve some limited information of clients that are using NFSv4. 

### Prometheus metrics:
* nfs_iobytes_read Total of bytes read
* nfs_iobytes_write Total of bytes write
* nfs_network_connections Total amount of network connections
* nfs_network_netcount Total amount of packets
* nfs_network_tcpcount Total amount of TCP packets
* nfs_network_udpcount Total amount of UDP packets
* nfs_reply_cache_hits Number of cache hits
* nfs_reply_cache_misses Number of cache misses
* nfs_reply_cache_nocache Number of nocache
* <b>nfsv4_op_deleg_per_client Number of deleg operations per NFSv4 client</b>
* <b>nfsv4_op_layout_per_client Number of layout operations per NFSv4 client</b>
* <b>nfsv4_op_lock_per_client Number of lock operations per NFSv4 client</b>
* <b>nfsv4_op_open_per_client Number of open operations per NFSv4 client</b>
* <b>number_of_nfsv4_clients Number of NFSv4 clients</b>
* <b>nfsv4_exports_total Number of NFSv4 exports</b>
 
### Build the project:
* Release: <b>```cargo build --release```</b>

### Debian package:
* First install: <b>```cargo install cargo-deb```</b>
* Generate the debian package: <b>```cargo deb -v```</b>

### Crate:
[https://crates.io/crates/prometheus-linux-nfsdv4-exporter](https://crates.io/crates/prometheus-linux-nfsdv4-exporter)

### Contributing:
<a href="https://github.com/Gandi/prometheus-linux-nfsdv4-exporter/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=Gandi/prometheus-linux-nfsdv4-exporter" />
</a>

### License:

The project is made available under the BSD 2-Clause license. See the `LICENSE` file for more information.
