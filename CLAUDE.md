# CLAUDE.md — AI Context for cert-drill

## What This Project Is
cert-drill is a terminal-based certification exam study tool written in Rust. It presents multiple-choice questions, captures the user's answer AND their reasoning/thought process, then auto-grades against an answer key. The reasoning capture is the core differentiator — it's designed to work alongside AI tutoring sessions where an AI reviews not just correctness but the quality of the user's reasoning.

## Project Structure
```
cert-drill/
├── exams/              # Exam packs (shipped with repo as examples)
│   └── <exam-id>/
│       ├── exam.toml       # Metadata: title, domains, passing score
│       ├── questions.md    # Questions in structured markdown
│       └── answers.toml    # Answer key with explanations
├── data/               # User state (gitignored, not in repo)
│   └── <exam-id>/
│       ├── attempt-*.toml  # Raw answers + reasoning per session
│       └── graded-*.toml   # Graded results
└── src/                # Rust source
    ├── main.rs         # CLI entry point
    ├── cli.rs          # clap command definitions
    ├── exam.rs         # Data models + markdown question parser
    ├── session.rs      # Interactive quiz session
    ├── grader.rs       # Auto-grading + review display
    ├── tracker.rs      # Progress tracking + weak area identification
    └── render.rs       # Terminal output helpers
```

## Key Data Formats

### questions.md
Questions use this markdown structure:
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

[answers.Q5]
correct = ["A", "C"]
explanation = "Multi-select: both A and C are needed because..."
```

### attempt-*.toml (user data)
```toml
exam_id = "aws-saa-c03"
timestamp = "2026-04-14T15:00:00Z"
range = [1, 20]

[[answers]]
question = 1
selected = ["B"]
reasoning = "User's thought process explaining why they chose this answer"
confidence = 3
```

## How AI Should Use This Context
When a user shares their cert-drill attempt files or asks for study help:
1. The `reasoning` field in each answer is the most valuable data — it reveals misconceptions
2. The `confidence` field indicates where the user is guessing vs. certain
3. Low confidence + correct answer = lucky guess, still worth reviewing
4. High confidence + wrong answer = misconception that needs correction
5. Grade results are in `data/<exam-id>/graded-*.toml`
6. Progress tracking aggregates across all attempts

## Building and Running
```bash
cargo build --release
cargo run -- list                           # list available exams
cargo run -- take aws-saa-c03 --range 1-20  # take a quiz
cargo run -- grade aws-saa-c03              # grade latest attempt
cargo run -- progress aws-saa-c03           # view progress
cargo run -- review aws-saa-c03 --missed    # review missed questions
```

## Design Philosophy
- Exam packs are portable and human-readable (markdown + TOML)
- User reasoning is a first-class data point, not an afterthought
- Auto-grading works without AI, but the data is structured for AI review
- The tool complements AI tutoring — it captures structured data that AI can analyze
