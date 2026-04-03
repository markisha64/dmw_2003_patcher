use anyhow::Context as _;
use tokio::process::Command;

async fn exists(exec: &str) -> anyhow::Result<bool> {
    Ok(Command::new(exec).output().await?.status.success())
}

async fn find_bin(name: &str) -> anyhow::Result<String> {
    if exists(name).await? {
        return Ok(String::from(name));
    }

    let built = format!("mkpsxiso/build/{name}");
    if exists(&built).await? {
        return Ok(built);
    }

    Err(anyhow::anyhow!("Can't find bin"))
}

pub async fn extract(path: &std::path::PathBuf) -> anyhow::Result<bool> {
    let bin = find_bin("dumpsxiso").await?;

    let file_name = path
        .file_name()
        .context("failed to get file_name")?
        .to_str()
        .context("failed to convert file name to string")?;

    Ok(Command::new(bin)
        .arg("-x")
        .arg(format!("extract/{}/", file_name))
        .arg("-s")
        .arg("extract/out.xml")
        .arg("-pt")
        .arg(path)
        .output()
        .await?
        .status
        .success())
}

pub async fn build(rom_name: &str, filename: String) -> anyhow::Result<bool> {
    let binf = find_bin("mkpsxiso").await?;

    let bin = format!("patched/{}/{}/new.bin", rom_name, filename);
    let cue = format!("patched/{}/{}/new.cue", rom_name, filename);

    Ok(Command::new(binf)
        .arg("-o")
        .arg(&bin)
        .arg("-c")
        .arg(&cue)
        .arg("extract/out.xml")
        .arg("-y")
        .output()
        .await?
        .status
        .success())
}
