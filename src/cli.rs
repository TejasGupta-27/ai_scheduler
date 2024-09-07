use clap::{Arg, Command, ArgAction};

pub fn setup_cli() -> clap::ArgMatches {
    Command::new("AI Process Scheduler")
        .version("1.0")
        .about("AI-powered process scheduling using reinforcement learning")
        .arg(Arg::new("start")
            .long("start")
            .help("Start the scheduler")
            .action(ArgAction::SetTrue))  // Use this for flags that don't take values
        .arg(Arg::new("status")
            .long("status")
            .help("View the system status")
            .action(ArgAction::SetTrue))  // Similarly for `--status`
        .get_matches()
}
