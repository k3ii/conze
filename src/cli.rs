use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("conze")
        .subcommand(
            Command::new("bridge")
                .aliases(["pond", "pont", "pon", "puente"])
                .arg(Arg::new("month").short('m').long("month").aliases(["mois"]))
                .arg(Arg::new("year").short('y').long("year").aliases(["lanner"]))
                .arg(
                    Arg::new("country")
                        .long("country")
                        .short('c')
                        .help("Specify country (e.g., MU for Mauritius, ZA for South Africa)"),
                ),
        )
        .subcommand(
            Command::new("calendar")
                .aliases(["cal"])
                .arg(Arg::new("month").short('m').long("month").aliases(["mois"]))
                .arg(Arg::new("year").short('y').long("year").aliases(["lanner"]))
                .arg(
                    Arg::new("compare")
                        .long("compare")
                        .short('c')
                        .aliases(["cmp", "cpm"])
                        .help("Compare holidays with another country (e.g., ZA for South Africa)"),
                ),
        )
        .subcommand(
            Command::new("config")
                .arg(
                    Arg::new("default-country")
                        .long("default-country")
                        .value_name("COUNTRY_CODE")
                        .ignore_case(true)
                        .help("Sets the default country"),
                )
                .subcommand(Command::new("show").about("Displays the current configuration")),
        )
        .subcommand(
            Command::new("list")
                .about("List holidays for a specific country and year")
                .arg(
                    Arg::new("country")
                        .long("country")
                        .short('c')
                        .help("Specify country (e.g., MU for Mauritius, ZA for South Africa)"),
                )
                .arg(
                    Arg::new("year")
                        .short('y')
                        .long("year")
                        .help("Specify the year"),
                ),
        )
}
