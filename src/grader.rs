use crate::exam::{self, Attempt, GradedAnswer};
use colored::Colorize;
use std::path::PathBuf;

fn exams_dir() -> PathBuf {
    PathBuf::from("exams")
}

fn data_dir() -> PathBuf {
    PathBuf::from("data")
}

pub fn grade(exam_id: &str, attempt: Option<&str>, score_only: bool, missed_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let attempt_data = load_attempt(exam_id, attempt)?;
    let answer_key = exam::load_answer_key(&exams_dir().join(exam_id))?;
    let questions = exam::load_questions(&exams_dir().join(exam_id))?;

    let mut graded = Vec::new();
    let mut correct_count = 0;

    for ua in &attempt_data.answers {
        let key = format!("Q{}", ua.question);
        let entry = answer_key.answers.get(&key);

        let (is_correct, expected, explanation) = match entry {
            Some(e) => {
                let mut user_sorted = ua.selected.clone();
                user_sorted.sort();
                let mut expected_sorted = e.correct.clone();
                expected_sorted.sort();
                (user_sorted == expected_sorted, e.correct.clone(), e.explanation.clone())
            }
            None => (false, vec![], format!("No answer key entry for Q{}", ua.question)),
        };

        if is_correct {
            correct_count += 1;
        }

        graded.push(GradedAnswer {
            question: ua.question,
            correct: is_correct,
            selected: ua.selected.clone(),
            expected,
            reasoning: ua.reasoning.clone(),
            explanation,
        });
    }

    let total = graded.len();
    let pct = if total > 0 { (correct_count as f64 / total as f64) * 100.0 } else { 0.0 };

    println!("\n{}", format!("=== Results: {} ===", exam_id).bold());
    println!("Score: {}/{} ({:.1}%)\n", correct_count, total, pct);

    if !score_only {
        for g in &graded {
            if missed_only && g.correct {
                continue;
            }
            print_graded_question(g, &questions);
        }
        println!("Score: {}/{} ({:.1}%)\n", correct_count, total, pct);
    }

    save_graded(exam_id, &attempt_data.timestamp, &graded)?;
    Ok(())
}

pub fn review(exam_id: &str, missed_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let graded_dir = data_dir().join(exam_id);
    let mut files: Vec<_> = std::fs::read_dir(&graded_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("graded-"))
        .collect();
    files.sort_by_key(|e| e.file_name());

    if files.is_empty() {
        return Err("No graded attempts found. Run `cert-drill grade` first.".into());
    }

    let questions = exam::load_questions(&exams_dir().join(exam_id))?;

    for (i, file) in files.iter().enumerate() {
        let content = std::fs::read_to_string(file.path())?;
        let graded: Vec<GradedAnswer> = toml::from_str::<GradedWrapper>(&content)?.results;
        let correct = graded.iter().filter(|g| g.correct).count();
        let total = graded.len();
        let pct = if total > 0 { (correct as f64 / total as f64) * 100.0 } else { 0.0 };

        // Extract timestamp from filename: graded-<timestamp>.toml
        let fname = file.file_name();
        let name = fname.to_string_lossy();
        let ts = name.strip_prefix("graded-").and_then(|s| s.strip_suffix(".toml")).unwrap_or(&name);

        println!("{}", format!("=== Attempt {} — {} — {}/{} ({:.1}%) ===", i + 1, ts, correct, total, pct).bold());
        println!();

        for g in &graded {
            if missed_only && g.correct {
                continue;
            }
            print_graded_question(g, &questions);
        }
    }
    Ok(())
}

fn print_graded_question(g: &GradedAnswer, questions: &[exam::Question]) {
    let q = questions.iter().find(|q| q.number == g.question);
    let icon = if g.correct { "✅" } else { "❌" };

    println!("{} {}", icon, format!("Q{}", g.question).bold());

    if let Some(q) = q {
        println!("{}\n", q.text);
        for (label, text) in &q.choices {
            let is_correct = g.expected.contains(&label.to_string());
            let is_selected = g.selected.contains(&label.to_string());
            let marker = match (is_selected, is_correct) {
                (true, true) => " ✓ (your answer, correct)".green().to_string(),
                (true, false) => " ✗ (your answer)".red().to_string(),
                (false, true) => " ← correct".green().to_string(),
                (false, false) => String::new(),
            };
            println!("  {}) {}{}", label, text, marker);
        }
    }

    if g.correct {
        if !g.reasoning.is_empty() {
            println!("  {}: {}", "Reasoning".dimmed(), g.reasoning);
        }
        println!("  {}\n", "Nice.".green());
    } else {
        if !g.reasoning.is_empty() {
            println!("  {}: {}", "Your reasoning".dimmed(), g.reasoning);
        }
        println!("  {}: {}", "Explanation".yellow(), g.explanation);
        println!();
    }
}

#[derive(serde::Deserialize)]
struct GradedWrapper {
    results: Vec<GradedAnswer>,
}

fn save_graded(exam_id: &str, timestamp: &str, graded: &[GradedAnswer]) -> Result<(), Box<dyn std::error::Error>> {
    let dir = data_dir().join(exam_id);
    std::fs::create_dir_all(&dir)?;
    let filename = format!("graded-{}.toml", timestamp.replace(':', "-"));
    let wrapper = std::collections::HashMap::from([("results", graded)]);
    let content = format!("# Graded results for {} at {}\n\n{}", exam_id, timestamp,
        toml::to_string_pretty(&wrapper)?);
    std::fs::write(dir.join(filename), content)?;
    Ok(())
}

fn find_latest_file(dir: &PathBuf, prefix: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut files: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with(prefix))
        .collect();
    files.sort_by_key(|e| e.file_name());
    files.last()
        .map(|e| e.path())
        .ok_or_else(|| "No matching files found".into())
}

pub fn load_attempt(exam_id: &str, attempt: Option<&str>) -> Result<Attempt, Box<dyn std::error::Error>> {
    let dir = data_dir().join(exam_id);
    let path = match attempt {
        Some("latest") | None => find_latest_file(&dir, "attempt-")?,
        Some(ts) => dir.join(format!("attempt-{}.toml", ts.replace(':', "-"))),
    };
    let content = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

pub fn export(exam_id: &str, attempt: Option<&str>, ai_context: bool, missed_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    let attempt_data = load_attempt(exam_id, attempt)?;
    let answer_key = exam::load_answer_key(&exams_dir().join(exam_id))?;
    let questions = exam::load_questions(&exams_dir().join(exam_id))?;

    let graded = grade_attempt(&attempt_data, &answer_key);
    let correct_count = graded.iter().filter(|g| g.correct).count();
    let total = graded.len();
    let pct = if total > 0 { (correct_count as f64 / total as f64) * 100.0 } else { 0.0 };

    if ai_context {
        println!("I just took a practice exam for {} and scored {}/{} ({:.1}%).", exam_id, correct_count, total, pct);
        println!("Below are the questions I got wrong along with my reasoning.");
        println!("Please review my thought process, identify misconceptions, and help me understand what I got wrong.\n");
    } else {
        println!("# {} — Results", exam_id);
        println!("Score: {}/{} ({:.1}%)\n", correct_count, total, pct);
    }

    for g in &graded {
        if missed_only && g.correct { continue; }
        let q = questions.iter().find(|q| q.number == g.question);
        let icon = if g.correct { "✅" } else { "❌" };

        if ai_context && g.correct { continue; } // AI context only shows misses

        println!("{} Q{}", icon, g.question);
        if let Some(q) = q {
            println!("{}\n", q.text);
            for (label, text) in &q.choices {
                let marker = if g.expected.contains(&label.to_string()) { " ← correct" } else { "" };
                let mine = if g.selected.contains(&label.to_string()) { " (my answer)" } else { "" };
                println!("  {}) {}{}{}", label, text, mine, marker);
            }
        }
        println!("  Answer: {}  Correct: {}", g.selected.join(", "), g.expected.join(", "));
        if !g.reasoning.is_empty() {
            println!("  My reasoning: {}", g.reasoning);
        }
        if !g.correct {
            println!("  Explanation: {}", g.explanation);
        }
        println!();
    }

    Ok(())
}

fn grade_attempt(attempt: &Attempt, answer_key: &exam::AnswerKey) -> Vec<GradedAnswer> {
    let mut graded = Vec::new();
    for ua in &attempt.answers {
        let key = format!("Q{}", ua.question);
        let entry = answer_key.answers.get(&key);
        let (is_correct, expected, explanation) = match entry {
            Some(e) => {
                let mut user_sorted = ua.selected.clone();
                user_sorted.sort();
                let mut expected_sorted = e.correct.clone();
                expected_sorted.sort();
                (user_sorted == expected_sorted, e.correct.clone(), e.explanation.clone())
            }
            None => (false, vec![], format!("No answer key entry for Q{}", ua.question)),
        };
        graded.push(GradedAnswer {
            question: ua.question,
            correct: is_correct,
            selected: ua.selected.clone(),
            expected,
            reasoning: ua.reasoning.clone(),
            explanation,
        });
    }
    graded
}
