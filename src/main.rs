use std::collections::HashSet;
use std::io::{Result, Write};

use oops::Oops;
use stdinix::stdinix;

use clap::Parser;

/// A posix-style tool for filtering stdin based on a blocklist/allowlist.
#[derive(Parser, Debug)]
#[command(name = "filterfrom")]
struct Opt {
    /// Similar to `-k` in `sort`, if a column is provided, stdin's `column` column is matched against `list`.
    #[arg(short, long)]
    column: Option<i32>,

    /// When enabld, the filter is reversed (only matching lines are passsed to stdout).
    #[arg(short, long)]
    allow: bool,

    /// File to use as a blocklist (or allowlist).
    #[arg()]
    list: std::path::PathBuf,
}

fn filter(line: &str, column: Option<i32>, allow: bool, list: &HashSet<String>) -> Result<bool> {
    let list_contains = if let Some(column) = column {
        let matchee = {
            if column >= 0 {
                line.split_whitespace().nth(column as usize)
            } else {
                line.split_whitespace().nth_back((-1 * column - 1) as usize)
            }
        }
        .oops(&format!(
            "Requested column {} absent on input {}",
            column, line
        ))?;

        list.contains(matchee)
    } else {
        list.contains(line)
    };

    //               allow    true    false
    // list_contains
    // true                   true    false
    // false                  false   true
    Ok(allow == list_contains)
}

fn main() -> Result<()> {
    let Opt {
        column,
        allow,
        list,
    } = Opt::parse();

    let set = std::fs::read_to_string(list)?
        .lines()
        .map(|s| s.trim().to_owned())
        .collect::<HashSet<String>>();

    stdinix(|line| {
        if filter(line, column, allow, &set)? {
            println!("{}", line.trim());
            std::io::stdout().flush()
        } else {
            Ok(())
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn hello_world_set() -> HashSet<String> {
        let mut set = HashSet::new();
        set.insert("Helloworld".into());

        set
    }

    #[test]
    fn test_filer_allow_nocolumn() -> Result<()> {
        assert!(filter("Helloworld", None, true, &hello_world_set())?);

        assert!(!filter("elloworld", None, true, &hello_world_set())?);

        Ok(())
    }

    #[test]
    fn test_filer_allow_column() -> Result<()> {
        assert!(filter(
            "qwerty Helloworld",
            Some(1),
            true,
            &hello_world_set()
        )?);

        assert!(!filter(
            "qwerty Helloworld pikachu",
            Some(2),
            true,
            &hello_world_set()
        )?);

        Ok(())
    }

    #[test]
    fn test_filter_ban_nocolumn() -> Result<()> {
        assert!(filter(
            "not just Helloworld",
            None,
            false,
            &hello_world_set()
        )?);

        assert!(!filter("Helloworld", None, false, &hello_world_set())?);

        Ok(())
    }

    #[test]
    fn test_filter_ban_column() -> Result<()> {
        assert!(filter(
            "not just Helloworld",
            Some(0),
            false,
            &hello_world_set()
        )?);

        assert!(!filter("Helloworld", Some(0), false, &hello_world_set())?);

        Ok(())
    }
}
