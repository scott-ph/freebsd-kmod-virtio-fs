/*-
 * Copyright (c) 2020 D Scott Phillips <scott@scott.ph>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 */

#![no_std]

extern crate freebsd_kpi;
use freebsd_kpi::types::*;
use freebsd_kpi::*;

mod vfs;

#[no_mangle]
pub unsafe extern fn vtfs_modevent(
    _module: module_t,
    evtype: modeventtype,
    _data: *mut c_void,
) -> c_int {
    #[allow(non_upper_case_globals)]
    let msg = match evtype {
        modeventtype_MOD_LOAD => "MOD_LOAD\0",
        modeventtype_MOD_UNLOAD => "MOD_UNLOAD\0",
        modeventtype_MOD_SHUTDOWN => "MOD_SHUTDOWN\0",
        modeventtype_MOD_QUIESCE => "MOD_QUIESCE\0",
        _ => "MOD_UNKNOWN\0",
    };
    printf("vtfs_modevent %s\n\0".as_ptr() as *const i8, msg.as_ptr());
    0
}

#[no_mangle]
pub unsafe extern fn vtfs_probe(_dev: device_t) -> c_int {
    printf("vtfs_probe\n\0".as_ptr() as *const i8);
    0
}

#[no_mangle]
pub unsafe extern fn vtfs_attach(_dev: device_t) -> c_int {
    printf("vtfs_attach\n\0".as_ptr() as *const i8);
    0
}

#[no_mangle]
pub unsafe extern fn vtfs_detach(_dev: device_t) -> c_int {
    printf("vtfs_detach\n\0".as_ptr() as *const i8);
    0
}
