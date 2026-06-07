use anyhow::Context as _;
use anyhow::anyhow;
use tokio::process::Command;

async fn exists(exec: &str) -> bool {
    Command::new(exec)
        .output()
        .await
        .map(|x| x.status.success())
        .unwrap_or(false)
}

async fn find_bin(name: &str) -> anyhow::Result<String> {
    if exists(name).await {
        return Ok(String::from(name));
    }

    let built = format!("./{name}");
    if exists(&built).await {
        return Ok(built);
    }

    let built = format!("mkpsxiso/build/{name}");
    if exists(&built).await {
        return Ok(built);
    }

    Err(anyhow::anyhow!("Can't find {}", name))
}

pub async fn extract(path: &std::path::PathBuf) -> anyhow::Result<()> {
    let bin = find_bin("dumpsxiso").await?;

    let file_name = path
        .file_name()
        .context("failed to get file_name")?
        .to_str()
        .context("failed to convert file name to string")?;

    let output = Command::new(bin)
        .arg("-x")
        .arg(format!("extract/{}/", file_name))
        .arg("-s")
        .arg("extract/out.xml")
        .arg("-pt")
        .arg("--lba")
        .arg(path)
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(String::from_utf8_lossy(&output.stdout).to_string()));
    }

    Ok(())
}

pub async fn build(rom_name: &str, filename: String) -> anyhow::Result<()> {
    let binf = find_bin("mkpsxiso").await?;

    let bin = format!("patched/{}/{}/new.bin", rom_name, filename);
    let cue = format!("patched/{}/{}/new.cue", rom_name, filename);

    let output = Command::new(binf)
        .arg("-o")
        .arg(&bin)
        .arg("-c")
        .arg(&cue)
        .arg("extract/out.xml")
        .arg("-y")
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!(String::from_utf8_lossy(&output.stdout).to_string()));
    }

    Ok(())
}
