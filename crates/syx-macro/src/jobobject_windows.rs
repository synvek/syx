//! C-5e: Windows opt-in Job Object `PROCESS_MEMORY_LIMIT` hard wall.
//! Reuses `SYX_EMBED_RSS_MB` / `SYX_MACRO_RSS_MB` (MB → bytes).
//! Linux keeps C-5d `prlimit`; macOS / other → fail-closed.
//! ≠ full Job Object GA · ≠ macOS sandbox · ≠ C-5 `[x]`.

/// Mode when RSS MB env is set on Windows (Job Object wired).
pub const MODE_JOB_OBJECT_MEMORY_WIRED: &str = "jobobject-memory-wired";

/// Mode when RSS MB env is set on an OS without prlimit/Job Object (fail-closed).
pub const MODE_JOB_OBJECT_UNSUPPORTED: &str = "jobobject-unsupported-fail-closed";

/// Apply RSS hard wall to the **current** process.
/// Linux → `prlimit(RLIMIT_AS)`; Windows → Job Object process-memory limit; else Err.
pub fn try_apply_rss_hard_wall_bytes(limit_bytes: u64) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        crate::rlimit_linux::try_set_as_limit_bytes(limit_bytes)
    }
    #[cfg(target_os = "windows")]
    {
        apply_job_memory_to_current(limit_bytes)
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        let _ = limit_bytes;
        Err("RSS hard wall unsupported on this OS (C-5e fail-closed)".into())
    }
}

/// Apply RSS hard wall to `pid`.
/// Linux → `prlimit(pid)`; Windows → create Job + `AssignProcessToJobObject`; else Err.
pub fn try_apply_rss_hard_wall_for_pid(pid: u32, limit_bytes: u64) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        crate::rlimit_linux::try_set_as_limit_bytes_for_pid(pid as i32, limit_bytes)
    }
    #[cfg(target_os = "windows")]
    {
        apply_job_memory_to_pid(pid, limit_bytes)
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        let _ = (pid, limit_bytes);
        Err("RSS hard wall unsupported on this OS (C-5e fail-closed)".into())
    }
}

/// Windows-only: create a Job Object with `PROCESS_MEMORY_LIMIT` (exported for nails).
pub fn try_create_job_with_process_memory_limit(limit_bytes: u64) -> Result<JobHandle, String> {
    #[cfg(target_os = "windows")]
    {
        create_job_with_process_memory_limit(limit_bytes)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = limit_bytes;
        Err("CreateJobObject unsupported on this OS (C-5e fail-closed)".into())
    }
}

/// Windows-only: assign `pid` to an existing job (exported for nails).
pub fn try_assign_pid_to_job(job: &JobHandle, pid: u32) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        assign_pid_to_job(job, pid)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (job, pid);
        Err("AssignProcessToJobObject unsupported on this OS (C-5e fail-closed)".into())
    }
}

/// Opaque Job Object handle (Windows). On other OS this is a ZST placeholder.
#[derive(Debug)]
pub struct JobHandle {
    #[cfg(target_os = "windows")]
    handle: win::HANDLE,
}

impl Drop for JobHandle {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        {
            if !self.handle.is_null() {
                unsafe {
                    win::CloseHandle(self.handle);
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn apply_job_memory_to_current(limit_bytes: u64) -> Result<(), String> {
    let job = create_job_with_process_memory_limit(limit_bytes)?;
    let cur = unsafe { win::GetCurrentProcess() };
    if unsafe { win::AssignProcessToJobObject(job.handle, cur) } == 0 {
        return Err(format!(
            "AssignProcessToJobObject(current) failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    // Keep job alive for process lifetime by leaking the handle intentionally:
    // closing the last handle does not remove processes from the job, but
    // leaking avoids Drop racing with in-flight allocations on the worker path.
    std::mem::forget(job);
    Ok(())
}

#[cfg(target_os = "windows")]
fn apply_job_memory_to_pid(pid: u32, limit_bytes: u64) -> Result<(), String> {
    let job = create_job_with_process_memory_limit(limit_bytes)?;
    assign_pid_to_job(&job, pid)?;
    // Same as current: forget so Drop does not race child lifetime.
    std::mem::forget(job);
    Ok(())
}

#[cfg(target_os = "windows")]
fn create_job_with_process_memory_limit(limit_bytes: u64) -> Result<JobHandle, String> {
    let handle = unsafe { win::CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()) };
    if handle.is_null() {
        return Err(format!(
            "CreateJobObjectW failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    let mut info: win::JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
    info.BasicLimitInformation.LimitFlags = win::JOB_OBJECT_LIMIT_PROCESS_MEMORY;
    info.ProcessMemoryLimit = limit_bytes as usize;
    let ok = unsafe {
        win::SetInformationJobObject(
            handle,
            win::JobObjectExtendedLimitInformation as i32,
            &mut info as *mut _ as *mut _,
            std::mem::size_of_val(&info) as u32,
        )
    };
    if ok == 0 {
        unsafe {
            win::CloseHandle(handle);
        }
        return Err(format!(
            "SetInformationJobObject(PROCESS_MEMORY_LIMIT) failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(JobHandle { handle })
}

#[cfg(target_os = "windows")]
fn assign_pid_to_job(job: &JobHandle, pid: u32) -> Result<(), String> {
    let access = win::PROCESS_SET_QUOTA | win::PROCESS_TERMINATE | win::PROCESS_QUERY_INFORMATION;
    let proc = unsafe { win::OpenProcess(access, 0, pid) };
    if proc.is_null() {
        return Err(format!(
            "OpenProcess({pid}) failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    let ok = unsafe { win::AssignProcessToJobObject(job.handle, proc) };
    unsafe {
        win::CloseHandle(proc);
    }
    if ok == 0 {
        return Err(format!(
            "AssignProcessToJobObject(pid={pid}) failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

/// Minimal Win32 FFI (no windows-sys dep). Symbols kept for census nails.
#[cfg(target_os = "windows")]
mod win {
    use std::ffi::c_void;

    pub type HANDLE = *mut c_void;
    pub type BOOL = i32;
    pub type DWORD = u32;

    pub const JobObjectExtendedLimitInformation: u32 = 9;
    pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: u32 = 0x0000_0100;
    pub const PROCESS_TERMINATE: u32 = 0x0001;
    pub const PROCESS_SET_QUOTA: u32 = 0x0100;
    pub const PROCESS_QUERY_INFORMATION: u32 = 0x0400;

    #[repr(C)]
    pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
        pub PerProcessUserTimeLimit: i64,
        pub PerJobUserTimeLimit: i64,
        pub LimitFlags: u32,
        pub MinimumWorkingSetSize: usize,
        pub MaximumWorkingSetSize: usize,
        pub ActiveProcessLimit: u32,
        pub Affinity: usize,
        pub PriorityClass: u32,
        pub SchedulingClass: u32,
    }

    #[repr(C)]
    pub struct IO_COUNTERS {
        pub ReadOperationCount: u64,
        pub WriteOperationCount: u64,
        pub OtherOperationCount: u64,
        pub ReadTransferCount: u64,
        pub WriteTransferCount: u64,
        pub OtherTransferCount: u64,
    }

    #[repr(C)]
    pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
        pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
        pub IoInfo: IO_COUNTERS,
        pub ProcessMemoryLimit: usize,
        pub JobMemoryLimit: usize,
        pub PeakProcessMemoryUsed: usize,
        pub PeakJobMemoryUsed: usize,
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn CreateJobObjectW(lpJobAttributes: *mut c_void, lpName: *const u16) -> HANDLE;
        pub fn SetInformationJobObject(
            hJob: HANDLE,
            JobObjectInformationClass: i32,
            lpJobObjectInformation: *mut c_void,
            cbJobObjectInformationLength: DWORD,
        ) -> BOOL;
        pub fn AssignProcessToJobObject(hJob: HANDLE, hProcess: HANDLE) -> BOOL;
        pub fn OpenProcess(dwDesiredAccess: DWORD, bInheritHandle: BOOL, dwProcessId: DWORD)
            -> HANDLE;
        pub fn CloseHandle(hObject: HANDLE) -> BOOL;
        pub fn GetCurrentProcess() -> HANDLE;
    }
}

// Keep CreateJobObjectW / AssignProcessToJobObject / PROCESS_MEMORY_LIMIT tokens
// visible in source on all targets for census/evergreen hooks (cfg-gated bodies above).
#[allow(dead_code)]
const _C5E_HOOK_CREATE_JOB_OBJECT: &str = "CreateJobObjectW";
#[allow(dead_code)]
const _C5E_HOOK_ASSIGN: &str = "AssignProcessToJobObject";
#[allow(dead_code)]
const _C5E_HOOK_PROCESS_MEMORY: &str = "PROCESS_MEMORY_LIMIT";
#[allow(dead_code)]
const _C5E_HOOK_JOB_OBJECT_EXTENDED: &str = "JOBOBJECT_EXTENDED_LIMIT_INFORMATION";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c5e_non_windows_create_fails_closed() {
        #[cfg(not(target_os = "windows"))]
        {
            let err =
                try_create_job_with_process_memory_limit(64 * 1024 * 1024).expect_err("fail-closed");
            assert!(
                err.contains("unsupported") || err.contains("fail-closed"),
                "{err}"
            );
            let err2 = try_apply_rss_hard_wall_bytes(64 * 1024 * 1024);
            #[cfg(target_os = "linux")]
            {
                // Linux path uses prlimit — high limit should succeed.
                err2.expect("Linux prlimit via hard-wall dispatcher");
            }
            #[cfg(not(any(target_os = "linux", target_os = "windows")))]
            {
                let e = err2.expect_err("macOS/other fail-closed");
                assert!(
                    e.contains("unsupported") || e.contains("fail-closed"),
                    "{e}"
                );
            }
        }
        #[cfg(target_os = "windows")]
        {
            // High ceiling: must create + assign without killing the test process.
            let high = 8u64 * 1024 * 1024 * 1024;
            try_apply_rss_hard_wall_bytes(high).expect("Windows Job Object high limit");
        }
    }
}
