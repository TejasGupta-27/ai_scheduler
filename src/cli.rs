use clap::{Arg, Command, ArgAction};

pub fn setup_cli() -> clap::ArgMatches {
    Command::new("AI Process Scheduler")
        .version("1.0")
        .about("AI-powered process scheduling using reinforcement learning")
        .subcommand(
            Command::new("start")
                .about("Start the AI scheduler")
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .help("Enable verbose output")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .help("Specify a custom config file")
                        .value_name("FILE"),  // Replaces `takes_value(true)`
                ),
        )
        .subcommand(
            Command::new("status")
                .about("View the system status")
                .arg(
                    Arg::new("detail")
                        .short('d')
                        .long("detail")
                        .help("Show detailed system metrics")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("config")
                .about("Manage the system configuration")
                .arg(
                    Arg::new("set")
                        .long("set")
                        .help("Set a configuration parameter")
                        .value_name("PARAM=VALUE")  // Replaces `takes_value(true)`
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("stop")
                .about("Stop the AI scheduler")
        )
        .get_matches()
}
