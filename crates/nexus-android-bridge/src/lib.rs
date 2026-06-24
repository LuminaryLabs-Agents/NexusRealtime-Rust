use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use nexus_host::HostRuntime;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static RUNTIME: Lazy<Mutex<Option<HostRuntime>>> = Lazy::new(|| Mutex::new(None));

#[no_mangle]
pub extern "system" fn Java_dev_luminarylabs_nexusrealtime_MainActivity_nativeInit(
    mut env: JNIEnv,
    _class: JClass,
    sequence_json: JString,
    manifest_json: JString,
) -> jstring {
    let sequence = read_java_string(&mut env, &sequence_json);
    let manifest = read_java_string(&mut env, &manifest_json);

    let status = match HostRuntime::from_json(&sequence, &manifest) {
        Ok(mut runtime) => {
            runtime.start();
            let status = runtime.status();
            if let Ok(mut slot) = RUNTIME.lock() {
                *slot = Some(runtime);
            }
            status
        }
        Err(error) => format!("Nexus host init failed: {error}"),
    };

    java_string(&mut env, &status)
}

#[no_mangle]
pub extern "system" fn Java_dev_luminarylabs_nexusrealtime_MainActivity_nativeTick(
    mut env: JNIEnv,
    _class: JClass,
    dt: f32,
) -> jstring {
    let status = match RUNTIME.lock() {
        Ok(mut slot) => match slot.as_mut() {
            Some(runtime) => {
                let buffer = runtime.tick(dt);
                format!("{} | {}", runtime.status(), buffer.summary())
            }
            None => "Nexus host is not initialized".to_string(),
        },
        Err(_) => "Nexus host runtime lock failed".to_string(),
    };

    java_string(&mut env, &status)
}

fn read_java_string(env: &mut JNIEnv, value: &JString) -> String {
    env.get_string(value)
        .map(|text| text.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn java_string(env: &mut JNIEnv, value: &str) -> jstring {
    env.new_string(value)
        .map(|text| text.into_raw())
        .unwrap_or(std::ptr::null_mut())
}
