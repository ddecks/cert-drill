use crate::exam;
use colored::Colorize;
use std::path::PathBuf;

fn exams_dir() -> PathBuf {
    PathBuf::from("exams")
}

fn data_dir() -> PathBuf {
    PathBuf::from("data")
}

pub fn show_progress(exam_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let exam_path = exams_dir().join(exam_id);
    let meta = exam::load_meta(&exam_path)?;
    let data_path = data_dir().join(exam_id);

    if !data_path.exists() {
        println!("No attempts yet for '{}'. Run `cert-drill take {}` to start.", exam_id, exam_id);
        return Ok(());
    }

    // Collect all graded files
    let mut graded_files: Vec<_> = std::fs::read_dir(&data_path)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("graded-"))
        .collect();
    graded_files.sort_by_key(|e| e.file_name());

    println!("\n{}", format!("=== Progress: {} ===", meta.title).bold());
    println!("Passing score: {:.0}%\n", meta.passing_score);

    let mut total_correct = 0u32;
    let mut total_questions = 0u32;
    let mut domain_stats: std::collections::HashMap<String, (u32, u32)> = std::collections::HashMap::new();
    let mut missed_questions: Vec<u32> = Vec::new();

    let questions = exam::load_questions(&exam_path)?;

    for file in &graded_files {
        let content = std::fs::read_to_string(file.path())?;
        let wrapper: GradedWrapper = toml::from_str(&content)?;

        for g in &wrapper.results {
            total_questions += 1;
            let domain = questions.iter()
                .find(|q| q.number == g.question)
                .map(|q| q.domain.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            let entry = domain_stats.entry(domain).or_insert((0, 0));
            entry.1 += 1;

            if g.correct {
                total_correct += 1;
                entry.0 += 1;
            } else {
                missed_questions.push(g.question);
            }
        }
    }

    let overall_pct = if total_questions > 0 {
        (total_correct as f64 / total_questions as f64) * 100.0
    } else {
        0.0
    };

    let status = if overall_pct >= meta.passing_score { "PASSING".green() } else { "BELOW PASSING".red() };
    println!("Overall: {}/{} ({:.1}%) {}\n", total_correct, total_questions, overall_pct, status);

    println!("{}", "By Domain:".bold());
    for domain in &meta.domains {
        if let Some((correct, total)) = domain_stats.get(&domain.name) {
            let pct = (*correct as f64 / *total as f64) * 100.0;
            let bar = progress_bar(pct);
            println!("  {} {:.1}% ({}/{}) {}", bar, pct, correct, total, domain.name);
        }
    }

    if !missed_questions.is_empty() {
        println!("\n{}", "Missed Questions:".bold());
        println!("  Q{}", missed_questions.iter().map(|q| q.to_string()).collect::<Vec<_>>().join(", Q"));
        println!("\n  Run `cert-drill review {} --missed` to drill these.", exam_id);
    }

    Ok(())
}

fn progress_bar(pct: f64) -> String {
    let filled = (pct / 5.0) as usize;
    let empty = 20 - filled.min(20);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

#[derive(serde::Deserialize)]
struct GradedWrapper {
    results: Vec<exam::GradedAnswer>,
}
