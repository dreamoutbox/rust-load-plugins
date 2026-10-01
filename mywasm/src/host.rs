//! Host API bindings and safe wrappers for WASM plugins.

mod ffi {
    #[link(wasm_import_module = "host")]
    unsafe extern "C" {
        // level: 0=INFO, 1=WARN, 2=ERROR
        // msg_ptr / msg_len: UTF-8 byte slice in WASM linear memory
        pub fn host_log(level: i32, msg_ptr: i32, msg_len: i32);

        // Returns host version as integer: major * 100 + minor
        pub fn host_get_version() -> i32;
    }
}

/// Structured version returned by the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// Send an info log message to the host.
pub fn host_log(msg: impl AsRef<str>) {
    let s = msg.as_ref();
    unsafe {
        ffi::host_log(0, s.as_ptr() as i32, s.len() as i32);
    }
}

/// Query the host version.
pub fn host_get_version() -> Version {
    let raw = unsafe { ffi::host_get_version() };
    Version {
        major: (raw / 100) as u32,
        minor: (raw % 100) as u32,
    }
}
