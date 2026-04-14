use clap::{Parser, Subcommand};

fn get_exam_list() -> String {
    let dir = std::path::Path::new("exams");
    if !dir.exists() {
        return "Available exams: (none — use `cert-drill load <path>` to add one)".to_string();
    }
    let mut exams = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                exams.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }
    if exams.is_empty() {
        "Available exams: (none — use `cert-drill load <path>` to add one)".to_string()
    } else {
        exams.sort();
        format!("Available exams:\n  {}", exams.join("\n  "))
    }
}

#[derive(Parser)]
#[command(name = "cert-drill", about = "Certification exam drill tool with reasoning tracking", after_help = get_exam_list())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Load an exam pack from a directory
    Load {
        /// Path to exam pack directory
        path: String,
    },
    /// Take a quiz session
    Take {
        /// Exam identifier (e.g., aws-saa-c03)
        exam: String,
        /// Question range (e.g., "1-20")
        #[arg(short, long)]
        range: Option<String>,
        /// Randomize question order
        #[arg(long, default_value_t = false)]
        random: bool,
        /// Filter by domain (substring match, e.g., "Secure" or "Domain 1")
        #[arg(short, long)]
        domain: Option<String>,
        /// Cram mode: skip reasoning prompts, just answer letters
        #[arg(short, long, default_value_t = false)]
        cram: bool,
    },
    /// Grade an attempt
    Grade {
        /// Exam identifier
        exam: String,
        /// Attempt to grade ("latest" or timestamp)
        #[arg(short, long)]
        attempt: Option<String>,
        /// Show only the score (no question details)
        #[arg(short, long, default_value_t = false)]
        score_only: bool,
        /// Show only missed questions
        #[arg(short, long, default_value_t = false)]
        missed: bool,
    },
    /// Export graded results as markdown or AI context
    Export {
        /// Exam identifier
        exam: String,
        /// Attempt to export ("latest" or timestamp)
        #[arg(short, long)]
        attempt: Option<String>,
        /// Format output for AI review (adds prompt framing)
        #[arg(long, default_value_t = false)]
        ai_context: bool,
        /// Show only missed questions
        #[arg(short, long, default_value_t = false)]
        missed: bool,
    },
    /// Show progress and weak areas
    Progress {
        /// Exam identifier
        exam: String,
    },
    /// Review questions from past attempts
    Review {
        /// Exam identifier
        exam: String,
        /// Show only missed questions
        #[arg(short, long, default_value_t = false)]
        missed: bool,
    },
    /// Import answers from a markdown file
    Import {
        /// Exam identifier
        exam: String,
        /// Path to markdown answer file
        file: String,
    },
    /// Study flashcards (reveal-and-rate mode)
    Flashcard {
        /// Exam identifier
        exam: String,
        /// Randomize order
        #[arg(long, default_value_t = true)]
        random: bool,
        /// Filter by domain
        #[arg(short, long)]
        domain: Option<String>,
    },
    /// List available exams
    List {
        /// Show domains for all exams
        #[arg(long, default_value_t = false)]
        domains: bool,
        /// Show detailed info for a specific exam
        #[arg(short, long)]
        name: Option<String>,
    },
}
