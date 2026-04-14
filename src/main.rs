mod cli;
mod exam;
mod flashcard;
mod grader;
mod import;
mod render;
mod session;
mod tracker;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Load { path } => session::load_exam(&path),
        Command::Take { exam, range, random, domain, cram } => session::take(&exam, range.as_deref(), random, domain.as_deref(), cram),
        Command::Grade { exam, attempt, score_only, missed } => grader::grade(&exam, attempt.as_deref(), score_only, missed),
        Command::Export { exam, attempt, ai_context, missed } => grader::export(&exam, attempt.as_deref(), ai_context, missed),
        Command::Progress { exam } => tracker::show_progress(&exam),
        Command::Review { exam, missed } => grader::review(&exam, missed),
        Command::Import { exam, file } => import::import_answers(&exam, &file),
        Command::Flashcard { exam, random, domain } => flashcard::study(&exam, random, domain.as_deref()),
        Command::List { domains, name } => session::list_exams(domains, name.as_deref()),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
