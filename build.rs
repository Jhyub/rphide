#[cfg(target_os = "windows")]
extern crate windres;

#[cfg(target_os = "windows")]
use windres::Build;

#[cfg(target_os = "windows")]
fn main() {
    Build::new().compile("logo.rc").unwrap();
}

#[cfg(target_os = "linux")]
fn main() {

}