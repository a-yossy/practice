use std::io::BufRead;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?")?;
    let haystack = r#"regex = "0.2.5""#;
    assert!(semver.is_match(haystack));
    let captures = semver
        .captures(haystack)
        .ok_or_else(|| anyhow::anyhow!("semver regex should have matched"))?;
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start(), 13);

    let haystack = "In the beginning, there was 1.0.0. \
                For a while, we used 1.0.1-beta, \
                but in the end, we settled on 1.2.4.";
    let matches: Vec<&str> = semver
        .find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();
    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

    lazy_static! {
        static ref SEMVER: Regex =
            Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("error parsing regex");
    }
    let stdin = std::io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result?;
        if let Some(match_) = SEMVER.find(&line) {
            println!("{}", match_.as_str());
        }
    }

    Ok(())
}
