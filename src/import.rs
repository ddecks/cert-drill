use crate::exam::{Attempt, UserAnswer};
use chrono::Utc;
use colored::Colorize;
use std::path::PathBuf;

fn data_dir() -> PathBuf {
    PathBuf::from("data")
}

/// Import answers from a markdown file.
///
/// Supported formats:
///   1. D - reasoning here #tag1 #tag2
///   1. D reasoning here
///   1. A,C - reasoning
///   1) D - reasoning
///   Q1. D - reasoning
pub fn import_answers(exam_id: &str, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file)?;
    let answers = parse_answer_sheet(&content)?;

    if answers.is_empty() {
        return Err("No answers found. Expected format: '1. D - reasoning here'".into());
    }

    let min_q = answers.iter().map(|a| a.question).min().unwrap_or(1);
    let max_q = answers.iter().map(|a| a.question).max().unwrap_or(1);

    let attempt = Attempt {
        exam_id: exam_id.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        range: [min_q, max_q],
        answers,
    };

    let dir = data_dir().join(exam_id);
    std::fs::create_dir_all(&dir)?;
    let filename = format!("attempt-{}.toml", attempt.timestamp.replace(':', "-"));
    let content = toml::to_string_pretty(&attempt)?;
    std::fs::write(dir.join(&filename), content)?;

    println!("{} Imported {} answers (Q{}-Q{}) from {}",
        "✓".green(), max_q - min_q + 1, min_q, max_q, file);
    println!("  Saved to data/{}/{}", exam_id, filename);
    println!("  Run `cert-drill grade {} --attempt latest` to see results.", exam_id);
    Ok(())
}

fn parse_answer_sheet(content: &str) -> Result<Vec<UserAnswer>, Box<dyn std::error::Error>> {
    let mut answers = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines, comments (#), headers, and non-answer lines
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("---")
            || trimmed.starts_with("###")
        {
            continue;
        }

        if let Some(answer) = try_parse_answer_line(trimmed) {
            answers.push(answer);
        }
    }

    Ok(answers)
}

fn try_parse_answer_line(line: &str) -> Option<UserAnswer> {
    // Match patterns like:
    //   1. D - reasoning
    //   1) D - reasoning
    //   Q1. D - reasoning
    //   1. A,C reasoning
    //   1. D
    let line = line.trim_start_matches('Q').trim_start_matches('q');

    // Find the question number at the start
    let num_end = line.find(|c: char| !c.is_ascii_digit())?;
    if num_end == 0 {
        return None;
    }
    let question: u32 = line[..num_end].parse().ok()?;

    // Skip the separator (. or ))
    let rest = line[num_end..].trim_start_matches('.').trim_start_matches(')').trim();
    if rest.is_empty() {
        return None;
    }

    // Extract the answer letter(s) — everything before a dash, space-then-lowercase, or end of line
    let (answer_part, reasoning) = split_answer_reasoning(rest);

    let selected: Vec<String> = answer_part
        .split(|c: char| c == ',' || c.is_whitespace())
        .map(|s| s.trim().to_uppercase())
        .filter(|s| s.len() == 1 && s.chars().next().unwrap().is_ascii_uppercase())
        .collect();

    if selected.is_empty() {
        return None;
    }

    Some(UserAnswer {
        question,
        selected,
        reasoning: reasoning.to_string(),
        confidence: None,
    })
}

fn split_answer_reasoning(s: &str) -> (&str, &str) {
    // Try splitting on " - " first (most common in the user's format)
    if let Some(pos) = s.find(" - ") {
        return (&s[..pos], s[pos + 3..].trim());
    }
    // Try splitting on " — "
    if let Some(pos) = s.find(" — ") {
        return (&s[..pos], s[pos + 5..].trim());
    }
    // Try splitting after the answer letters (first lowercase word or end)
    // Find where answer letters end: after all uppercase letters and commas
    let answer_end = s.find(|c: char| c.is_lowercase() || c == '#')
        .unwrap_or(s.len());

    if answer_end < s.len() {
        (s[..answer_end].trim(), s[answer_end..].trim())
    } else {
        (s, "")
    }
}
