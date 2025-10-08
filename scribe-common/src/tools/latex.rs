use std::error::Error;
use tempfile::TempDir;
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;
use tokio::process::Command;

/// Converts latex to SVG.
///
/// Requires the `latex` and `dvisvgm` tools to be on the `$PATH`.
pub async fn latex_to_svg(source: &str) -> Result<String, LatexError> {
    let dvi = latex_to_dvi(source).await?;
    dvi_to_svg(&dvi).await
}

async fn latex_to_dvi(source: &str) -> Result<Vec<u8>, LatexError> {
    if !is_command_available("latex").await {
        return Err(LatexError::MissingTool("latex".into()));
    }

    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("input.tex");

    fs::write(&file_path, source).await?;

    let output = Command::new("latex")
        .arg("-halt-on-error")
        .arg("-interaction=nonstopmode")
        .arg(file_path)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(LatexError::CompileError {
            command: "latex".into(),
            stderr: stderr.into(),
        });
    }

    let dvi_content = fs::read(&temp_dir.path().join("input.dvi")).await?;
    Ok(dvi_content)
}

async fn dvi_to_svg(dvi: &[u8]) -> Result<String, LatexError> {
    if !is_command_available("dvisvgm").await {
        return Err(LatexError::MissingTool("dvisvgm".into()));
    }

    let mut child = Command::new("dvisvgm")
        .arg("--exact")
        .arg("--clipjoin")
        .arg("--font-format=woff")
        .arg("--bbox=papersize")
        .arg("--zoom=1.5")
        .arg("--stdin")
        .arg("--stdout")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(dvi).await?;
    drop(stdin);

    let output = child.wait_with_output().await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(LatexError::CompileError {
            command: "dvisvgm".into(),
            stderr: stderr.into(),
        });
    }

    let svg = String::from_utf8_lossy(&output.stdout).into();
    Ok(svg)
}

async fn is_command_available(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[derive(Debug, Error)]
pub enum LatexError {
    #[error("The CLI tool `{0}` is missing in the $PATH.")]
    MissingTool(String),
    #[error("Failed to run `{command}`:\n{stderr}")]
    CompileError { command: String, stderr: String },
    #[error("Error while compiling latex.")]
    Other(#[source] Box<dyn Error>),
}

impl From<std::io::Error> for LatexError {
    fn from(value: std::io::Error) -> Self {
        Self::Other(value.into())
    }
}
