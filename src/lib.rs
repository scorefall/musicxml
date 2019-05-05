use roxmltree;

/// Page margins (Unit: 0.0001 millimeters).
#[derive(Debug)]
pub struct Margins {
    left_margin: u32,
    right_margin: u32,
    top_margin: u32,
    bottom_margin: u32,
}

/// 
#[derive(Debug)]
pub struct ScoreInfo {
    work_title: String,
    composer: String,
    arranger: String,
    lyricist: String,
    margins: Margins,
    word_font: String,
    lyric_font: String,
}

/// The MusicXML Score Structure.
#[derive(Debug)]
pub struct Score {
    info: ScoreInfo,
    partlist: ,
    parts: ,
}

impl From<String> for Score {
    fn from(string: String) -> Score {
        match roxmltree::Document::parse(&string) {
            Ok(doc) => print!("{:?}", doc),
            Err(e) => println!("Error: {}.", e),
        }

        Score { }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
