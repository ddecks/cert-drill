use crate::exam;
use crate::paths::{data_dir, exams_dir};
use chrono::{DateTime, Duration, Utc};
use colored::Colorize;
use dialoguer::Input;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Persistent rating for a single flashcard, keyed by card front text.
#[derive(Debug, Serialize, Deserialize, Clone)]
struct CardRating {
    /// Number of times rated
    reviews: u32,
    /// Last rating: 1=forgot, 2=fuzzy, 3=got_it
    last_rating: u8,
    /// Current interval in hours until next review
    interval_hours: f64,
    /// When this card is next due
    due: String,
    /// Cumulative counts
    forgot: u32,
    fuzzy: u32,
    got_it: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct RatingsFile {
    cards: HashMap<String, CardRating>,
}

fn ratings_path(exam_id: &str) -> PathBuf {
    data_dir().join(exam_id).join("flashcard-ratings.toml")
}

fn load_ratings(exam_id: &str) -> RatingsFile {
    let path = ratings_path(exam_id);
    if let Ok(content) = std::fs::read_to_string(&path) {
        toml::from_str(&content).unwrap_or_default()
    } else {
        RatingsFile::default()
    }
}

fn save_ratings(exam_id: &str, ratings: &RatingsFile) -> Result<(), Box<dyn std::error::Error>> {
    let path = ratings_path(exam_id);
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(path, toml::to_string_pretty(ratings)?)?;
    Ok(())
}

/// Simple SM-2-inspired interval calculation.
/// forgot  → reset to 10 minutes
/// fuzzy   → interval * 1.2 (min 1 hour)
/// got_it  → interval * 2.5 (min 8 hours)
fn next_interval(current_hours: f64, rating: u8) -> f64 {
    match rating {
        1 => 10.0 / 60.0, // 10 minutes
        2 => (current_hours * 1.2).max(1.0),
        _ => (current_hours * 2.5).max(8.0),
    }
}

/// Sort cards: due/unseen first (by due date), then future cards last.
fn sort_by_due(cards: &mut [(usize, exam::Flashcard)], ratings: &RatingsFile) {
    let now = Utc::now();
    cards.sort_by(|(_, a), (_, b)| {
        let due_a = ratings.cards.get(&a.front)
            .and_then(|r| r.due.parse::<DateTime<Utc>>().ok())
            .unwrap_or(DateTime::<Utc>::MIN_UTC);
        let due_b = ratings.cards.get(&b.front)
            .and_then(|r| r.due.parse::<DateTime<Utc>>().ok())
            .unwrap_or(DateTime::<Utc>::MIN_UTC);
        let a_overdue = due_a <= now;
        let b_overdue = due_b <= now;
        // Due/unseen cards first, then sort by due date ascending
        match (a_overdue, b_overdue) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => due_a.cmp(&due_b),
        }
    });
}

pub fn study(exam_id: &str, random: bool, domain: Option<&str>, progress: bool) -> Result<(), Box<dyn std::error::Error>> {
    let exam_path = exams_dir().join(exam_id);
    if !exam_path.exists() {
        return Err(format!("Exam '{}' not found.", exam_id).into());
    }

    let cards = exam::load_flashcards(&exam_path)?;
    if cards.is_empty() {
        return Err("No flashcards found. Add a flashcards.md file to the exam pack.".into());
    }

    let mut ratings = load_ratings(exam_id);

    if progress {
        return show_progress(&cards, &ratings, exam_id);
    }

    // Index cards for stable ordering, then filter
    let mut indexed: Vec<(usize, exam::Flashcard)> = cards.into_iter().enumerate().collect();
    if let Some(d) = domain {
        indexed.retain(|(_, c)| c.domain.to_lowercase().contains(&d.to_lowercase()));
    }

    if indexed.is_empty() {
        return Err("No flashcards match that domain filter.".into());
    }

    let now = Utc::now();
    let due_count = indexed.iter()
        .filter(|(_, c)| {
            ratings.cards.get(&c.front)
                .and_then(|r| r.due.parse::<DateTime<Utc>>().ok())
                .map(|d| d <= now)
                .unwrap_or(true) // unseen = due
        })
        .count();

    if random {
        let mut rng = rand::thread_rng();
        indexed.shuffle(&mut rng);
    } else {
        sort_by_due(&mut indexed, &ratings);
    }

    println!("\n{}", format!("=== Flashcards: {} ({} cards, {} due) ===", exam_id, indexed.len(), due_count).bold());
    println!("Press {} to reveal, then rate: {} {} {}", "Enter".cyan(), "1=forgot".red(), "2=fuzzy".yellow(), "3=got it".green());
    println!("Type {} to quit\n", "!q".cyan());

    let mut got_it = 0u32;
    let mut fuzzy_count = 0u32;
    let mut forgot_count = 0u32;

    for (i, (_, card)) in indexed.iter().enumerate() {
        if !card.domain.is_empty() {
            println!("{}", format!("[{}]", card.domain).dimmed());
        }

        // Show due status
        if let Some(r) = ratings.cards.get(&card.front) {
            if let Ok(due) = r.due.parse::<DateTime<Utc>>() {
                if due > now {
                    let hours = (due - now).num_minutes() as f64 / 60.0;
                    if hours > 24.0 {
                        print!("{} ", format!("(due in {:.0}d)", hours / 24.0).dimmed());
                    } else {
                        print!("{} ", format!("(due in {:.0}h)", hours).dimmed());
                    }
                } else {
                    print!("{} ", "(due)".yellow());
                }
            }
        } else {
            print!("{} ", "(new)".cyan());
        }

        println!("{} {}", format!("({}/{})", i + 1, indexed.len()).dimmed(), card.front.bold());

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

        let r: u8 = match rating.trim() {
            "1" => { forgot_count += 1; 1 }
            "2" => { fuzzy_count += 1; 2 }
            _ => { got_it += 1; 3 }
        };

        // Update rating
        let entry = ratings.cards.entry(card.front.clone()).or_insert(CardRating {
            reviews: 0,
            last_rating: 3,
            interval_hours: 0.0,
            due: Utc::now().to_rfc3339(),
            forgot: 0,
            fuzzy: 0,
            got_it: 0,
        });
        entry.reviews += 1;
        entry.last_rating = r;
        match r {
            1 => entry.forgot += 1,
            2 => entry.fuzzy += 1,
            _ => entry.got_it += 1,
        }
        let new_interval = next_interval(entry.interval_hours, r);
        entry.interval_hours = new_interval;
        let minutes = (new_interval * 60.0) as i64;
        entry.due = (Utc::now() + Duration::minutes(minutes)).to_rfc3339();

        println!();
    }

    let total = got_it + fuzzy_count + forgot_count;
    if total > 0 {
        println!("{}", "── Session Summary ──".bold());
        println!("  {} got it  {} fuzzy  {} forgot  ({} total)",
            got_it.to_string().green(), fuzzy_count.to_string().yellow(), forgot_count.to_string().red(), total);
        save_ratings(exam_id, &ratings)?;
        println!("  {}", "Ratings saved.".dimmed());
    }

    Ok(())
}

fn show_progress(cards: &[exam::Flashcard], ratings: &RatingsFile, exam_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let total = cards.len();
    let reviewed: Vec<_> = cards.iter().filter_map(|c| ratings.cards.get(&c.front)).collect();
    let unseen = total - reviewed.len();
    let due = cards.iter().filter(|c| {
        ratings.cards.get(&c.front)
            .and_then(|r| r.due.parse::<DateTime<Utc>>().ok())
            .map(|d| d <= now)
            .unwrap_or(true)
    }).count();

    let total_forgot: u32 = reviewed.iter().map(|r| r.forgot).sum();
    let total_fuzzy: u32 = reviewed.iter().map(|r| r.fuzzy).sum();
    let total_got_it: u32 = reviewed.iter().map(|r| r.got_it).sum();
    let total_reviews: u32 = reviewed.iter().map(|r| r.reviews).sum();

    println!("\n{}", format!("=== Flashcard Progress: {} ===", exam_id).bold());
    println!("  Total cards: {}", total);
    println!("  Reviewed:    {} ({} unseen)", reviewed.len(), unseen);
    println!("  Due now:     {}", if due > 0 { due.to_string().yellow().to_string() } else { "0".green().to_string() });
    println!();
    println!("  Total reviews: {}", total_reviews);
    println!("    {} got it  {} fuzzy  {} forgot",
        total_got_it.to_string().green(), total_fuzzy.to_string().yellow(), total_forgot.to_string().red());

    if total_reviews > 0 {
        let pct_got = (total_got_it as f64 / total_reviews as f64) * 100.0;
        println!("    Mastery rate: {:.0}%", pct_got);
    }

    // Per-domain breakdown
    let mut domains: std::collections::BTreeMap<String, (u32, u32, u32, u32, usize)> = std::collections::BTreeMap::new();
    for card in cards {
        let key = if card.domain.is_empty() { "Uncategorized".to_string() } else { card.domain.clone() };
        let entry = domains.entry(key).or_insert((0, 0, 0, 0, 0));
        entry.4 += 1; // total cards in domain
        if let Some(r) = ratings.cards.get(&card.front) {
            entry.0 += r.got_it;
            entry.1 += r.fuzzy;
            entry.2 += r.forgot;
            entry.3 += r.reviews;
        }
    }

    println!("\n  {}", "By Topic:".bold());
    for (domain, (got, fuz, forg, revs, card_count)) in &domains {
        if *revs == 0 {
            println!("    {} ({} cards) — {}", domain, card_count, "not started".dimmed());
        } else {
            let pct = (*got as f64 / *revs as f64) * 100.0;
            println!("    {} ({} cards) — {:.0}% mastery  ({} got, {} fuzzy, {} forgot)",
                domain, card_count, pct,
                got.to_string().green(), fuz.to_string().yellow(), forg.to_string().red());
        }
    }

    println!();
    Ok(())
}
