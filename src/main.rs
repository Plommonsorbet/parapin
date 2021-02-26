//use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use anyhow::{bail, Context, Result};

type Config = HashMap<u32, Vec<char>>;

#[derive(StructOpt, Debug)]
#[structopt(name = "parapin")]
struct Cli {
    #[structopt(short = "c", long, parse(from_os_str))]
    config: Option<PathBuf>,

    input: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ParaPin {
    translate_map: HashMap<char, u32>,
}

fn stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("unable to read from stdin")?;
    Ok(buffer)
}

fn user_input() -> Result<String> {
    println!("Enter text to translate to pin");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("unable to read user input")?;

    Ok(input)
}

fn read_config(path: &Path) -> Result<Config> {
    let contents = fs::read_to_string(path).context("unable to read config file")?;

    Ok(serde_yaml::from_str(&contents).context("unable to parse config as yaml")?)
}

fn default_translate_map() -> HashMap<char, u32> {
    [
        ('a', 0),
        ('b', 0),
        ('c', 1),
        ('d', 1),
        ('e', 2),
        ('f', 2),
        ('g', 3),
        ('h', 3),
        ('i', 4),
        ('j', 4),
        ('k', 4),
        ('l', 5),
        ('m', 5),
        ('n', 5),
        ('o', 6),
        ('p', 6),
        ('q', 6),
        ('r', 7),
        ('s', 7),
        ('t', 7),
        ('u', 8),
        ('v', 8),
        ('w', 8),
        ('x', 9),
        ('y', 9),
        ('z', 9),
    ]
    .iter()
    .cloned()
    .collect()
}

impl ParaPin {
    fn from(config: Config) -> Result<Self> {
        let mut translate_map: HashMap<char, u32> = HashMap::new();

        for (digit, characters) in config {
            if digit > 9 {
                bail!("The translation digits defined in the config can only be 0-9");
            }
            for c in characters {
                if let Some(_duplicate) = translate_map.insert(c, digit) {
                    bail!("The char '{}' was used multiple times in the config", c);
                };
            }
        }

        Ok(ParaPin { translate_map })
    }

    fn translate(&self, c: &char) -> Result<u32> {
        match self.translate_map.get(c) {
            Some(c) => Ok(*c),
            None => bail!("could not lookup character '{}' in the config."),
        }
    }

    fn pin(&self, message: &str) -> Result<String> {
        let mut pin = String::new();

        for c in message.trim().chars() {
            pin.push_str(&self.translate(&c)?.to_string())
        }

        Ok(pin)
    }
}

fn main() -> Result<()> {
    let opt = Cli::from_args();

    let pp = match &opt.config {
        Some(path) => ParaPin::from(read_config(&path)?)?,
        None => ParaPin {
            translate_map: default_translate_map(),
        },
    };

    let pin = match &opt.input {
        Some(input) => match input.as_str() {
            "-" => pp.pin(&stdin()?)?,
            _ => pp.pin(&input)?,
        },
        None => pp.pin(&user_input()?)?,
    };

    println!("pin: {}", pin);

    Ok(())
}
