use anyhow::Result;

pub fn normalize(input: &str) -> String {
    input.trim().to_lowercase()
}

pub fn run() -> Result<()> {
    Ok(())
}
