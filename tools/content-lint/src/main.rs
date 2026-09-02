use anyhow::{Context, Result, bail};
use kana_curriculum::load_lesson;
use std::{env, fs, path::Path};

fn main() -> Result<()> {
    let path = env::args().nth(1).unwrap_or_else(|| "content/zh-CN".into());
    let root = Path::new(&path);

    if !root.exists() {
        bail!("content path does not exist: {}", root.display());
    }

    let mut checked = 0usize;
    visit(root, &mut checked)?;
    println!("validated {checked} lesson file(s)");
    Ok(())
}

fn visit(path: &Path, checked: &mut usize) -> Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path).with_context(|| format!("read {}", path.display()))? {
            visit(&entry?.path(), checked)?;
        }
        return Ok(());
    }

    let is_yaml = matches!(
        path.extension().and_then(|v| v.to_str()),
        Some("yaml" | "yml")
    );
    if !is_yaml {
        return Ok(());
    }

    let yaml = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    load_lesson(&yaml).with_context(|| format!("validate {}", path.display()))?;
    *checked += 1;
    println!("ok  {}", path.display());
    Ok(())
}
