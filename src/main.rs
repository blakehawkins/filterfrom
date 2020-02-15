use oops::Oops;
use stdinix::stdinix;

#[paw::main]
fn main(mut args: paw::Args) -> std::io::Result<()> {
    let ban_path = args.nth(1).oops("Missing banlist")?;

    let banset = std::fs::read_to_string(ban_path)?
        .lines()
        .map(|s| s.trim().to_owned())
        .collect::<std::collections::HashSet<String>>();

    stdinix(|line| {
        if !banset.contains(line.trim()) {
            println!("{}", line.trim());
        }

        Ok(())
    })
}
