use crate::exam::{self, Attempt, UserAnswer};
use crate::grader;
use chrono::Utc;
use colored::Colorize;
use dialoguer::Input;
use rand::seq::SliceRandom;
use std::path::{Path, PathBuf};

fn exams_dir() -> PathBuf {
    PathBuf::from("exams")
}

fn data_dir() -> PathBuf {
    PathBuf::from("data")
}

pub fn load_exam(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src = Path::new(path);
    let meta = exam::load_meta(src)?;
    let dest = exams_dir().join(&meta.id);
    if dest.exists() {
        println!("Exam '{}' already loaded, updating...", meta.id);
    }
    copy_dir(src, &dest)?;
    println!("{} Loaded exam: {} ({})", "✓".green(), meta.title, meta.id);
    Ok(())
}

pub fn list_exams() -> Result<(), Box<dyn std::error::Error>> {
    let dir = exams_dir();
    if !dir.exists() {
        println!("No exams loaded. Use `cert-drill load <path>` to add one.");
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Ok(meta) = exam::load_meta(&entry.path()) {
                println!("  {} — {}", meta.id.bold(), meta.title);
            }
        }
    }
    Ok(())
}

struct AnswerDraft {
    selected: Vec<String>,
    reasoning: String,
}

impl Default for AnswerDraft {
    fn default() -> Self {
        Self { selected: vec![], reasoning: String::new() }
    }
}

fn show_help(cram: bool) {
    println!("\n{}", "Commands:".bold());
    println!("  {}    — go to next question", ">".cyan());
    println!("  {}    — go to previous question", "<".cyan());
    println!("  {}  — jump to question N", "#N".cyan());
    println!("  {}    — submit all answers", "!s".cyan());
    println!("  {}    — quit without saving", "!q".cyan());
    println!("  {}    — show this help", "?".cyan());
    println!();
    println!("  To answer, just type the letter: {}", "A".cyan());
    println!("  Multi-select: {}", "A,C".cyan());
    if !cram {
        println!("  Tags in reasoning: {}", "#shotInDark #unsure #confident".dimmed());
    }
    println!();
}

pub fn take(exam_id: &str, range: Option<&str>, random: bool, domain: Option<&str>, cram: bool) -> Result<(), Box<dyn std::error::Error>> {
    let exam_path = exams_dir().join(exam_id);
    if !exam_path.exists() {
        return Err(format!("Exam '{}' not found. Run `cert-drill list` to see available exams.", exam_id).into());
    }

    let meta = exam::load_meta(&exam_path)?;
    let questions = exam::load_questions(&exam_path)?;
    let (start, end) = parse_range(range, questions.len() as u32)?;

    let mut filtered: Vec<_> = questions.iter()
        .filter(|q| q.number >= start && q.number <= end)
        .filter(|q| match domain {
            Some(d) => q.domain.to_lowercase().contains(&d.to_lowercase()),
            None => true,
        })
        .collect();

    if filtered.is_empty() {
        return Err("No questions match that range/domain.".into());
    }

    if random {
        let mut rng = rand::thread_rng();
        filtered.shuffle(&mut rng);
    }

    let mode_label = if cram { " [CRAM]".yellow().to_string() } else { String::new() };
    println!("\n{}{}", format!("=== {} ===", meta.title).bold(), mode_label);
    println!("Questions {}-{}  (type {} for help)\n", start, end, "?".cyan());

    let mut drafts: Vec<AnswerDraft> = filtered.iter().map(|_| AnswerDraft::default()).collect();
    let mut idx: usize = 0;

    loop {
        let q = filtered[idx];
        let draft = &drafts[idx];

        println!("{}", format!("Q{} ({}/{})", q.number, idx + 1, filtered.len()).bold());
        println!("{}\n", q.text);
        for (label, text) in &q.choices {
            println!("  {}) {}", label, text);
        }

        if !draft.selected.is_empty() || !draft.reasoning.is_empty() {
            println!();
            if !draft.selected.is_empty() {
                println!("  {}: {}", "Answer".yellow(), draft.selected.join(", "));
            }
            if !draft.reasoning.is_empty() {
                println!("  {}: {}", "Notes".yellow(), draft.reasoning);
            }
        }
        println!();

        let input: String = Input::new()
            .with_prompt(">")
            .allow_empty(true)
            .interact_text()?;

        let trimmed = input.trim();

        match trimmed {
            "?" => { show_help(cram); continue; }
            ">" | "" => {
                if idx + 1 < filtered.len() { idx += 1; }
                else { println!("{}", "Last question. Type !s to submit.".yellow()); }
                continue;
            }
            "<" => {
                if idx > 0 { idx -= 1; } else { println!("{}", "First question.".yellow()); }
                continue;
            }
            "!s" => break,
            "!q" => { println!("Quit without saving."); return Ok(()); }
            _ => {}
        }

        if trimmed.starts_with('#') && trimmed.len() > 1 && trimmed[1..].chars().all(|c| c.is_ascii_digit()) {
            let target: u32 = trimmed[1..].parse().unwrap_or(0);
            if let Some(pos) = filtered.iter().position(|q| q.number == target) {
                idx = pos;
            } else {
                println!("{}", format!("Q{} not in range {}-{}", target, start, end).red());
            }
            continue;
        }

        let letters: Vec<String> = trimmed
            .split(|c: char| c == ',' || c.is_whitespace())
            .map(|s| s.trim().to_uppercase())
            .filter(|s| s.len() == 1 && s.chars().next().unwrap().is_ascii_uppercase())
            .collect();

        if letters.is_empty() {
            println!("{}", "Type a letter to answer (A-D), or ? for help.".red());
            continue;
        }

        drafts[idx].selected = letters;

        // In cram mode, skip reasoning
        if !cram {
            let reasoning: String = Input::new()
                .with_prompt("Reasoning")
                .allow_empty(true)
                .interact_text()?;
            drafts[idx].reasoning = reasoning;
        }

        println!();

        if idx + 1 < filtered.len() {
            idx += 1;
        } else {
            println!("{}", "Last question answered. Type !s to submit.".yellow());
        }
    }

    let unanswered: Vec<_> = filtered.iter().zip(drafts.iter())
        .filter(|(_, d)| d.selected.is_empty())
        .map(|(q, _)| q.number)
        .collect();

    if !unanswered.is_empty() {
        println!("{}", format!("{} unanswered: Q{}", unanswered.len(),
            unanswered.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", Q")).yellow());
        let confirm: String = Input::new()
            .with_prompt("Submit anyway? (y/n)")
            .interact_text()?;
        if confirm.trim().to_lowercase() != "y" {
            return Ok(());
        }
    }

    let user_answers: Vec<UserAnswer> = filtered.iter().zip(drafts.into_iter())
        .map(|(q, d)| UserAnswer {
            question: q.number,
            selected: d.selected,
            reasoning: d.reasoning,
            confidence: None,
        })
        .collect();

    let attempt = Attempt {
        exam_id: exam_id.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        range: [start, end],
        answers: user_answers,
    };

    save_attempt(&attempt)?;
    println!("\n{} Saved.\n", "✓".green());

    // Post-submit flow
    post_submit(exam_id)?;
    Ok(())
}

fn post_submit(exam_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Always grade and show score
    grader::grade(exam_id, Some("latest"), true, false)?;

    loop {
        println!("{}", "What next?".bold());
        println!("  {}  — show all results", "1".cyan());
        println!("  {}  — show only missed", "2".cyan());
        println!("  {}  — export as markdown", "3".cyan());
        println!("  {}  — export as AI review context", "4".cyan());
        println!("  {}  — done", "5".cyan());
        println!();

        let input: String = Input::new()
            .with_prompt(">")
            .default("5".into())
            .interact_text()?;

        match input.trim() {
            "1" => grader::grade(exam_id, Some("latest"), false, false)?,
            "2" => grader::grade(exam_id, Some("latest"), false, true)?,
            "3" => grader::export(exam_id, Some("latest"), false, false)?,
            "4" => grader::export(exam_id, Some("latest"), true, true)?,
            _ => break,
        }
    }
    Ok(())
}

fn save_attempt(attempt: &Attempt) -> Result<(), Box<dyn std::error::Error>> {
    let dir = data_dir().join(&attempt.exam_id);
    std::fs::create_dir_all(&dir)?;
    let filename = format!("attempt-{}.toml", attempt.timestamp.replace(':', "-"));
    let content = toml::to_string_pretty(attempt)?;
    std::fs::write(dir.join(filename), content)?;
    Ok(())
}

fn parse_range(range: Option<&str>, max: u32) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    match range {
        Some(r) => {
            let parts: Vec<&str> = r.split('-').collect();
            if parts.len() != 2 {
                return Err("Range must be in format 'start-end' (e.g., 1-20)".into());
            }
            Ok((parts[0].parse()?, parts[1].parse()?))
        }
        None => Ok((1, max)),
    }
}

fn copy_dir(src: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(dest)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let target = dest.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
