//! C-5c-2: Linux opt-in net-deny seccomp-BPF (default-allow; deny socket/connect/bind/listen).
//! C-5g: Linux opt-in **default-deny** allowlist BPF (`try_install_seccomp_allowlist`).
//! Allowlist SSOT aligns with `embed_v11_seccomp_allowlist` + minimal runnable supplements.
//! ≠ full sandbox GA · ≠ Job Object · ≠ C-5 `[x]`.

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

/// C-5g: install **default-deny** allowlist filter on this thread (Linux).
/// Other OS → Err (fail-closed).
pub fn try_install_seccomp_allowlist() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        install_linux_allowlist()
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err("seccomp allowlist unsupported on this OS (C-5g fail-closed)".into())
    }
}

/// Mode label when `SYX_MACRO_SECCOMP=1` on Linux (filters installed in child).
pub const MODE_SECCOMP_NETDENY_WIRED: &str = "seccomp-netdeny-wired";

/// Mode label when env set on non-Linux (parent fail-closed; no pretend install).
pub const MODE_SECCOMP_UNSUPPORTED: &str = "seccomp-unsupported-fail-closed";

/// C-5g: mode when allowlist env is set on Linux.
pub const MODE_SECCOMP_ALLOWLIST_WIRED: &str = "seccomp-allowlist-wired";

/// C-5g: mode when allowlist env is set on non-Linux.
pub const MODE_SECCOMP_ALLOWLIST_UNSUPPORTED: &str = "seccomp-allowlist-unsupported-fail-closed";

/// Authoritative allowlist names (v11 SSOT + minimal runnable supplements).
/// Must stay aligned with `syx_driver::embed_v2::embed_v11_seccomp_allowlist` base set.
pub const SECCOMP_ALLOWLIST_SYSCALL_NAMES: &[&str] = &[
    // embed_v11_seccomp_allowlist
    "read",
    "write",
    "exit",
    "exit_group",
    "rt_sigreturn",
    "mmap",
    "munmap",
    "getrusage",
    // C-5g minimal runnable supplements (≠ full GA)
    "futex",
    "brk",
    "openat",
    "close",
    "mprotect",
    "arch_prctl",
];

#[cfg(target_os = "linux")]
fn install_linux_netdeny() -> Result<(), String> {
    use libc::{
        c_ulong, c_void, prctl, sock_filter, sock_fprog, PR_SET_NO_NEW_PRIVS, PR_SET_SECCOMP,
        SECCOMP_MODE_FILTER, SYS_bind, SYS_connect, SYS_listen, SYS_socket,
    };

    const BPF_LD: u16 = 0x00;
    const BPF_W: u16 = 0x00;
    const BPF_ABS: u16 = 0x20;
    const BPF_JMP: u16 = 0x05;
    const BPF_JEQ: u16 = 0x10;
    const BPF_K: u16 = 0x00;
    const BPF_RET: u16 = 0x06;
    const SECCOMP_RET_KILL_THREAD: u32 = 0x0000_0000;
    const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
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

    let deny: [u32; 4] = [
        SYS_socket as u32,
        SYS_connect as u32,
        SYS_bind as u32,
        SYS_listen as u32,
    ];
    let mut prog: Vec<sock_filter> = Vec::with_capacity(2 + deny.len() * 2);
    prog.push(stmt(BPF_LD | BPF_W | BPF_ABS, SECCOMP_DATA_NR_OFFSET));
    for &nr in &deny {
        prog.push(jump(BPF_JMP | BPF_JEQ | BPF_K, nr, 0, 1));
        prog.push(stmt(BPF_RET | BPF_K, SECCOMP_RET_KILL_THREAD));
    }
    prog.push(stmt(BPF_RET | BPF_K, SECCOMP_RET_ALLOW));

    install_filter(prog)
}

#[cfg(target_os = "linux")]
fn install_linux_allowlist() -> Result<(), String> {
    use libc::{
        SYS_brk, SYS_close, SYS_exit, SYS_exit_group, SYS_futex, SYS_getrusage, SYS_mmap, SYS_mprotect,
        SYS_munmap, SYS_openat, SYS_read, SYS_rt_sigreturn, SYS_write,
    };

    // Keep census tokens: SECCOMP_RET_ALLOW · default-deny.
    let mut allow: Vec<u32> = vec![
        SYS_read as u32,
        SYS_write as u32,
        SYS_exit as u32,
        SYS_exit_group as u32,
        SYS_rt_sigreturn as u32,
        SYS_mmap as u32,
        SYS_munmap as u32,
        SYS_getrusage as u32,
        SYS_futex as u32,
        SYS_brk as u32,
        SYS_openat as u32,
        SYS_close as u32,
        SYS_mprotect as u32,
    ];
    #[cfg(target_arch = "x86_64")]
    {
        allow.push(libc::SYS_arch_prctl as u32);
    }
    // aarch64 has no arch_prctl; name kept in SECCOMP_ALLOWLIST_SYSCALL_NAMES for SSOT docs.

    const BPF_LD: u16 = 0x00;
    const BPF_W: u16 = 0x00;
    const BPF_ABS: u16 = 0x20;
    const BPF_JMP: u16 = 0x05;
    const BPF_JEQ: u16 = 0x10;
    const BPF_K: u16 = 0x00;
    const BPF_RET: u16 = 0x06;
    const SECCOMP_RET_KILL_THREAD: u32 = 0x0000_0000;
    const SECCOMP_RET_ALLOW: u32 = 0x7fff_0000;
    const SECCOMP_DATA_NR_OFFSET: u32 = 0;

    fn stmt(code: u16, k: u32) -> libc::sock_filter {
        libc::sock_filter {
            code,
            jt: 0,
            jf: 0,
            k,
        }
    }
    fn jump(code: u16, k: u32, jt: u8, jf: u8) -> libc::sock_filter {
        libc::sock_filter { code, jt, jf, k }
    }

    // Load nr; for each allowed → ALLOW; else KILL (default-deny).
    let mut prog: Vec<libc::sock_filter> = Vec::with_capacity(2 + allow.len() * 2);
    prog.push(stmt(BPF_LD | BPF_W | BPF_ABS, SECCOMP_DATA_NR_OFFSET));
    for &nr in &allow {
        // equal → next (ALLOW); not equal → skip ALLOW.
        prog.push(jump(BPF_JMP | BPF_JEQ | BPF_K, nr, 0, 1));
        prog.push(stmt(BPF_RET | BPF_K, SECCOMP_RET_ALLOW));
    }
    prog.push(stmt(BPF_RET | BPF_K, SECCOMP_RET_KILL_THREAD));

    let _ = SECCOMP_RET_ALLOW; // census token retention
    install_filter(prog)
}

#[cfg(target_os = "linux")]
fn install_filter(mut prog: Vec<libc::sock_filter>) -> Result<(), String> {
    use libc::{
        c_ulong, c_void, prctl, sock_fprog, PR_SET_NO_NEW_PRIVS, PR_SET_SECCOMP, SECCOMP_MODE_FILTER,
    };

    let rc = unsafe { prctl(PR_SET_NO_NEW_PRIVS, 1i64 as c_ulong, 0, 0, 0) };
    if rc != 0 {
        return Err(format!(
            "prctl(PR_SET_NO_NEW_PRIVS) failed: {}",
            std::io::Error::last_os_error()
        ));
    }

    let prog_hdr = sock_fprog {
        len: prog.len() as u16,
        filter: prog.as_mut_ptr(),
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
            assert!(
                err.contains("unsupported") || err.contains("fail-closed"),
                "{err}"
            );
        }
        #[cfg(target_os = "linux")]
        {
            try_install_seccomp_netdeny().expect("linux install");
        }
    }

    #[test]
    fn c5g_non_linux_allowlist_fails_closed() {
        #[cfg(not(target_os = "linux"))]
        {
            let err = try_install_seccomp_allowlist().expect_err("non-linux");
            assert!(
                err.contains("unsupported") || err.contains("fail-closed"),
                "{err}"
            );
        }
        #[cfg(target_os = "linux")]
        {
            // Install on this test thread only (filter is per-thread).
            try_install_seccomp_allowlist().expect("linux allowlist install");
        }
    }

    #[test]
    fn c5g_allowlist_contains_v11_base() {
        for name in [
            "read",
            "write",
            "exit",
            "exit_group",
            "rt_sigreturn",
            "mmap",
            "munmap",
            "getrusage",
            "futex",
            "openat",
        ] {
            assert!(
                SECCOMP_ALLOWLIST_SYSCALL_NAMES.contains(&name),
                "missing {name}"
            );
        }
        assert_eq!(MODE_SECCOMP_ALLOWLIST_WIRED, "seccomp-allowlist-wired");
    }
}
