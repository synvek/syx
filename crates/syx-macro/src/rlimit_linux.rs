//! C-5d: Linux opt-in `prlimit(RLIMIT_AS)` hard address-space wall.
//! Opt-in via `SYX_EMBED_RSS_MB` / `SYX_MACRO_RSS_MB` (MB → bytes).
//! ≠ Job Object · ≠ macOS sandbox · ≠ C-5 `[x]`.

/// Mode when RSS MB env is set on Linux (prlimit wired).
pub const MODE_RLIMIT_AS_WIRED: &str = "rlimit-as-wired";

/// Mode when RSS MB env is set on non-Linux (parent fail-closed).
pub const MODE_RLIMIT_UNSUPPORTED: &str = "rlimit-unsupported-fail-closed";

/// Set `RLIMIT_AS` (virtual address space) for the current process (pid 0).
/// Linux only; other OS → Err (fail-closed).
pub fn try_set_as_limit_bytes(limit_bytes: u64) -> Result<(), String> {
    try_set_as_limit_bytes_for_pid(0, limit_bytes)
}

/// Set `RLIMIT_AS` for `pid` (0 = current). Linux only; other OS → Err.
pub fn try_set_as_limit_bytes_for_pid(pid: i32, limit_bytes: u64) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        set_as_limit_linux(pid, limit_bytes)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (pid, limit_bytes);
        Err("prlimit(RLIMIT_AS) unsupported on this OS (C-5d fail-closed)".into())
    }
}

#[cfg(target_os = "linux")]
fn set_as_limit_linux(pid: i32, limit_bytes: u64) -> Result<(), String> {
    use libc::{prlimit, rlimit, RLIMIT_AS};

    let lim = rlimit {
        rlim_cur: limit_bytes,
        rlim_max: limit_bytes,
    };
    let rc = unsafe { prlimit(pid, RLIMIT_AS, &lim, std::ptr::null_mut()) };
    if rc != 0 {
        return Err(format!(
            "prlimit(RLIMIT_AS) failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c5d_non_linux_install_fails_closed() {
        #[cfg(not(target_os = "linux"))]
        {
            let err = try_set_as_limit_bytes(64 * 1024 * 1024).expect_err("must fail-closed");
            assert!(
                err.contains("unsupported") || err.contains("fail-closed"),
                "{err}"
            );
        }
        #[cfg(target_os = "linux")]
        {
            // High ceiling: must succeed without killing the test process.
            let high = 8u64 * 1024 * 1024 * 1024; // 8 GiB AS
            try_set_as_limit_bytes(high).expect("Linux prlimit high AS");
        }
    }

    /// Linux: prlimit on a child that grows RSS → kernel kill (hard wall evidence).
    #[cfg(target_os = "linux")]
    #[test]
    fn c5d_prlimit_kills_allocating_child() {
        use std::process::{Command, Stdio};
        use std::thread;
        use std::time::Duration;

        let mut child = Command::new("python3")
            .args([
                "-c",
                "import time; b=bytearray(256*1024*1024); time.sleep(2)",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn python alloc");
        let pid = child.id() as i32;
        // Tiny AS relative to 256MiB allocation → child must die.
        try_set_as_limit_bytes_for_pid(pid, 16 * 1024 * 1024).expect("prlimit child");
        let mut dead = false;
        for _ in 0..50 {
            match child.try_wait() {
                Ok(Some(status)) => {
                    assert!(!status.success(), "child must not exit 0 under AS wall");
                    dead = true;
                    break;
                }
                Ok(None) => thread::sleep(Duration::from_millis(50)),
                Err(e) => panic!("try_wait: {e}"),
            }
        }
        if !dead {
            let _ = child.kill();
            let _ = child.wait();
            panic!("C-5d: expected prlimit to kill allocating child");
        }
    }
}
