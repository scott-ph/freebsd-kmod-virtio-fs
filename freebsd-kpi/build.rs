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

extern crate bindgen;
extern crate cc;
use std::env;
use std::path;
use std::process;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let gen_result = process::Command::new("sh")
        .arg("-c")
        .arg(
            &[
                "echo '.include <bsd.kmod.mk>' | make -f - ",
                "bus_if.h device_if.h virtio_if.h virtio_bus_if.h ",
                "beforebuild",
            ]
            .join(""),
        )
        .env("MAKEOBJDIR", &out_dir)
        .status()
        .expect("failed to generate headers");
    assert!(gen_result.success());

    let cflags = String::from_utf8(
        process::Command::new("sh")
            .arg("-c")
            .arg("echo '.include <bsd.kmod.mk>' | make -f - -V CFLAGS")
            .env("MAKEOBJDIR", &out_dir)
            .output()
            .expect("failed to get CFLAGS")
            .stdout,
    )
    .unwrap();

    let mut libkpi_a = cc::Build::new();
    libkpi_a.file("src/kpi.c")
        .include(&out_dir)
        .flag("-flto=thin");

    for flag in cflags.split_whitespace() {
        libkpi_a.flag(flag);
    }

    libkpi_a
        .flag("-Wno-missing-prototypes")
        .flag("-Wno-sign-compare")
        .compile("libkpi.a");

    let mut bindings = bindgen::Builder::default();
    bindings = bindings
        .use_core()
        .ctypes_prefix("types")
        .header("src/kpi.c")
        .clang_arg(&["-I", &out_dir].join(""));

    for flag in cflags.split_whitespace() {
        bindings = bindings.clang_arg(flag);
    }

    bindings
        .clang_arg("-Wno-missing-prototypes")
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(path::PathBuf::from(out_dir).join("bindings.rs"))
        .expect("Failed to write bindings");
}
