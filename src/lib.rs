use quick_xml;

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
//    partlist: ,
//    parts: ,
}

impl From<String> for Score {
    fn from(string: String) -> Score {
        let mut reader = quick_xml::Reader::from_str(&string);
        reader.trim_text(true);

        Score { 
            info: ScoreInfo {
                work_title: String::new(),
                composer: String::new(),
                arranger: String::new(),
                lyricist: String::new(),
                margins: Margins {
                    left_margin: 0,
                    right_margin: 0,
                    top_margin: 0,
                    bottom_margin: 0,
                },
                word_font: String::new(),
                lyric_font: String::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
