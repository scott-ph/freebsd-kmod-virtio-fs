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

use freebsd_kpi::types::*;
use freebsd_kpi::*;

#[no_mangle]
pub extern fn vtfs_mount(_mp: *mut mount) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_unmount(_mp: *mut mount, _mntflags: c_int) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_root(_mp: *mut mount, _flags: c_int, _vpp: *mut *mut vnode) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_quotactl(
    _mp: *mut mount,
    _cmds: c_int,
    _uid: uid_t,
    _arg: *mut c_void,
) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_statfs(_mp: *mut mount, _sbp: *mut statfs) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_sync(_mp: *mut mount, _waitfor: c_int) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_vget(
    _mp: *mut mount,
    _ino: ino_t,
    _flags: c_int,
    _vpp: *mut *mut vnode,
) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_fhtovp(
    _mp: *mut mount,
    _fhp: *mut fid,
    _flags: c_int,
    _vpp: *mut *mut vnode,
) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_checkexp(
    _mp: *mut mount,
    _nam: *mut sockaddr,
    _extflagsp: *mut c_int,
    _credanonp: *mut *mut ucred,
    _numsecflavors: *mut c_int,
    _secflavors: *mut *mut c_int,
) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_init(_conf: *mut vfsconf) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_uninit(_conf: *mut vfsconf) -> c_int {
    EOPNOTSUPP as c_int
}

#[no_mangle]
pub extern fn vtfs_extattrctl(
    _mp: *mut mount,
    _cmd: c_int,
    _filename_vp: *mut vnode,
    _attrnamespace: c_int,
    _attrname: *const c_char,
) -> c_int {
    EOPNOTSUPP as c_int
}
