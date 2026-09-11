//! Rabbyte 🐇 — because 8 rabbits make 1 Rabbyte.
//!
//! Scans the system dictionary (`/usr/share/dict/words`) for every word ending
//! in "bit" and lovingly converts each one into its byte-aligned equivalent
//! ("Rabbit" -> "Rabbyte", "Habit" -> "Habyte", "Bit" -> "Byte", ...).

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::{Duration, Instant};

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// The one true byte of the rabbit world: 8 rabbits.
const RABBYTE_FACTOR: usize = 8;

/// Where the program looks for its word list (Linux system dictionary).
const DICT_PATH: &str = "/usr/share/dict/words";

fn main() {
    print_banner();

    let dict_path = Path::new(DICT_PATH);
    if !dict_path.is_file() {
        eprintln!("{}", missing_dict_message());
        std::process::exit(1);
    }

    // Quick first pass: count lines so the progress bar knows its total.
    let total = count_lines(dict_path);

    let pb = new_progress_bar(total);

    let started = Instant::now();
    let matches = scan_dictionary(dict_path, &pb);
    let elapsed = started.elapsed();

    pb.finish_and_clear();

    print_summary(total, matches, elapsed);
}

/// A short, silly header so the user knows the rabbits have been released.
fn print_banner() {
    println!("{}", "🐇 RABBYTE 🐇".bright_magenta().bold());
    println!(
        "{}",
        "8 rabbits = 1 Rabbyte. Converting your dictionary, one hop at a time..."
            .dimmed()
    );
    println!();
}

/// A friendly explanation when the Linux dictionary file is not available.
fn missing_dict_message() -> String {
    format!(
        "Oh no, the hutch is empty! Could not find the word list at:\n  {}\n\n\
         This program is Linux-only: it needs the system dictionary installed.\n\n\
         How to grow your rabbit hutch:\n\
         \u{2022} Debian/Ubuntu:  sudo apt install words\n\
         \u{2022} Fedora:        sudo dnf install words\n\
         \u{2022} Arch:          sudo pacman -S words\n\
         \u{2022} Alpine:        sudo apk add words\n\n\
         Then hop back here and try again!",
        DICT_PATH.bold()
    )
}

/// Count the total number of lines in the dictionary (cheap first pass).
fn count_lines(path: &Path) -> u64 {
    let reader = BufReader::new(File::open(path).expect("dictionary file should open after check"));
    reader.lines().map_while(Result::ok).count() as u64
}

/// Build the progress bar: a parade of rabbits fills the bar as words are
/// scanned, with the live match count in the message slot.
fn new_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template("🐰 {bar:30.bright_green} {pos}/{len} ({percent}%) — {msg}")
            .expect("progress bar template should be valid")
            // indicatif requires all glyphs to share one display width, so the
            // bunny lives in the template, not the bar itself.
            .progress_chars("=>-"),
    );
    pb.set_message("0 Rabbytes so far");
    pb.enable_steady_tick(Duration::from_millis(120));
    pb
}

/// Scan the dictionary line by line, advancing the progress bar per line and
/// printing a joke line for every word ending in "bit" (case-insensitive).
/// Lines are emitted through the progress bar so it never gets corrupted.
/// Returns the number of matches (Rabytes) found.
fn scan_dictionary(path: &Path, pb: &ProgressBar) -> usize {
    let reader = BufReader::new(File::open(path).expect("dictionary file should open after check"));

    let mut matches: usize = 0;

    for line in reader.lines().map_while(Result::ok) {
        pb.inc(1);

        if !ends_with_bit(&line) {
            continue;
        }

        matches += 1;

        emit(&pb, render_joke_line(matches, &line));

        pb.set_message(format!(
            "{matches} Rabbyte{} so far",
            if matches == 1 { "" } else { "s" }
        ));
    }

    matches
}

/// Send a joke line out without corrupting the progress bar: through the bar
/// itself when it is visible (the bar suspends, the line scrolls out above
/// it, and the bar redraws right below, untouched), or plainly when the bar
/// is hidden (e.g. output is piped), because indicatif drops lines otherwise.
fn emit(pb: &ProgressBar, line: String) {
    if pb.is_hidden() {
        println!("{line}");
    } else {
        pb.println(line);
    }
}

/// Case-insensitive suffix check: every word ending in "bit" counts
/// ("Bit", "raBBIT", "Habit", "Cubit", ...).
fn ends_with_bit(word: &str) -> bool {
    word.len() >= 3 && word.as_bytes()[word.len() - 3..].eq_ignore_ascii_case(b"bit")
}

/// Turn a word ending in "bit" into its byte-flavored form: the trailing
/// "bit" becomes "byte", preserving the original casing
/// ("Rabbit" -> "Rabbyte", "Habit" -> "Habyte", "Bit" -> "Byte").
fn rabbyte_of(word: &str) -> String {
    let (stem, suffix) = word.split_at(word.len() - 3);
    let mut out = String::with_capacity(word.len() + 1);
    out.push_str(stem);
    // "byte" is one char longer than "bit": each char inherits the casing of
    // its original counterpart, and the extra trailing 'e' follows the style
    // of the last original char ("BIT" -> "BYTE", "bit" -> "byte").
    let extra_upper = suffix.chars().last().is_some_and(|c| c.is_uppercase());
    for (i, new) in "byte".chars().enumerate() {
        let upper = suffix
            .chars()
            .nth(i)
            .map_or(extra_upper, |orig| orig.is_uppercase());
        if upper {
            out.extend(new.to_uppercase());
        } else {
            out.push(new);
        }
    }
    out
}

/// Format one match as `[N] Rabbit =( * 8 )=> Rabbyte` with playful colors:
/// the counter in cyan, the word in green, the math in yellow, the Rabbyte
/// in bright magenta.
fn render_joke_line(count: usize, word: &str) -> String {
    format!(
        "{} {} {} {}",
        format!("[{count}]").cyan(),
        word.green(),
        "=( * 8 )=>".yellow(),
        rabbyte_of(word).bright_magenta().bold(),
    )
}

/// Final report once every word has been scanned.
fn print_summary(total_words: u64, total_matches: usize, elapsed: Duration) {
    println!();
    println!(
        "{}",
        "✨ Scan complete — the hutch has been harvested! ✨"
            .bright_cyan()
            .bold()
    );
    println!(
        "Hopped through {} word{} in {:.1?} and produced {} Rabbyte{} \
         (1 Rabbyte = {} rabbits, obviously).",
        total_words.to_string().bright_green(),
        if total_words == 1 { "" } else { "s" },
        elapsed,
        total_matches.to_string().bright_green(),
        if total_matches == 1 { "" } else { "s" },
        RABBYTE_FACTOR
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix_match_is_case_insensitive() {
        for w in ["bit", "Bit", "BIT", "raBBIT", "Habit", "habIT", "Cubit"] {
            assert!(ends_with_bit(w), "{w} should match");
        }
    }

    #[test]
    fn non_matches_do_not_match() {
        for w in ["rabbiting", "orbitz", "bite", "rabbit ", "", "bi"] {
            assert!(!ends_with_bit(w), "{w:?} should not match");
        }
    }

    #[test]
    fn rabbyte_transformation() {
        assert_eq!(rabbyte_of("Rabbit"), "Rabbyte");
        assert_eq!(rabbyte_of("Habit"), "Habyte");
        assert_eq!(rabbyte_of("Hobbit"), "Hobbyte");
        assert_eq!(rabbyte_of("Bit"), "Byte");
        assert_eq!(rabbyte_of("BIT"), "BYTE");
        assert_eq!(rabbyte_of("raBBIT"), "raBBYTE");
    }

    #[test]
    fn joke_line_format() {
        let line = render_joke_line(2, "Habit");
        for part in ["[2]", "Habit", "=( * 8 )=>", "Habyte"] {
            assert!(line.contains(part), "expected {part:?} in {line:?}");
        }
    }
}
