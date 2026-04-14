# CLAUDE.md — AI Context for cert-drill

## What This Project Is
cert-drill is a terminal-based certification exam study tool written in Rust. It presents multiple-choice questions, captures the user's answer AND their reasoning/thought process, then auto-grades against an answer key. It also supports flashcard study mode. The reasoning capture is the core differentiator — it's designed to work alongside AI tutoring sessions where an AI reviews not just correctness but the quality of the user's reasoning.

## Project Structure
```
cert-drill/
├── exams/              # Exam packs (shipped with repo as examples)
│   └── <exam-id>/
│       ├── exam.toml       # Metadata: title, domains, passing score
│       ├── questions.md    # Questions in structured markdown
│       ├── answers.toml    # Answer key with explanations
│       └── flashcards.md   # Optional: flashcards (Q:/A: format)
├── data/               # User state (gitignored, not in repo)
│   └── <exam-id>/
│       ├── attempt-*.toml  # Raw answers + reasoning per session
│       └── graded-*.toml   # Graded results
└── src/                # Rust source
    ├── main.rs         # CLI entry point
    ├── cli.rs          # clap command definitions
    ├── exam.rs         # Data models + markdown question/flashcard parser
    ├── session.rs      # Interactive quiz session + post-submit flow
    ├── grader.rs       # Auto-grading, review, export (markdown + AI context)
    ├── tracker.rs      # Progress tracking + weak area identification
    ├── flashcard.rs    # Reveal-and-rate flashcard session
    ├── import.rs       # Markdown answer sheet importer
    └── render.rs       # Terminal output helpers
```

## Commands
```
cert-drill list [--domains] [--name <id>]     # list exams, optionally with detail
cert-drill take <exam> [--range 1-20] [--random] [--domain "X"] [--cram]
cert-drill grade <exam> [--attempt latest] [--score-only] [--missed]
cert-drill export <exam> [--ai-context] [--missed]
cert-drill progress <exam>
cert-drill review <exam> [--missed]
cert-drill import <exam> <file>
cert-drill flashcard <exam> [--random] [--domain "X"]
cert-drill load <path>
```

## Key Data Formats

### questions.md
```markdown
## Domain Name
**Q1.** Question text here
A) Choice A
B) Choice B
C) Choice C
D) Choice D
```
Multi-select questions include "(Select TWO)" or "(Select THREE)" in the question text.

### answers.toml
```toml
[answers.Q1]
correct = ["B"]
explanation = "Explanation of why B is correct."
```

### flashcards.md
```markdown
## Topic Name
Q: Front of card (term or question)
A: Back of card (definition or answer)
```

### attempt-*.toml (user data)
```toml
exam_id = "aws-saa-c03"
timestamp = "2026-04-14T15:00:00Z"
range = [1, 20]

[[answers]]
question = 1
selected = ["B"]
reasoning = "User's thought process explaining why they chose this answer #confident"
```

### Import format (markdown answer sheets)
```
1. D - reasoning here #shotInDark
5. A,C VPC would encrypt but not sure
```

## How AI Should Use This Context
When a user shares their cert-drill attempt files or asks for study help:
1. The `reasoning` field in each answer is the most valuable data — it reveals misconceptions
2. Tags like `#shotInDark`, `#unsure`, `#confident` in reasoning indicate self-assessed confidence
3. The `--ai-context` export flag generates a pre-formatted prompt with missed questions + reasoning
4. Grade results are in `data/<exam-id>/graded-*.toml`
5. Progress tracking aggregates across all attempts by domain
6. Flashcards are in `exams/<exam-id>/flashcards.md` organized by topic

## Post-Submit Flow
After submitting a quiz, the score is shown automatically, then the user is prompted:
1. Show all results
2. Show only missed
3. Export as markdown
4. Export as AI review context
5. Done

## Design Philosophy
- Exam packs are portable and human-readable (markdown + TOML)
- User reasoning is a first-class data point, not an afterthought
- Auto-grading works without AI, but the data is structured for AI review
- The tool complements AI tutoring — it captures structured data that AI can analyze
- Cram mode available for speed drilling without reasoning capture
- Works on Termux (Android) for mobile study
