use std::{collections::HashMap, fs::File, io::Write};

use serde::Serialize;

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

trait WriteHtml {
    fn write_html(&mut self, html: &String) -> std::io::Result<()>;
}

impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &String) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
    let writer = File::create("config.txt")?;
    let mut serializer = serde_json::Serializer::new(writer);
    config.serialize(&mut serializer)?;

    Ok(())
}

pub trait MegaSplicealbe {
    fn splice(&self, other: &dyn MegaSplicealbe) -> Box<dyn MegaSplicealbe>;
}

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}


fn main() {
    assert_eq!('$'.is_emoji(), false);
}
