use clap::Parser;
use regex::{Captures, Regex};
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(name = "cen")]
struct Config {
    #[arg(long, default_value = "*")]
    censor_symbol: String,

    #[arg(long, default_value_t = true)]
    match_len: bool,

    #[arg(long, default_value_t = 3)]
    censor_symbol_len: usize,

    #[arg(long, value_delimiter = ',')]
    blacklist: Vec<String>,

    #[arg(long, value_delimiter = ',')]
    file_extensions: Vec<String>,

    #[arg(long)]
    maniac: bool,
}

fn main() -> io::Result<()> {
    let config = Config::parse();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ansi_re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let mut output = String::new();

    let mut last = 0;
    for m in ansi_re.find_iter(&buffer) {
        let text = &buffer[last..m.start()];
        let censored = censor(text, &config);
        output.push_str(&censored);
        output.push_str(m.as_str());
        last = m.end();
    }
    let tail = &buffer[last..];
    output.push_str(&censor(tail, &config));

    let final_output = if config.maniac {
        filter_out_asterisks(&output, &config)
    } else {
        output
    };

    io::stdout().write_all(final_output.as_bytes())?;
    io::stdout().flush()?;
    Ok(())
}

fn filter_out_asterisks(input: &str, config: &Config) -> String {
    input
        .lines()
        .filter(|line| !line.contains(&config.censor_symbol))
        .collect::<Vec<_>>()
        .join("\n")
}

fn censor_keep_colors(
    word: &str,
    censor_symbol: &str,
    match_len: bool,
    censor_symbol_len: usize,
) -> String {
    let ansi_re = Regex::new(r"(\x1b\[[0-9;]*m)").unwrap();

    let parts = ansi_re.split(word).collect::<Vec<_>>();
    let matches = ansi_re.find_iter(word).collect::<Vec<_>>();

    let visible_len: usize = parts.iter().map(|s| s.chars().count()).sum();

    let n = if match_len {
        visible_len
    } else {
        censor_symbol_len
    };
    let censored_symbols = censor_symbol.repeat(n);

    let mut result = String::new();
    let mut censored_chars = censored_symbols.chars();

    for (i, part) in parts.iter().enumerate() {
        for _ in 0..part.chars().count() {
            if let Some(c) = censored_chars.next() {
                result.push(c);
            }
        }
        if i < matches.len() {
            result.push_str(matches[i].as_str());
        }
    }
    result
}

fn censor(input: &str, config: &Config) -> String {
    let word_pattern = config
        .blacklist
        .iter()
        .map(|w| regex::escape(w))
        .collect::<Vec<_>>()
        .join("|");

    let ext_pattern = config
        .file_extensions
        .iter()
        .map(|ext| format!(r"\b\S+\.{}\b", regex::escape(ext)))
        .collect::<Vec<_>>()
        .join("|");

    let combined_pattern = format!("(?i){}|{}", word_pattern, ext_pattern);
    let re = Regex::new(&combined_pattern).unwrap();

    re.replace_all(input, |caps: &Captures| {
        censor_keep_colors(
            &caps[0],
            &config.censor_symbol,
            config.match_len,
            config.censor_symbol_len,
        )
    })
    .to_string()
}
