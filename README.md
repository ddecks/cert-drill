# cert-drill 🎯

A terminal-based certification exam study tool that captures your answers **and your reasoning** — designed to work alongside AI-assisted study sessions.

## Why?

Most quiz tools tell you right/wrong. cert-drill also records *why* you chose each answer. This creates structured data that's useful for:

- **Self-review** — see your thought process alongside correct explanations
- **AI tutoring** — export your results in a format optimized for AI review of your reasoning
- **Progress tracking** — scores by domain with weak area identification
- **Flashcards** — reveal-and-rate cards for drilling concepts

## Install

Requires [Rust](https://rustup.rs/). Works on Linux, macOS, Windows, and [Termux](https://termux.dev/) (Android).

```bash
git clone https://github.com/ddecks/cert-drill.git
cd cert-drill
cargo install --path .
```

On Termux:
```bash
pkg install rust
git clone https://github.com/ddecks/cert-drill.git
cd cert-drill
cargo install --path .
```

## Quick Start

```bash
# See what exams are available
cert-drill list

# Take a 20-question quiz
cert-drill take aws-saa-c03 --range 1-20

# After submitting, you'll see your score and a menu to:
#   1 — show all results
#   2 — show only missed
#   3 — export as markdown
#   4 — export as AI review context
#   5 — done

# Study flashcards
cert-drill flashcard aws-saa-c03
```

## Commands

### `cert-drill list`

List available exams.

```bash
cert-drill list                        # one-liner per exam
cert-drill list --domains              # show domains for all exams
cert-drill list --name aws-saa-c03     # full detail: domains, flashcard topics, progress
```

### `cert-drill take <EXAM>`

Interactive quiz session. Answer questions, record your reasoning, then submit.

```bash
cert-drill take aws-saa-c03                          # all questions
cert-drill take aws-saa-c03 --range 1-20             # questions 1-20
cert-drill take aws-saa-c03 --domain "Secure"        # filter by domain
cert-drill take aws-saa-c03 --random                  # randomize order
cert-drill take aws-saa-c03 --cram                    # skip reasoning prompts (speed drill)
cert-drill take aws-saa-c03 --range 57-80 --random --cram  # combine flags
```

During a session:
- Type a letter to answer: `A`, `B`, `C`, `D`
- Multi-select: `A,C`
- After answering, type your reasoning (supports `#tags` like `#shotInDark #unsure`)
- Navigation: `>` next, `<` back, `#5` jump to Q5, `!s` submit, `!q` quit, `?` help

After submitting, your score is shown automatically and you can choose what to do next.

### `cert-drill grade <EXAM>`

Grade an attempt against the answer key.

```bash
cert-drill grade aws-saa-c03                    # grade latest, show all
cert-drill grade aws-saa-c03 --missed           # show only wrong answers
cert-drill grade aws-saa-c03 --score-only       # just the score
cert-drill grade aws-saa-c03 --attempt latest   # explicit attempt selection
```

### `cert-drill export <EXAM>`

Export graded results as markdown or AI-ready context.

```bash
cert-drill export aws-saa-c03                   # markdown export
cert-drill export aws-saa-c03 --missed          # only missed questions
cert-drill export aws-saa-c03 --ai-context      # formatted for AI review
cert-drill export aws-saa-c03 --ai-context | pbcopy   # copy to clipboard
```

The `--ai-context` flag wraps your missed questions with a prompt asking an AI to review your reasoning and identify misconceptions.

### `cert-drill flashcard <EXAM>`

Reveal-and-rate flashcard study mode.

```bash
cert-drill flashcard aws-saa-c03                    # all cards, randomized
cert-drill flashcard aws-saa-c03 --domain "Caching" # filter by topic
```

Each card shows the front (question/term), you press Enter to reveal the back (answer/definition), then rate yourself: `1` = forgot, `2` = fuzzy, `3` = got it.

### `cert-drill progress <EXAM>`

View scores by domain, weak areas, and improvement trends.

```bash
cert-drill progress aws-saa-c03
```

### `cert-drill review <EXAM>`

Review questions from past graded attempts.

```bash
cert-drill review aws-saa-c03              # all questions
cert-drill review aws-saa-c03 --missed     # only wrong answers
```

### `cert-drill import <EXAM> <FILE>`

Import answers from a markdown file (for grading answers you wrote outside the tool).

```bash
cert-drill import aws-saa-c03 my-answers.md
```

Supported formats:
```
1. D - reasoning here #shotInDark
5. A,C VPC would encrypt but I'm not sure
Q1. D - reasoning
1) B - seems right
```

Lines starting with `#`, `###`, or `---` are skipped (comments/headers/separators).

### `cert-drill load <PATH>`

Import an exam pack from a directory.

```bash
cert-drill load ./my-exam-pack/
```

## Creating Exam Packs

An exam pack is a directory with 2-3 files:

```
exams/my-cert/
├── exam.toml         # required: metadata
├── questions.md      # required: questions in structured markdown
├── answers.toml      # required: answer key with explanations
└── flashcards.md     # optional: flashcards
```

See `exams/aws-saa-c03/` for a complete example.

### exam.toml

```toml
id = "my-cert"
title = "My Certification Exam"
passing_score = 72.0

[[domains]]
name = "Domain 1: Topic Name"
weight = 30.0
question_range = [1, 30]
```

### questions.md

```markdown
## Domain 1: Topic Name

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

### flashcards.md

```markdown
## Topic Name

Q: Term or question
A: Definition or answer

Q: Another term
A: Another definition
```

## AI Integration

This project includes `CLAUDE.md` and `llms.txt` context files so AI assistants understand the project structure and data formats.

The `--ai-context` export flag generates output specifically formatted for AI review:

```bash
cert-drill export aws-saa-c03 --ai-context | pbcopy
# Paste into your AI chat for targeted feedback on your reasoning
```

## Included Exam Packs

- **aws-saa-c03** — AWS Solutions Architect Associate (SAA-C03)
  - 100 multiple-choice questions across 4 domains
  - 119 flashcards across 16 topics
  - Full answer key with explanations

## License

MIT
