---
name: github-pr-codex-review-loop
description: Create PRs with GitHub CLI, wait for Codex review in the background, then start fix work with a subagent.
---

# GitHub PR + Codex Review Loop

## Purpose
Create a PR with `gh`, wait for Codex review results in the background, then start fix implementation in a separate subagent.

## When to use
- User asks to create a PR via GitHub CLI.
- User asks to wait for Codex review and then start fixes.
- You want to automate PR creation + review polling + response.

## Workflow
1. Pre-check branch, diff, and test status.
2. Push branch if needed: `git push -u origin HEAD`.
3. Create PR with `gh pr create`.
4. Request Codex review using repository-standard command (for example, `/codex review` comment).
5. Start a background subagent (`run_in_background: true`) to poll PR reviews.
6. Once review arrives, start a fix subagent with:
   - PR number and branch
   - structured review findings with severity
   - expected deliverables (code updates, tests, verification commands)
   - completion criteria
7. Run tests/lints, commit, push, and report response status in PR comment.

## Polling command example
```bash
gh pr view <pr-number> --json reviews,comments,latestReviews
```

## PR body template
PR body must be written in **Japanese**.

```markdown
## 概要
- <変更の背景>

## 変更内容
- <変更点1>
- <変更点2>

## テスト
- [ ] cargo fmt -- --check
- [ ] cargo clippy -- -D warnings
- [ ] cargo test --workspace
```

## Review response template
```markdown
Codex レビューの指摘に対応しました。

## 対応済み
- <指摘A> → <対応内容>
- <指摘B> → <対応内容>

## 保留/非対応
- <指摘C> → <理由>
```
