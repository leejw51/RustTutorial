fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = tonic_build::configure().out_dir("src");
    config.compile(&["proto/say.proto"],&["proto/"])?;
    Ok(())
}
