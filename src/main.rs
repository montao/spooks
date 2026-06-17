use std::error::Error;
use std::io::{self, Write};
use std::{env, process};

use getopts::Options;
use rand::seq::SliceRandom;

const DEFAULT_WORD_COUNT: usize = 5;
const SPOOK_LINES: &str = include_str!("spook.lines");

fn word_list() -> Vec<&'static str> {
    SPOOK_LINES
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

fn random_words<R: rand::Rng + ?Sized>(
    rng: &mut R,
    word_count: usize,
) -> Result<Vec<&'static str>, &'static str> {
    let words = word_list();
    if words.is_empty() {
        return Err("spook word list is empty");
    }

    let mut selected = Vec::with_capacity(word_count);
    for _ in 0..word_count {
        if let Some(word) = words.choose(rng) {
            selected.push(*word);
        }
    }

    Ok(selected)
}

fn spook(mut output: impl Write) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let words = random_words(&mut rng, DEFAULT_WORD_COUNT)
        .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;

    writeln!(output, "{}", words.join(" "))?;
    Ok(())
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {}", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(String::as_str).unwrap_or("spooks");

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_usage(program, &opts);
        return Ok(());
    }

    if matches.opt_present("v") {
        print_version();
        return Ok(());
    }

    spook(io::stdout())?;
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("spooks: {}", error);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn word_list_is_not_empty_or_padded() {
        let words = word_list();

        assert!(!words.is_empty());
        assert!(words.iter().all(|word| !word.is_empty()));
        assert!(words.iter().all(|word| *word == word.trim()));
    }

    #[test]
    fn random_words_uses_requested_count() {
        let mut rng = StdRng::seed_from_u64(42);
        let words = random_words(&mut rng, DEFAULT_WORD_COUNT).unwrap();

        assert_eq!(DEFAULT_WORD_COUNT, words.len());
        assert!(words.iter().all(|word| word_list().contains(word)));
    }
}
