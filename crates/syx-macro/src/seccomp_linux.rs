//! C-5c-2: Linux opt-in net-deny seccomp-BPF.
//! Deny `socket`/`connect`/`bind`/`listen`; default-allow remaining syscalls (runnable expand).
//! ≠ full sandbox GA · ≠ v10 five-syscall allowlist · ≠ Job Object.

/// Install net-deny filter on this thread (Linux). Other OS → Err (fail-closed).
pub fn try_install_seccomp_netdeny() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        install_linux_netdeny()
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err("seccomp net-deny unsupported on this OS (C-5c-2 fail-closed)".into())
    }
}

/// Mode label when `SYX_MACRO_SECCOMP=1` on Linux (filters installed in child).
pub const MODE_SECCOMP_NETDENY_WIRED: &str = "seccomp-netdeny-wired";

/// Mode label when env set on non-Linux (parent fail-closed; no pretend install).
pub const MODE_SECCOMP_UNSUPPORTED: &str = "seccomp-unsupported-fail-closed";

#[cfg(target_os = "linux")]
fn install_linux_netdeny() -> Result<(), String> {
    use libc::{
        c_ulong, c_void, prctl, sock_filter, sock_fprog, PR_SET_NO_NEW_PRIVS, PR_SET_SECCOMP,
        SECCOMP_MODE_FILTER, SYS_bind, SYS_connect, SYS_listen, SYS_socket,
    };

    // Classic BPF helpers (linux/filter.h).
    const BPF_LD: u16 = 0x00;
    const BPF_W: u16 = 0x00;
    const BPF_ABS: u16 = 0x20;
    const BPF_JMP: u16 = 0x05;
    const BPF_JEQ: u16 = 0x10;
    const BPF_K: u16 = 0x00;
    const BPF_RET: u16 = 0x06;
    const SECCOMP_RET_KILL_THREAD: u32 = 0x0000_0000;
    const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
    // offsetof(struct seccomp_data, nr) == 0
    const SECCOMP_DATA_NR_OFFSET: u32 = 0;

    fn stmt(code: u16, k: u32) -> sock_filter {
        sock_filter {
            code,
            jt: 0,
            jf: 0,
            k,
        }
    }
    fn jump(code: u16, k: u32, jt: u8, jf: u8) -> sock_filter {
        sock_filter { code, jt, jf, k }
    }

    // Load syscall nr; for each denied → KILL; else ALLOW.
    let deny: [u32; 4] = [
        SYS_socket as u32,
        SYS_connect as u32,
        SYS_bind as u32,
        SYS_listen as u32,
    ];
    let mut prog: Vec<sock_filter> = Vec::with_capacity(2 + deny.len() * 2);
    prog.push(stmt(BPF_LD | BPF_W | BPF_ABS, SECCOMP_DATA_NR_OFFSET));
    for &nr in &deny {
        // BPF_JEQ: equal → next (KILL); not equal → skip KILL.
        prog.push(jump(BPF_JMP | BPF_JEQ | BPF_K, nr, 0, 1));
        prog.push(stmt(BPF_RET | BPF_K, SECCOMP_RET_KILL_THREAD));
    }
    prog.push(stmt(BPF_RET | BPF_K, SECCOMP_RET_ALLOW));

    // NO_NEW_PRIVS required before SECCOMP_MODE_FILTER without CAP_SYS_ADMIN.
    let rc = unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1i64 as c_ulong, 0, 0, 0) };
    if rc != 0 {
        return Err(format!(
            "prctl(PR_SET_NO_NEW_PRIVS) failed: {}",
            std::io::Error::last_os_error()
        ));
    }

    let mut filter = prog;
    let prog_hdr = sock_fprog {
        len: filter.len() as u16,
        filter: filter.as_mut_ptr(),
    };
    let rc = unsafe {
        prctl(
            PR_SET_SECCOMP,
            SECCOMP_MODE_FILTER as c_ulong,
            &prog_hdr as *const sock_fprog as *const c_void as c_ulong,
            0,
            0,
        )
    };
    if rc != 0 {
        return Err(format!(
            "prctl(PR_SET_SECCOMP) failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c5c2_non_linux_install_fails_closed() {
        #[cfg(not(target_os = "linux"))]
        {
            let err = try_install_seccomp_netdeny().expect_err("non-linux");
            assert!(err.contains("unsupported") || err.contains("fail-closed"), "{err}");
        }
        #[cfg(target_os = "linux")]
        {
            // Smoke: install once in this thread (may affect only this test thread).
            try_install_seccomp_netdeny().expect("linux install");
        }
    }
}
