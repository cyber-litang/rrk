use anyhow::{Context, Result};
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn write_accounts<W: Write>(mut w: W, prefix: &str, ids: &[String]) -> Result<()> {
    for id in ids {
        writeln!(w, "{id}:bupt{id}::students::{prefix}/{id}:/bin/bash")?;
    }
    Ok(())
}

pub fn add_accounts(prefix: &str, ids: &[String]) -> Result<()> {
    let mut new_users = Command::new("newusers")
        .stdin(Stdio::piped())
        .arg("--badname")
        .spawn()?;
    let stdin = new_users
        .stdin
        .take()
        .context("failed to open stdin for newusers")?;
    write_accounts(stdin, prefix, ids)?;
    let status = new_users.wait()?;
    if !status.success() {
        anyhow::bail!("newusers failed");
    }
    for id in ids {
        let chmod = Command::new("chmod")
            .arg("-R")
            .arg("0700")
            .arg(format!("{prefix}/{id}"))
            .status()?;
        if !chmod.success() {
            println!("chmod for {id} failed");
        }
        let passwd = Command::new("passwd").arg("-e").arg(id).status()?;
        if !passwd.success() {
            println!("set expire for {id} failed");
        }
    }
    Ok(())
}

pub fn get_all_accounts() -> Result<Vec<String>> {
    let getent = Command::new("getent")
        .arg("group")
        .arg("students")
        .stdout(Stdio::piped())
        .spawn()?;
    let output = Command::new("cut")
        .arg("-d:")
        .arg("-f3")
        .stdin(getent.stdout.unwrap())
        .output()?;
    if !output.status.success() {
        anyhow::bail!("get group gid failed");
    }
    let students_gid: i64 = String::from_utf8_lossy(&output.stdout).trim().parse()?;
    let getent = Command::new("getent")
        .arg("passwd")
        .stdout(Stdio::piped())
        .spawn()?;
    let output = Command::new("cut")
        .arg("-d:")
        .arg("-f1,4")
        .stdin(getent.stdout.unwrap())
        .output()?;
    if !output.status.success() {
        anyhow::bail!("get passwd failed");
    }
    let mut result = Vec::new();
    for line in String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|l| !l.is_empty())
    {
        let (name, gid) = line.split_once(':').context("failed to split")?;
        if gid.parse::<i64>()? == students_gid {
            result.push(name.trim().to_string());
        }
    }
    Ok(result)
}

pub fn delete_accounts(ids: &[String]) -> Result<()> {
    for id in ids {
        let status = Command::new("userdel").arg("-r").arg(id).status()?;
        if !status.success() {
            println!("delete {id} failed");
        }
    }
    Ok(())
}

pub fn delete_all_accounts() -> Result<()> {
    let accounts = get_all_accounts()?;
    println!("accounts {:?}", accounts);
    delete_accounts(&accounts)?;
    Ok(())
}
