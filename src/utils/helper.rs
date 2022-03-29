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
extern crate libc;
extern crate nix;
extern crate semver;
use nix::sys::utsname::*;
use semver::Version;

use std::path::Path;

// NFS
pub const PROC_NFSDV4: &'static str = "/proc/fs/nfsd/";
pub const VAR_NFSDV4: &'static str = "/var/lib/nfs/";
pub const PROC_RPC: &'static str = "/proc/net/rpc/";

// Linux kernel
const LINUX_MINIMAL_VERSION: &'static str = "5.3.0";

// Open ReadOnly Unix mode
pub const OPEN_READONLY: i32 = libc::O_RDONLY | libc::O_NONBLOCK | libc::O_NDELAY | libc::O_NOATIME | libc::O_CLOEXEC;

// Check variable type (used during development only)
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[inline]
pub fn is_kernel_compatible() -> bool {
    let sysinfo = uname();
    let _kernel_version: Vec<&str> = sysinfo.release().split('-').collect();
    let kernel_version = _kernel_version[0];

    if Version::parse(kernel_version).unwrap() >= Version::parse(LINUX_MINIMAL_VERSION).unwrap() {
        return true;
    }
    false
}

#[inline]
pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}
