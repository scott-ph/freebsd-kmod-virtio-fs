KMOD=		virtio_fs
SRCS=		virtio_fs.c device_if.h bus_if.h
RUSTLIB=	${.CURDIR}/target/x86_64-freebsd-kernel/release/libkmod_virtio_fs.a
OBJS=		${RUSTLIB}
CFLAGS+=	-flto=thin
CLEANDIRS+=	target

${RUSTLIB}: rust

.PHONY: rust
rust:
	env RUST_TARGET_PATH=${.CURDIR} xargo build --release --target x86_64-freebsd-kernel

.include <bsd.kmod.mk>
