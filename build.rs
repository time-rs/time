use serde::Deserialize;
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

const INPUT_DIRECTORY: &str = "src/format/languages";
const OUTPUT_FILE: &str = "src/format/language.rs";

#[derive(Deserialize)]
struct Language {
    name: String,
    iso_code: String,
    months: [String; 12],
    abbr_months: [String; 12],
    weekdays: [String; 7],
    abbr_weekdays: [String; 7],
}

fn languages() -> io::Result<Vec<Language>> {
    let mut languages: Vec<Language> = vec![];

    for file in fs::read_dir(INPUT_DIRECTORY)? {
        let mut file = File::open(file?.path())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        languages.push(toml::from_str(&contents)?);
    }

    languages.sort_unstable_by(|a, b| a.iso_code.cmp(&b.iso_code));

    Ok(languages)
}

fn main() -> io::Result<()> {
    let languages = languages()?;

    let mut out_file = File::create(OUTPUT_FILE)?;

    out_file.write(b"\
//! The `Language` struct and its various methods.

// Localizations are sourced from glibc.
// https://sourceware.org/git/?p=glibc.git;a=tree;f=localedata/locales;hb=HEAD

/// Languages used in formatting. Follows [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes).
///
/// Additional languages may be added at any time. Contributions will be
/// accepted by native and highly fluent speakers of any living language.
///
/// All languages must have the following:
/// - Month names
/// - Short month names
/// - Weekday names
/// - Short weekday names
#[non_exhaustive]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {\n\
    ")?;

    // Generate `Language` struct
    for language in &languages {
        writeln!(out_file, "    /// {}", language.name)?;
        writeln!(out_file, "    {},", language.iso_code)?;
    }

    out_file.write(
        b"}

impl Language {
    /// Get the month names for the given language.
    #[inline]
    pub fn month_names(self) -> [&'static str; 12] {
        match self {\n\
    ",
    )?;

    // Month names
    for language in &languages {
        writeln!(out_file, "            Self::{} => [", language.iso_code)?;
        for month in &language.months {
            writeln!(out_file, "                \"{}\",", month.escape_default())?;
        }
        writeln!(out_file, "            ],")?;
    }

    out_file.write(
        b"        }
    }

    /// Get the abbreviated month names for the given language.
    #[inline]
    pub fn short_month_names(self) -> [&'static str; 12] {
        match self {\n\
    ",
    )?;

    // Abbreviated month names
    for language in &languages {
        writeln!(out_file, "            Self::{} => [", language.iso_code)?;
        for month in &language.abbr_months {
            writeln!(out_file, "                \"{}\",", month.escape_default())?;
        }
        writeln!(out_file, "            ],")?;
    }

    out_file.write(
        b"        }
    }

    /// Get the names of days of the week for the given language. Starts with
    /// Monday.
    #[inline]
    pub fn week_days(self) -> [&'static str; 7] {
        match self {\n\
    ",
    )?;

    // Weekdays
    for language in &languages {
        writeln!(out_file, "            Self::{} => [", language.iso_code)?;
        for weekday in &language.weekdays {
            writeln!(
                out_file,
                "                \"{}\",",
                weekday.escape_default()
            )?;
        }
        writeln!(out_file, "            ],")?;
    }

    out_file.write(
        b"        }
    }

    /// Get the abbreviated names of days of the week for the given language.
    /// Starts with Monday.
    #[inline]
    pub fn short_week_days(self) -> [&'static str; 7] {
        match self {\n\
    ",
    )?;

    // Abbreviated weekdays
    for language in &languages {
        writeln!(out_file, "            Self::{} => [", language.iso_code)?;
        for weekday in &language.abbr_weekdays {
            writeln!(
                out_file,
                "                \"{}\",",
                weekday.escape_default()
            )?;
        }
        writeln!(out_file, "            ],")?;
    }

    out_file.write(b"        }\n    }\n}\n")?;

    Ok(())
}
