use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::net::IpAddr;
use std::rc::Rc;
use std::str::FromStr;

use num::Complex;

fn main() {
    assert_eq!("カニ".chars().next(), Some('カ'));

    assert!(32u8.is_ascii_whitespace());
    assert!(b'9'.is_ascii_digit());

    assert_eq!('F'.to_digit(16), Some(15));
    assert_eq!(std::char::from_digit(15, 16), Some('f'));
    assert!(char::is_digit('f', 16));

    let mut upper = 's'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);

    let mut upper = 'ß'.to_uppercase();
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), Some('S'));
    assert_eq!(upper.next(), None);

    let ch = 'İ';
    let mut lower = ch.to_lowercase();
    assert_eq!(lower.next(), Some('i'));
    assert_eq!(lower.next(), Some('\u{307}'));
    assert_eq!(lower.next(), None);

    assert_eq!('B' as u32, 66);
    assert_eq!('饂' as u8, 66);
    assert_eq!('二' as i8, -116);

    assert_eq!(char::from(66), 'B');
    assert_eq!(std::char::from_u32(0x9942), Some('饂'));
    assert_eq!(std::char::from_u32(0xd800), None);

    let spacey = "man hat tan";
    let spaceless: String = spacey.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!(spaceless, "manhattan");

    let full = "bookkeeping";
    assert_eq!(&full[..4], "book");
    assert_eq!(&full[5..], "eeping");
    assert_eq!(&full[2..4], "ok");
    assert_eq!(full[..].len(), 11);
    assert_eq!(full[5..].contains("boo"), false);

    let parenthesized = "Rust (饂)";
    assert_eq!(parenthesized[6..].chars().next(), Some('饂'));

    let mut also_spaceless = "con".to_string();
    also_spaceless.extend("tri but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");

    let mut letter = String::new();
    writeln!(letter, "Whose {} these are I think I know", "rutabagas").unwrap();
    writeln!(letter, "His house is in the village though").unwrap();
    assert_eq!(
        letter,
        "Whose rutabagas these are I think I know\nHis house is in the village though\n"
    );

    let left = "partners".to_string();
    let mut right = "crime".to_string();
    assert_eq!(left + " in " + &right, "partners in crime");
    right += " and punishment";
    assert_eq!(right, "crime and punishment");

    let _parenthetical = "(".to_string() + ")";

    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    let mut winston = "Churchill".to_string();
    winston.drain(2..6);
    assert_eq!(winston, "Chill");

    let mut beverage = "a piña colada".to_string();
    beverage.replace_range(2..7, "kahlua");
    assert_eq!(beverage, "a kahlua colada");

    let haystack = "One fine day, in the middle of the night";
    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));
    assert_eq!(
        "## Elephants".trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()),
        "Elephants"
    );
    let code = "\t    function noodle() { ";
    assert_eq!(
        code.trim_start_matches([' ', '\t'].as_ref()),
        "function noodle() { "
    );

    assert!("2017".starts_with(char::is_numeric));

    let quip = "We also know there are known unknowns";
    assert_eq!(quip.find("know"), Some(8));
    assert_eq!(quip.rfind("know"), Some(31));
    assert_eq!(quip.find("ya know"), None);
    assert_eq!(quip.rfind(char::is_uppercase), Some(0));

    assert_eq!(
        "The only thing we have to fear is fear itself".replace("fear", "spin"),
        "The only thing we have to spin is spin itself"
    );
    assert_eq!(
        "`Borrow` and `BorrowMut`".replace(|ch: char| !ch.is_alphanumeric(), ""),
        "BorrowandBorrowMut"
    );

    assert_eq!(
        "élan".char_indices().collect::<Vec<_>>(),
        vec![(0, 'é'), (2, 'l'), (3, 'a'), (4, 'n')]
    );

    assert_eq!(
        "jimb:1000:Jim Blandy:".split(':').collect::<Vec<_>>(),
        vec!["jimb", "1000", "Jim Blandy", ""]
    );
    assert_eq!(
        "127.0.0.1  localhost\n\
            127.0.0.1  www.reddit.com\n"
            .split_terminator('\n')
            .collect::<Vec<_>>(),
        vec!["127.0.0.1  localhost", "127.0.0.1  www.reddit.com"]
    );

    let poem = "This  is  just  to say\n\
            I have eaten\n\
            the plums\n\
            again\n";
    assert_eq!(
        poem.split_whitespace().collect::<Vec<_>>(),
        vec![
            "This", "is", "just", "to", "say", "I", "have", "eaten", "the", "plums", "again"
        ]
    );

    assert_eq!("\t*.rs ".trim(), "*.rs");
    assert_eq!("001990".trim_start_matches('0'), "1990");

    let slice = "banana";
    assert_eq!(slice.strip_suffix("na"), Some("bana"));

    assert_eq!(usize::from_str("3600"), Ok(3600));
    assert_eq!(f64::from_str("128.56"), Ok(128.56));
    assert_eq!(bool::from_str("true"), Ok(true));
    assert!(f64::from_str("not").is_err());
    assert!(bool::from_str("TRUE").is_err());
    assert_eq!(char::from_str("é"), Ok('é'));
    assert!(char::from_str("test").is_err());

    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
    assert_eq!(
        address,
        IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50])
    );
    let _address = "fe80::0000:3ea9:f4ff:fe34:7a50".parse::<IpAddr>().unwrap();

    let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
    assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));
    let bad_utf8: Vec<u8> = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().into_bytes(),
        vec![0x9f, 0xf0, 0xa6, 0x80]
    );

    fn get_name() -> Cow<'static, str> {
        std::env::var("USER")
            .map(|v| v.into())
            .unwrap_or("whoever you are".into())
    }
    println!("Greetings, {}!", get_name());

    fn get_title() -> Option<&'static str> {
        Some("Administrator")
    }

    let mut name = get_name();
    if let Some(title) = get_title() {
        write!(name.to_mut(), " {}", title).unwrap();
    }
    println!("Greetings, {}!", name);

    let mut map = HashMap::new();
    map.insert("foo", 1);
    map.insert("bar", 2);
    println!("{:#?}", map);

    let original = Rc::new("mazurka".to_string());
    let cloned = original.clone();
    let impostor = Rc::new("mazurka".to_string());
    println!("text: {}, {}, {}", original, cloned, impostor);
    println!("pointers: {:p}, {:p}, {:p}", original, cloned, impostor);

    assert_eq!(
        format!(
            "{mode} {2} {} {}",
            "people",
            "eater",
            "purple",
            mode = "flying"
        ),
        "flying purple people eater"
    );

    // impl fmt::Display for Complex<f64> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         let im_sign = if self.im < 0.0 { '-' } else { '+' };
    //         write!(f, "{} {} {}i", self.re, im_sign, f64::abs(self.im))
    //     }
    // }

    // impl fmt::Display for Complex<f64> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         let (re, im) = (self.re, self.im);
    //         if f.alternate() {
    //             let abs = f64::sqrt(re * re + im * im);
    //             let angle = f64::atan2(im, re) / std::f64::consts::PI * 180.0;
    //             write!(f, "{:.2}∠{:.2}", abs, angle)
    //         } else {
    //             let im_sign = if im < 0.0 { '-' } else { '+' };
    //             write!(f, "{} {} {}i", re, im_sign, f64::abs(im))
    //         }
    //     }
    // }

    fn logging_enabled() -> bool {
        true
    }
    fn write_log_entry(entry: std::fmt::Arguments) {
        if logging_enabled() {
            let mut log_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("log-file-name")
                .expect("failed to open log file");

            log_file.write_fmt(entry).expect("failed to write to log");
        }
    }
}
