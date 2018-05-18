

#[macro_use]
extern crate error_chain;
extern crate regex;

use std::process::Command;
use regex::Regex;
use regex::RegexSetBuilder;
error_chain!{
    foreign_links {
        Io(std::io::Error);
        Regex(regex::Error);
        Utf8(std::string::FromUtf8Error);
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn run() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message")?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
            Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            }
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
use std::collections::HashSet;
use std::io::Write;
use std::process::{ Stdio};
fn run_02() -> Result<()> {
    let mut child = Command::new("python").stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")?
        .write_all(b"import this; copyright(); credits(); exit()")?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output.split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        bail!("External command failed:\n {}", err)
    }
}



fn run_03() -> Result<()> {
    let directory = std::env::current_dir()?;
    let du_output = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| "Could not capture `du` standard output.")?;

    let sort_output = Command::new("sort")
        .arg("-hr")
        .stdin(du_output)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| "Could not capture `sort` standard output.")?;

    let head_output = Command::new("head")
        .args(&["-n", "10"])
        .stdin(sort_output)
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    println!(
        "Top 10 biggest files and directories in '{}':\n{}",
        directory.display(),
        String::from_utf8(head_output.stdout)?
    );

    Ok(())
}

use std::fs::File;
use std::io::BufReader;

fn run_04() -> Result<()> {
    let outputs = File::create("out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
use std::io::BufRead;
fn run_05() -> Result<()> {
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| "Could not capture standard output.")?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}





fn run_06() -> Result<()> {

    let log_path = "application.log";
    let buffered = BufReader::new(File::open(log_path)?);

    let set = RegexSetBuilder::new(&[
        r#"version "\d\.\d\.\d""#,
        r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
        r#"warning.*timeout expired"#,
    ]).case_insensitive(true)
        .build()?;


    buffered
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| set.is_match(line.as_str()))
        .for_each(|x| println!("{}", x));

    Ok(())
}


fn main() {
    run_05();
    run_04();
    run_03();
    run_02();
    run();
}

