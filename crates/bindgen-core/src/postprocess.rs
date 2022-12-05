use std::{
    ffi::OsStr,
    io::{Read, Write},
    process::{Command, Stdio},
};

pub fn postprocess<I, S>(file: &mut String, cmd: impl AsRef<OsStr>, args: I) -> Result<(), Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(file.as_bytes())?;
    file.truncate(0);
    child.stdout.take().unwrap().read_to_string(file)?;
    let status = child.wait()?;
    assert!(status.success());

    Ok(())
}
