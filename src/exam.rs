use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Exam pack metadata loaded from exam.toml
#[derive(Debug, Deserialize, Serialize)]
pub struct ExamMeta {
    pub id: String,
    pub title: String,
    pub passing_score: f64,
    pub domains: Vec<Domain>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub name: String,
    pub weight: f64,
    pub question_range: [u32; 2],
}

/// A single question parsed from questions.md
#[derive(Debug, Clone)]
pub struct Question {
    pub number: u32,
    pub domain: String,
    pub text: String,
    pub choices: Vec<(char, String)>,
    pub select_count: u32, // 1 for single-select, 2+ for multi-select
}

/// Answer key entry from answers.toml
#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerKey {
    pub answers: HashMap<String, AnswerEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerEntry {
    pub correct: Vec<String>,
    pub explanation: String,
}

/// A user's answer to a single question
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserAnswer {
    pub question: u32,
    pub selected: Vec<String>,
    pub reasoning: String,
    pub confidence: Option<u8>, // 1-5 self-rated confidence
}

/// A complete attempt (one quiz session)
#[derive(Debug, Deserialize, Serialize)]
pub struct Attempt {
    pub exam_id: String,
    pub timestamp: String,
    pub range: [u32; 2],
    pub answers: Vec<UserAnswer>,
}

/// Graded result for a single question
#[derive(Debug, Serialize, Deserialize)]
pub struct GradedAnswer {
    pub question: u32,
    pub correct: bool,
    pub selected: Vec<String>,
    pub expected: Vec<String>,
    pub reasoning: String,
    pub explanation: String,
}

/// A flashcard (front/back)
#[derive(Debug, Clone)]
pub struct Flashcard {
    pub front: String,
    pub back: String,
    pub domain: String,
}

/// Load exam metadata from exam.toml
pub fn load_meta(exam_dir: &Path) -> Result<ExamMeta, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(exam_dir.join("exam.toml"))?;
    Ok(toml::from_str(&content)?)
}

/// Load answer key from answers.toml
pub fn load_answer_key(exam_dir: &Path) -> Result<AnswerKey, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(exam_dir.join("answers.toml"))?;
    Ok(toml::from_str(&content)?)
}

/// Parse questions from questions.md
pub fn load_questions(exam_dir: &Path) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(exam_dir.join("questions.md"))?;
    parse_questions(&content)
}

fn parse_questions(content: &str) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let mut questions = Vec::new();
    let mut current_domain = String::new();
    let mut current_q: Option<QuestionBuilder> = None;

    for line in content.lines() {
        // Domain headers: ## Domain 1: Design Secure Architectures
        if line.starts_with("## ") {
            current_domain = line.trim_start_matches("## ").to_string();
            continue;
        }

        // Question start: **Q1.** text...
        if line.starts_with("**Q") {
            if let Some(builder) = current_q.take() {
                questions.push(builder.build(&current_domain));
            }
            let rest = line.trim_start_matches("**Q");
            if let Some(dot_pos) = rest.find(".**") {
                let num: u32 = rest[..dot_pos].parse().unwrap_or(0);
                let text = rest[dot_pos + 4..].trim().to_string();
                let select_count = if text.contains("Select TWO") || text.contains("Select THREE") {
                    if text.contains("TWO") { 2 } else { 3 }
                } else {
                    1
                };
                current_q = Some(QuestionBuilder {
                    number: num,
                    text,
                    choices: Vec::new(),
                    select_count,
                });
            }
            continue;
        }

        // Choice lines: A) text, B) text, etc.
        if let Some(ref mut builder) = current_q {
            let trimmed = line.trim();
            if trimmed.len() >= 3 && trimmed.as_bytes()[1] == b')' && trimmed.as_bytes()[0].is_ascii_uppercase() {
                let label = trimmed.as_bytes()[0] as char;
                let text = trimmed[3..].trim().to_string();
                builder.choices.push((label, text));
            } else if !trimmed.is_empty() && !trimmed.starts_with('#') && !trimmed.starts_with("---") {
                // Continuation of question text
                if builder.choices.is_empty() {
                    builder.text.push(' ');
                    builder.text.push_str(trimmed);
                }
            }
        }
    }

    if let Some(builder) = current_q {
        questions.push(builder.build(&current_domain));
    }

    Ok(questions)
}

/// Parse flashcards from flashcards.md (or ## Flashcards section)
/// Format: Q: front text / A: back text (one per block, separated by blank lines)
pub fn load_flashcards(exam_dir: &Path) -> Result<Vec<Flashcard>, Box<dyn std::error::Error>> {
    let fc_path = exam_dir.join("flashcards.md");
    if !fc_path.exists() {
        return Ok(vec![]);
    }
    let content = std::fs::read_to_string(fc_path)?;
    let mut cards = Vec::new();
    let mut domain = String::new();
    let mut front = String::new();
    let mut back = String::new();
    let mut in_back = false;

    for line in content.lines() {
        if line.starts_with("## ") {
            if !front.is_empty() && !back.is_empty() {
                cards.push(Flashcard { front: front.trim().to_string(), back: back.trim().to_string(), domain: domain.clone() });
                front.clear(); back.clear();
            }
            domain = line.trim_start_matches("## ").to_string();
            in_back = false;
            continue;
        }
        let trimmed = line.trim();
        if trimmed.starts_with("Q:") || trimmed.starts_with("q:") {
            if !front.is_empty() && !back.is_empty() {
                cards.push(Flashcard { front: front.trim().to_string(), back: back.trim().to_string(), domain: domain.clone() });
            }
            front = trimmed[2..].trim().to_string();
            back.clear();
            in_back = false;
        } else if trimmed.starts_with("A:") || trimmed.starts_with("a:") {
            back = trimmed[2..].trim().to_string();
            in_back = true;
        } else if in_back && !trimmed.is_empty() {
            back.push(' ');
            back.push_str(trimmed);
        }
    }
    if !front.is_empty() && !back.is_empty() {
        cards.push(Flashcard { front: front.trim().to_string(), back: back.trim().to_string(), domain: domain.clone() });
    }
    Ok(cards)
}

struct QuestionBuilder {
    number: u32,
    text: String,
    choices: Vec<(char, String)>,
    select_count: u32,
}

impl QuestionBuilder {
    fn build(self, domain: &str) -> Question {
        Question {
            number: self.number,
            domain: domain.to_string(),
            text: self.text,
            choices: self.choices,
            select_count: self.select_count,
        }
    }
}
