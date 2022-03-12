extern crate core;

use json_generator::{create_ints_array, create_objects_array, create_strings_array};
use clap::{arg, ArgMatches, command, Command, ArgEnum, PossibleValue};

#[derive(strum::Display)]
enum Subcommand {
    #[strum(serialize = "generate")]
    Generate
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum GenerateMode {
    Ints,
    Objects,
    Strings,
}

impl GenerateMode {
    pub fn possible_values() -> impl Iterator<Item=PossibleValue<'static>> {
        GenerateMode::value_variants()
            .iter()
            .filter_map(ArgEnum::to_possible_value)
    }
}

impl std::fmt::Display for GenerateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for GenerateMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}


fn main() {
    let generate: String = Subcommand::Generate.to_string();

    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(generate)
                .about("Generation utilities")
                .arg(
                    arg!([DATATYPE])
                        .help("What kind of objects to run the program with")
                        .possible_values(GenerateMode::possible_values())
                )
                .arg(
                    arg!(-f --file <OUTPUT_PATH>)
                        .required(true)
                        .help("File to save generated data")
                )
                .arg(
                    arg!(-l --length <LENGTH>)
                        .required(false)
                        .help("Length of future array")
                        .default_value("10")
                        .validator(|s| s.parse::<usize>())
                )
                .arg(
                    arg!(--from <FROM>)
                        .required(false)
                        .default_value("5")
                        .help("[Except for 'Objects'] Value min bound")
                        .validator(|s| s.parse::<usize>())
                )
                .arg(
                    arg!(--to <TO>)
                        .required(false)
                        .default_value("10")
                        .help("[Except for 'Objects'] Value max bound")
                        .validator(|s| s.parse::<usize>())
                )
                .arg(
                    arg!(-p --pretty)
                        .required(false)
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some((_, sub_matches)) => generate_subcommand(sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

fn generate_subcommand(sub_matches: &ArgMatches) {
    let is_pretty: bool = sub_matches.is_present("pretty");
    let output_path: &str = match sub_matches.value_of("file") {
        Some(f) => f,
        _ => unreachable!("Please, specify output file")
    };

    let length: usize = sub_matches
        .value_of_t("length")
        .expect("default ensures there is always a value");

    match sub_matches
        .value_of_t("DATATYPE")
        .expect("'DATATYPE' is required and parsing will fail if its missing")
    {
        GenerateMode::Objects => {
            create_objects_array(length, output_path, is_pretty);
        }
        GenerateMode::Ints => {
            let from: usize = sub_matches
                .value_of_t("from")
                .expect("'from' is required when using 'ints' subcommand");
            let to: usize = sub_matches
                .value_of_t("to")
                .expect("'to' is required when using 'ints' subcommand");

            create_ints_array(length, (from, to), output_path);
        }
        GenerateMode::Strings => {
            let from: usize = sub_matches
                .value_of_t("from")
                .expect("from' is required when using 'ints' subcommand");
            let to: usize = sub_matches
                .value_of_t("to")
                .expect("'to' is required when using 'ints' subcommand");

            create_strings_array(length, (from, to), output_path);
        }
    }
}
