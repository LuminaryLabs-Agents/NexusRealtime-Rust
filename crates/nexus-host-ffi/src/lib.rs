use nexus_host::HostRuntime;
use std::ffi::{c_char, CStr, CString};

const DEMO_SEQUENCE: &str = r#"{"id":"macos_demo","type":"flow","children":[{"id":"panel","type":"host-command","command":"spawn_panel","data":{"position":[0,1.4,-2],"label":"NexusEngine Rust Demo"}}]}"#;
const DEMO_MANIFEST: &str = r#"{"schema":"nexus.dsk-manifest.v1","sources":{"core":"LuminaryLabs-Dev/NexusRealtime"},"kits":[{"id":"n:macos-demo","name":"macOS demo","kind":"rust-host"}]}"#;

#[no_mangle]
pub extern "C" fn nexus_host_demo_status() -> *mut c_char {
    into_c_string(demo_status())
}

#[no_mangle]
pub extern "C" fn nexus_host_tick_summary(
    sequence_json: *const c_char,
    manifest_json: *const c_char,
) -> *mut c_char {
    let sequence =
        unsafe { read_c_string(sequence_json) }.unwrap_or_else(|| DEMO_SEQUENCE.to_string());
    let manifest =
        unsafe { read_c_string(manifest_json) }.unwrap_or_else(|| DEMO_MANIFEST.to_string());
    into_c_string(run_host(&sequence, &manifest))
}

#[no_mangle]
pub extern "C" fn nexus_host_string_free(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(value);
    }
}

fn demo_status() -> String {
    run_host(DEMO_SEQUENCE, DEMO_MANIFEST)
}

fn run_host(sequence: &str, manifest: &str) -> String {
    match HostRuntime::from_json(sequence, manifest) {
        Ok(mut runtime) => {
            runtime.start();
            let before = runtime.status();
            let buffer = runtime.tick(1.0 / 60.0);
            format!("{before}\n{}\n{}", buffer.summary(), runtime.status())
        }
        Err(error) => format!("Nexus host failed: {error}"),
    }
}

unsafe fn read_c_string(value: *const c_char) -> Option<String> {
    if value.is_null() {
        return None;
    }
    CStr::from_ptr(value).to_str().ok().map(ToString::to_string)
}

fn into_c_string(value: String) -> *mut c_char {
    let sanitized = value.replace('\0', " ");
    CString::new(sanitized)
        .unwrap_or_else(|_| {
            CString::new("Nexus host returned invalid text").expect("static string is valid")
        })
        .into_raw()
}
