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

#include <sys/param.h>
#include <sys/types.h>
#include <sys/bus.h>
#include <sys/kernel.h>
#include <sys/module.h>
#include <sys/mount.h>

#include <dev/virtio/virtio.h>

extern int vtfs_probe(device_t);
extern int vtfs_attach(device_t);
extern int vtfs_detach(device_t);
extern int vtfs_modevent(module_t, int, void *);

static device_method_t vtfs_methods[] = {
	DEVMETHOD(device_probe,		vtfs_probe),
	DEVMETHOD(device_attach,	vtfs_attach),
	DEVMETHOD(device_detach,	vtfs_detach),

	DEVMETHOD_END
};

static driver_t vtfs_driver = {
	.name = "vtfs",
	.methods = vtfs_methods,
};
static devclass_t vtfs_devclass;

DRIVER_MODULE(virtio_fs, virtio_pci, vtfs_driver, vtfs_devclass, vtfs_modevent,
    0);
MODULE_VERSION(virtio_fs, 1);
MODULE_DEPEND(virtio_fs, virtio, 1, 1, 1);

#if __FreeBSD_version > 1300031
VIRTIO_SIMPLE_PNPTABLE(virtio_fs, VIRTIO_ID_FS, "VirtIO FS Adapter");
VIRTIO_SIMPLE_PNPINFO(virtio_pci, virtio_fs);
#endif

int vtfs_mount(struct mount *mp);
int vtfs_unmount(struct mount *mp, int mntflags);
int vtfs_root(struct mount *mp, int flags, struct vnode **vpp);
int vtfs_quotactl(struct mount *mp, int cmds, uid_t uid, void *arg);
int vtfs_statfs(struct mount *mp, struct statfs *sbp);
int vtfs_sync(struct mount *mp, int waitfor);
int vtfs_vget(struct mount *mp, ino_t ino, int flags, struct vnode **vpp);
int vtfs_fhtovp(struct mount *mp, struct fid *fhp, int flags,
    struct vnode **vpp);
int vtfs_checkexp(struct mount *mp, struct sockaddr *nam, int *extflagsp,
    struct ucred **credanonp, int *numsecflavors, int **secflavors);
int vtfs_init(struct vfsconf *);
int vtfs_uninit(struct vfsconf *);
int vtfs_extattrctl(struct mount *mp, int cmd, struct vnode *filename_vp,
    int attrnamespace, const char *attrname);

struct vfsops vtfs_vfsops = {
	.vfs_mount =		vtfs_mount,
	.vfs_unmount =		vtfs_unmount,
	.vfs_root =		vtfs_root,
	.vfs_quotactl =		vtfs_quotactl,
	.vfs_statfs =		vtfs_statfs,
	.vfs_sync =		vtfs_sync,
	.vfs_vget =		vtfs_vget,
	.vfs_fhtovp =		vtfs_fhtovp,
	.vfs_checkexp =		vtfs_checkexp,
	.vfs_init =		vtfs_init,
	.vfs_uninit =		vtfs_uninit,
	.vfs_extattrctl =	vtfs_extattrctl
};

VFS_SET(vtfs_vfsops, vtfs, VFCF_JAIL);
