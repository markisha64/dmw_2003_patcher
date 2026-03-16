use base64::prelude::*;
use bzip2::Compression;
use bzip2::write::BzEncoder;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;
use create_dmw_2003_patch::{Patch, PatchJSON};

#[derive(Parser)]
struct Args {
    source_bin: PathBuf,
    patched_bin: PathBuf,
    out_name: PathBuf,
}

fn find_changes(dir: &Path, changes: &mut Vec<Patch>) -> anyhow::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_changes(&path, changes)?;
        } else {
            let stripped = path.strip_prefix("./source_bin_extracted")?;
            let patched_path = Path::new("./patched_bin_extracted").join(stripped);

            let source_file = fs::read(&path)?;
            let patched_file = fs::read(&patched_path)?;

            // if changed
            if source_file != patched_file {
                let mut patch = Vec::new();

                bsdiff::diff(&source_file, &patched_file, &mut patch)?;

                let mut encoder = BzEncoder::new(Vec::new(), Compression::best());

                encoder.write_all(&patch)?;

                let base_64_str = BASE64_STANDARD.encode(encoder.finish()?);

                changes.push(Patch {
                    target: Path::new("./").join(stripped),
                    patch: base_64_str,
                });
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    Command::new("dumpsxiso")
        .arg("-x")
        .arg("source_bin_extracted")
        .arg("-pt")
        .arg(&args.source_bin)
        .output()?;

    Command::new("dumpsxiso")
        .arg("-x")
        .arg("patched_bin_extracted")
        .arg("-pt")
        .arg(&args.patched_bin)
        .output()?;

    let mut changes = Vec::new();

    find_changes(Path::new("./source_bin_extracted"), &mut changes)?;

    fs::write(
        args.out_name,
        serde_json::to_string_pretty(&PatchJSON { changes })?,
    )?;

    fs::remove_dir_all("./source_bin_extracted")?;
    fs::remove_dir_all("./patched_bin_extracted")?;

    Ok(())
}
