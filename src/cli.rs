use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("conze")
        .arg(Arg::new("month").short('m').long("month").aliases(["mois"]))
        .arg(Arg::new("year").short('y').long("year").aliases(["lanner"]))
        .subcommand(
            Command::new("bridge")
                .aliases(["pond", "pont", "pon", "puente"])
                .arg(Arg::new("month").short('m').aliases(["mois"]))
                .arg(Arg::new("year").short('y').aliases(["lanner"]))
                .arg(
                    Arg::new("country")
                        .long("country")
                        .short('c')
                        .help("Specify country (e.g., MU for Mauritius, SA for South Africa)")
                        .default_value("MU"),
                ),
        )
        .subcommand(
            Command::new("calendar")
                .aliases(["cal"])
                .arg(Arg::new("month").short('m').aliases(["mois"]))
                .arg(Arg::new("year").short('y').aliases(["lanner"])),
        )
}
