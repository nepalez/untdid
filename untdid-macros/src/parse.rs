//! The reader of the code list file (UNCL) of a UN/EDIFACT directory.

use crate::prelude::{HashMap, HashSet, ToKebabCase, ToPascalCase};

// The change indicators a line may start with: none, addition, change of an entry,
// change of a name, change of a text, and deletion.
const INDICATORS: &str = " +*#|X";

// The change indicator of a code marked for deletion.
const DELETED: char = 'X';

// The column where the code of an entry starts (one more in a few misaligned lines).
const CODE_COLUMNS: [usize; 2] = [5, 6];

// The column where a wrapped name of a code continues.
const NAME_COLUMN: usize = 11;

// The first column of the description of a code.
const DESCRIPTION_COLUMN: usize = 14;

// The apostrophes the names of codes use: the typewriter one and the typographic one.
const APOSTROPHES: [char; 2] = ['\'', '\u{2019}'];

// The Latin letters with diacritics the names of codes use, and their base letters.
const LETTERS: [(&str, char); 14] = [
    ("ÀÁÂÃÄÅ", 'A'),
    ("àáâãäå", 'a'),
    ("Ç", 'C'),
    ("ç", 'c'),
    ("ÈÉÊË", 'E'),
    ("èéêë", 'e'),
    ("ÌÍÎÏ", 'I'),
    ("ìíîï", 'i'),
    ("Ñ", 'N'),
    ("ñ", 'n'),
    ("ÒÓÔÕÖØ", 'O'),
    ("òóôõöø", 'o'),
    ("ÙÚÛÜ", 'U'),
    ("ùúûü", 'u'),
];

/// A data element of the directory together with its codes.
pub(crate) struct Element {
    pub number: String,
    pub name: String,
    pub description: String,
    pub type_name: String,
    pub feature: String,
    pub codes: Vec<Code>,
}

/// An active code of a data element.
pub(crate) struct Code {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub variant: String,
}

// A code as read from the file, before the deleted codes are dropped.
struct Entry {
    code: String,
    name: String,
    description: Vec<String>,
    deleted: bool,
}

// The part of an element the previous line belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Description,
    Metadata,
    Name,
    CodeDescription,
}

// The element being read, with its entries in the order of the file.
struct Draft {
    number: String,
    name: String,
    description: Vec<String>,
    entries: Vec<Entry>,
}

/// Reads every data element of the file in the order of the file.
pub(crate) fn parse(text: &str) -> Vec<Element> {
    let mut drafts: Vec<Draft> = Vec::new();
    let mut section = None;
    let lines = text
        .lines()
        .enumerate()
        .skip_while(|(_, line)| !line.starts_with("-----"));

    for (index, line) in lines {
        let number = index + 1;
        if line.trim().is_empty() || line.starts_with("-----") {
            section = None;
            continue;
        }
        let indicator = line.chars().next().unwrap_or(' ');
        assert!(
            INDICATORS.contains(indicator),
            "line {number}: unknown change indicator in {line:?}"
        );
        let rest = &line[indicator.len_utf8()..];
        let body = rest.trim_start();
        let column = 1 + rest.len() - body.len();

        if let Some(draft) = header(body, column) {
            drafts.push(draft);
            section = None;
            continue;
        }
        let draft = drafts
            .last_mut()
            .unwrap_or_else(|| panic!("line {number}: {line:?} precedes any data element"));

        if let Some(text) = body.strip_prefix("Desc:") {
            draft.description.push(text.trim().to_owned());
            section = Some(Section::Description);
        } else if let (Some(note), Some(entry)) =
            (body.strip_prefix("Note:"), draft.entries.last_mut())
        {
            entry
                .description
                .push(format!("Note: {}", note.trim()).trim_end().to_owned());
            section = Some(Section::CodeDescription);
        } else if body.starts_with("Repr:") || body.starts_with("Note:") {
            section = Some(Section::Metadata);
        } else if draft.entries.is_empty()
            && matches!(section, Some(Section::Description | Section::Metadata))
            && column > CODE_COLUMNS[1]
        {
            if section == Some(Section::Description) {
                draft.description.push(body.to_owned());
            }
        } else if CODE_COLUMNS.contains(&column) {
            draft.entries.push(entry(body, indicator, number));
            section = Some(Section::Name);
        } else if column == NAME_COLUMN && section == Some(Section::Name) {
            let entry = draft.entries.last_mut().expect("a code precedes its name");
            if !entry.name.ends_with('-') {
                entry.name.push(' ');
            }
            entry.name.push_str(body);
        } else if column >= DESCRIPTION_COLUMN
            && matches!(section, Some(Section::Name | Section::CodeDescription))
        {
            let entry = draft
                .entries
                .last_mut()
                .expect("a code precedes its description");
            entry.description.push(body.to_owned());
            section = Some(Section::CodeDescription);
        } else {
            panic!("line {number}: unexpected layout of {line:?}");
        }
    }

    drafts.into_iter().map(element).collect()
}

// Reads the header of a data element: its number, its name, and its usage indicator.
fn header(body: &str, column: usize) -> Option<Draft> {
    if !(4..=5).contains(&column) {
        return None;
    }
    let (number, rest) = body.split_at_checked(4)?;
    if !number.chars().all(|c| c.is_ascii_digit()) || !rest.starts_with("  ") {
        return None;
    }
    let name = rest
        .trim_end()
        .strip_suffix(|c| c == ']')?
        .trim_end_matches(['B', 'I', 'C'])
        .strip_suffix('[')?
        .trim();
    Some(Draft {
        number: number.to_owned(),
        name: name.to_owned(),
        description: Vec::new(),
        entries: Vec::new(),
    })
}

// Reads the line that opens an entry: its code and the first line of its name.
fn entry(body: &str, indicator: char, number: usize) -> Entry {
    let (code, name) = body
        .split_once("  ")
        .unwrap_or_else(|| panic!("line {number}: the code {body:?} has no name"));
    Entry {
        code: code.to_owned(),
        name: name.trim().to_owned(),
        description: Vec::new(),
        deleted: indicator == DELETED,
    }
}

// Turns a draft into an element, dropping the deleted codes and naming the types.
fn element(draft: Draft) -> Element {
    let base = base_name(&draft.name);
    let codes = variants(&draft.number, draft.entries);
    Element {
        type_name: words(&base).to_pascal_case(),
        feature: words(&base).to_kebab_case(),
        description: draft.description.join(" "),
        number: draft.number,
        name: draft.name,
        codes,
    }
}

// Drops the word "code" from the name of an element, since every element lists codes.
fn base_name(name: &str) -> String {
    if let Some(stem) = name.strip_suffix(" code qualifier") {
        format!("{stem} qualifier")
    } else if let Some(stem) = name.strip_suffix(" code") {
        stem.to_owned()
    } else {
        name.to_owned()
    }
}

// Names the variants of the active codes of an element.
fn variants(number: &str, entries: Vec<Entry>) -> Vec<Code> {
    let entries: Vec<_> = entries.into_iter().filter(|e| !e.deleted).collect();
    let names: Vec<_> = entries.iter().map(|e| variant(&e.name)).collect();
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for name in &names {
        *counts.entry(name.as_str()).or_default() += 1;
    }
    let mut seen = HashSet::new();
    let mut codes = Vec::with_capacity(entries.len());
    for (entry, name) in entries.iter().zip(&names) {
        let variant = if counts[name.as_str()] > 1 {
            format!("{name}{}", entry.code.to_pascal_case())
        } else {
            name.clone()
        };
        assert!(
            seen.insert(variant.clone()),
            "element {number}: the variant `{variant}` of the code `{}` repeats",
            entry.code
        );
        codes.push(Code {
            code: entry.code.clone(),
            name: entry.name.clone(),
            description: (!entry.description.is_empty()).then(|| entry.description.join(" ")),
            variant,
        });
    }
    codes
}

// Builds the variant name of a code from its name, spelling out a leading number.
fn variant(name: &str) -> String {
    let digits: String = name.chars().take_while(char::is_ascii_digit).collect();
    let name = if digits.is_empty() {
        name.to_owned()
    } else {
        let rest = &name[digits.len()..];
        let value: u32 = digits.parse().expect("a leading number fits u32");
        let suffix = ["st", "nd", "rd", "th"]
            .into_iter()
            .find(|suffix| rest.to_ascii_lowercase().starts_with(suffix));
        match suffix {
            Some(suffix) => format!("{} {}", ordinal(value), &rest[suffix.len()..]),
            None => format!("{} {rest}", cardinal(value)),
        }
    };
    let variant = words(&name).to_pascal_case();
    assert!(
        variant.starts_with(|c: char| c.is_ascii_alphabetic()),
        "the name {name:?} gives no valid variant name"
    );
    variant
}

// Drops the apostrophes, which join the words of a possessive, replaces the letters
// with diacritics by their base letters, and every other character but a letter
// or a digit by a space.
fn words(name: &str) -> String {
    name.chars()
        .filter(|c| !APOSTROPHES.contains(c))
        .map(|c| {
            LETTERS
                .iter()
                .find(|(letters, _)| letters.contains(c))
                .map_or(c, |&(_, base)| base)
        })
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect()
}

// Spells out a number in English words.
fn cardinal(value: u32) -> String {
    const ONES: [&str; 20] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    match value {
        0..20 => ONES[value as usize].to_owned(),
        20..100 => match value % 10 {
            0 => TENS[(value / 10) as usize].to_owned(),
            ones => format!("{} {}", TENS[(value / 10) as usize], ONES[ones as usize]),
        },
        100..1000 => match value % 100 {
            0 => format!("{} hundred", ONES[(value / 100) as usize]),
            rest => format!(
                "{} hundred {}",
                ONES[(value / 100) as usize],
                cardinal(rest)
            ),
        },
        _ => match value % 1000 {
            0 => format!("{} thousand", cardinal(value / 1000)),
            rest => format!("{} thousand {}", cardinal(value / 1000), cardinal(rest)),
        },
    }
}

// Spells out an ordinal number in English words.
fn ordinal(value: u32) -> String {
    let cardinal = cardinal(value);
    let (head, last) = cardinal.rsplit_once(' ').unwrap_or(("", &cardinal));
    let last = match last {
        "one" => "first".to_owned(),
        "two" => "second".to_owned(),
        "three" => "third".to_owned(),
        "five" => "fifth".to_owned(),
        "eight" => "eighth".to_owned(),
        "nine" => "ninth".to_owned(),
        "twelve" => "twelfth".to_owned(),
        word if word.ends_with('y') => format!("{}ieth", &word[..word.len() - 1]),
        word => format!("{word}th"),
    };
    if head.is_empty() {
        last
    } else {
        format!("{head} {last}")
    }
}
