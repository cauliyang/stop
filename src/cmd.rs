use anyhow::Context;
use std::process::Command;

pub fn squeue(args: Option<&[&str]>) -> anyhow::Result<String> {
    let mut output = Command::new("squeue");

    if let Some(arguments) = args {
        let output = output.args(arguments);
    }

    let output = output
        .output()
        .with_context(|| "squeue failed to execute".to_string())?;

    Ok(String::from_utf8(output.stdout)?)
}

pub fn sinfo(args: Option<&[&str]>) -> anyhow::Result<String> {
    let mut output = Command::new("sinfo");

    if let Some(arguments) = args {
        let output = output.args(arguments);
    }

    let output = output
        .output()
        .with_context(|| "sinfo failed to execute".to_string())?;

    Ok(String::from_utf8(output.stdout)?)
}

pub fn scontrol(args: Option<&[&str]>) -> anyhow::Result<String> {
    let mut output = Command::new("scontrol");
    let output = output.arg("show");

    if let Some(arguments) = args {
        let output = output.args(arguments);
    }

    let output = output
        .output()
        .with_context(|| "sinfo failed to execute".to_string())?;

    Ok(String::from_utf8(output.stdout)?)
}
