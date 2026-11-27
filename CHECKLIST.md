# Bootcamp Checklist (target: Sunday 29 Nov 2026)

What this repo needs before you can submit, with the current status of each
exercise and a three-day plan.

## Where things stand (checked Friday 27 Nov 2026)

- **0 of 37 tests pass.** `cargo test --locked` compiles, and every test fails
  with `not yet implemented`, because every function in `src/ex01.rs` to
  `src/ex08.rs` still ends in `todo!()`.
- The only branch is `main`. There is no `solutions/...` branch yet, and no
  submission pull request to `dev3pack/Solana-Rust-Bootcamp-EN-`.
- The exercises page at app.dev3pack.xyz needs a sign-in, so this list was
  built from the repo alone (README, `tasks/`, `tests/`). When you sign in,
  check whether that page lists anything **beyond** these 8 exercises, such as
  a quiz, a project, or attendance, and add it below.

## How the grade works (from the README)

- One point per passing test: **19** for Rust Basics 1 (ex01–ex04) and **18**
  for Rust Basics 2 (ex05–ex08).
- The grade comes from the checks on your **pull request** to the class repo,
  not from your fork.
- CI first runs `sha256sum -c tests.sha256`. Editing anything in `tests/`
  fails the build even if your code passes.
- A mentor may ask you to explain any function in a **two-minute call**.
  Code you cannot explain does not count (see `AI_POLICY.md`).
- Mentors look for unneeded `unwrap()` and `clone()` calls.

## Exercises

| # | Topic | Deck | Tests | Status | Day |
| --- | --- | --- | ---: | --- | --- |
| ex01 | Variables and types | Rust Basics 1 | 5 | Not started (5 `todo!()`) | Fri |
| ex02 | Functions and control flow | Rust Basics 1 | 5 | Not started (5 `todo!()`) | Fri |
| ex03 | Ownership | Rust Basics 1 | 4 | Not started (5 `todo!()`) | Fri |
| ex04 | Borrowing | Rust Basics 1 | 5 | Not started (5 `todo!()`) | Sat |
| ex05 | Structs and `impl` | Rust Basics 2 | 5 | Not started (5 `todo!()`) | Sat |
| ex06 | Enums and `match` | Rust Basics 2 | 4 | Not started (4 `todo!()`) | Sat |
| ex07 | `Result` and `?` | Rust Basics 2 | 5 | Not started (4 `todo!()`) | Sun |
| ex08 | Capstone: mini bank | Rust Basics 2 | 4 | Not started (5 `todo!()`) | Sun |
| | **Total** | | **37** | **0 / 37** | |

## Plan

### Friday 27 Nov: setup and the easy wins

- [ ] Fork check: this repo is your fork. Add the class repo as `upstream`:
      `git remote add upstream https://github.com/dev3pack/Solana-Rust-Bootcamp-EN-.git`
- [ ] Make your working branch: `git checkout -b solutions/michella`
- [ ] ex01: read `tasks/ex01.md`, fill `src/ex01.rs`, run `cargo test --test ex01` (5 passed)
- [ ] ex02: `cargo test --test ex02` (5 passed)
- [ ] ex03: `cargo test --test ex03` (4 passed)
- [ ] Commit and push: `git push -u origin solutions/michella`

### Saturday 28 Nov: finish Rust Basics 1, start Rust Basics 2

- [ ] ex04: `cargo test --test ex04` (5 passed). Rust Basics 1 is now 19 / 19.
- [ ] ex05: `cargo test --test ex05` (5 passed)
- [ ] ex06: `cargo test --test ex06` (4 passed)
- [ ] Commit and push

### Sunday 29 Nov: error handling, capstone, submit

- [ ] ex07: `cargo test --test ex07` (5 passed). Use `Result`, no `panic!` or `unwrap()`.
- [ ] ex08: `cargo test --test ex08` (4 passed). No `HashMap`, no traits; history is a `Vec<String>`.
- [ ] Run everything: `cargo test` shows 37 passed
- [ ] Check the tests are untouched: `shasum -a 256 -c tests.sha256`
- [ ] Push, then open the submission pull request:
      base `dev3pack/Solana-Rust-Bootcamp-EN-` / `main`, compare `solutions/michella`,
      title `Solutions — Michella`. Do not merge it.
- [ ] In the PR description, note any AI help, for example
      "Used an AI chat to understand E0502. Wrote the fix myself."
- [ ] Confirm the PR checks are green (the first run may wait for instructor approval)
- [ ] Practice a two-minute explanation for each exercise using the
      "Done when" questions at the bottom of each `tasks/exNN.md`
- [ ] Sign in to app.dev3pack.xyz/bootcamp/exercises and tick off anything
      else it asks for

## Rules to keep in mind

- Only change files in `src/`. Never edit `tests/`, `tests.sha256`, or `.github/`.
- If a test and a task disagree, the test wins.
- Questions go to issues in the **class** repo, with the exercise number, the
  command, and the full compiler error.
- This `CHECKLIST.md` is a note for you, not part of the homework. Before you
  open the submission PR, remove it from `solutions/michella`
  (`git rm CHECKLIST.md`) so the PR only changes `src/`.
