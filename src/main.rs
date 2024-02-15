mod duplicates;

use std::process::exit;

use clap::{CommandFactory, Parser, Subcommand};
use clap_stdin::FileOrStdin;
use duplicates::find_duplicates;
use srtlib::Subtitles;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check if a SRT file has consecutive repeated subtitles (possibly caused by Whisper hallucinations)
    Check {
        /// The SRT file
        #[arg(default_value = "-")]
        input: FileOrStdin,
    },

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: clap_complete_command::Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { input } => {
            let input = input.contents().expect("reading input should not fail");
            let subtitles = Subtitles::parse_from_str(input).expect("parsing SRT should not fail");
            let subtitle_texts = subtitles
                .into_iter()
                .map(|subtitle| subtitle.text)
                .collect::<Vec<_>>();
            let result = find_duplicates(subtitle_texts);
            if result.count > 0 {
                println!(
                    "Found {} repeated sequences of subtitles length {}: {:?}",
                    result.count, result.sequence.len(), result.sequence
                );
                exit(1);
            } else {
                println!("No repeated sequences of subtitles found");
            }
        }

        Commands::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
        }
    }
}
