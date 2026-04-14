# cert-drill 🎯

A terminal-based certification exam study tool that captures your answers **and your reasoning** — designed to work alongside AI-assisted study sessions.

## Why?

Most quiz tools tell you right/wrong. cert-drill also records *why* you chose each answer and how confident you were. This creates structured data that's useful for:
- Self-review: see your thought process alongside correct explanations
- AI tutoring: share attempt files with an AI that can identify misconceptions in your reasoning
- Progress tracking: scores by domain with weak area identification

## Quick Start

```bash
# Build
cargo build --release

# List available exams
cert-drill list

# Take a quiz (questions 1-20)
cert-drill take aws-saa-c03 --range 1-20

# Grade your latest attempt
cert-drill grade aws-saa-c03 --attempt latest

# View progress by domain
cert-drill progress aws-saa-c03

# Review missed questions
cert-drill review aws-saa-c03 --missed
```

## How It Works

Each question prompts you for:
1. Your answer (A/B/C/D or multi-select)
2. Your reasoning (free text — why you chose this)
3. Your confidence (1-5 scale)

Everything is saved as TOML, graded against the answer key, and tracked over time.

## Creating Exam Packs

An exam pack is a directory with three files:

```
exams/my-cert/
├── exam.toml       # Metadata
├── questions.md    # Questions in structured markdown
└── answers.toml    # Answer key with explanations
```

See `exams/aws-saa-c03/` for a complete example.

### exam.toml
```toml
id = "my-cert"
title = "My Certification Exam"
passing_score = 72.0

[[domains]]
name = "Domain 1: Topic"
weight = 30.0
question_range = [1, 30]
```

### questions.md
```markdown
## Domain 1: Topic

**Q1.** Your question text here?

A) Choice A
B) Choice B
C) Choice C
D) Choice D

**Q2.** A multi-select question? (Select TWO)

A) Choice A
B) Choice B
C) Choice C
D) Choice D
```

### answers.toml
```toml
[answers.Q1]
correct = ["B"]
explanation = "B is correct because..."

[answers.Q2]
correct = ["A", "C"]
explanation = "Both A and C are needed because..."
```

## AI Integration

This project includes `CLAUDE.md` and `llms.txt` context files so AI assistants understand the project structure and data formats. When sharing attempt data with an AI for study help, the reasoning and confidence fields enable targeted feedback on your thought process, not just your answers.

## Included Exam Packs

- `aws-saa-c03` — AWS Solutions Architect Associate (SAA-C03), 100 questions across 4 domains

## License

MIT
