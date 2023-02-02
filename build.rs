fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]").compile(
        &["./proto/backend.proto"],
        &["."],
    )?;
    Ok(())
}
