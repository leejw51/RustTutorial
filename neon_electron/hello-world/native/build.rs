use neon_build;
extern crate cc;
fn main() {
    neon_build::setup(); // must be called in build.rs

    // add project-specific build logic here...
     cc::Build::new()
        .cpp(true)
        .static_crt(true)
        .file("src/win_delay_load_hook.cc")
        .compile("hook");
}
