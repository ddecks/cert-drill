use crate::exam;
use colored::Colorize;
use dialoguer::Input;
use rand::seq::SliceRandom;
use std::path::PathBuf;

fn exams_dir() -> PathBuf {
    PathBuf::from("exams")
}

pub fn study(exam_id: &str, random: bool, domain: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let exam_path = exams_dir().join(exam_id);
    if !exam_path.exists() {
        return Err(format!("Exam '{}' not found.", exam_id).into());
    }

    let mut cards = exam::load_flashcards(&exam_path)?;
    if cards.is_empty() {
        return Err("No flashcards found. Add a flashcards.md file to the exam pack.".into());
    }

    if let Some(d) = domain {
        cards.retain(|c| c.domain.to_lowercase().contains(&d.to_lowercase()));
    }

    if random {
        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);
    }

    println!("\n{}", format!("=== Flashcards: {} ({} cards) ===", exam_id, cards.len()).bold());
    println!("Press {} to reveal, then rate: {} {} {}", "Enter".cyan(), "1=forgot".red(), "2=fuzzy".yellow(), "3=got it".green());
    println!("Type {} to quit\n", "!q".cyan());

    let mut got_it = 0;
    let mut fuzzy = 0;
    let mut forgot = 0;

    for (i, card) in cards.iter().enumerate() {
        if !card.domain.is_empty() {
            println!("{}", format!("[{}]", card.domain).dimmed());
        }
        println!("{} {}", format!("({}/{})", i + 1, cards.len()).dimmed(), card.front.bold());

        let input: String = Input::new()
            .with_prompt("Press Enter to reveal (or !q)")
            .allow_empty(true)
            .interact_text()?;

        if input.trim() == "!q" {
            break;
        }

        println!("  → {}\n", card.back.green());

        let rating: String = Input::new()
            .with_prompt("1=forgot  2=fuzzy  3=got it")
            .default("3".into())
            .interact_text()?;

        match rating.trim() {
            "1" => forgot += 1,
            "2" => fuzzy += 1,
            _ => got_it += 1,
        }
        println!();
    }

    let total = got_it + fuzzy + forgot;
    if total > 0 {
        println!("{}", "── Session Summary ──".bold());
        println!("  {} got it  {} fuzzy  {} forgot  ({} total)",
            got_it.to_string().green(), fuzzy.to_string().yellow(), forgot.to_string().red(), total);
    }

    Ok(())
}
