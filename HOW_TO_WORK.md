# HOW TO WORK — TANTRA PROJECT

## 1. Purpose

This file defines how the Tantra project must be continued across ChatGPT conversations and how AI should manage the GitHub repository.

**Language name:** Tantra  
**Current GitHub repository:** Tantra.In  
**Repository visibility:** Public  
**Current default branch:** main

The repository name `Tantra.In` is only the current GitHub repository name. It does **not** rename the programming language. The language remains **Tantra**.

---

## 2. Single Source of Project Truth

Before doing project work, the AI must read the relevant project-control files from the repository, especially:

1. `TANTRA_BLUEPRINT.md`
2. `COMMON_INSTRUCTIONS.md`
3. `AGENTS.md`
4. `HANDOVER.md`
5. The latest files under `handover/`, when present
6. The actual source/configuration/workflow files relevant to the requested task

Do not rely only on the current chat.

The repository itself is the persistent project memory.

---

## 3. New Chat Continuation Rule

The user intentionally wants future chats to require only a very short command such as:

> Start

or

> Continue Tantra

When a new Tantra chat starts, AI must independently reconstruct the project state by reading the repository control files and the relevant code.

The AI must determine:

- What has already been completed.
- What is currently being worked on.
- The current architecture.
- Current compiler/language stage.
- Current security stage.
- Current bugs/errors.
- Known technical debt.
- Known unfinished work.
- Planned next steps.
- Files changed recently.
- Tests already passed/failed.
- Workflow status.
- Release status.
- Open pull requests/issues when relevant.
- The exact next logical work item.

The user should NOT be required to write a long context prompt again.

If the repository does not contain enough information to safely continue, the AI must identify the missing information instead of inventing it.

---

## 4. Before Every Significant Work Session

Perform this sequence:

1. Read project-control files.
2. Inspect the current GitHub repository state.
3. Inspect relevant source files.
4. Identify current stage.
5. Identify known bugs/errors.
6. Identify requirements affected by the requested change.
7. Decide whether the requested work needs one agent or a multi-agent workflow.
8. Implement only after understanding dependencies.
9. Test/verify the change.
10. Perform security and regression checks appropriate to the change.
11. Update project documentation/state.
12. Create/update the stage handover.
13. Commit the completed changes to GitHub according to the repository rules.
14. Report what was changed and the resulting state.

---

## 5. GitHub Work Policy

The connected GitHub account is the primary project repository interface.

For normal project development, AI should directly manage:

- source files
- documentation
- tests
- configuration
- GitHub Actions workflows
- CI configuration
- security configuration
- release preparation
- release notes
- pull requests
- issue-related project work
- tags/releases when the connected tooling supports the required operation

### Main branch policy

The user has requested AI-managed direct development.

Therefore:

- Direct commits to `main` are allowed for normal work when repository/tooling state permits.
- Do not create unnecessary branches or pull requests merely for ceremony.
- Use a branch + pull request when it materially improves safety, reviewability, isolation, or is required by repository protection rules.
- The AI decides the appropriate Git workflow based on risk and repository state.
- Never bypass actual GitHub branch protection or security controls.

### Destructive operations

Before irreversible destructive actions such as deleting important project history, deleting critical production resources, or intentionally removing major project functionality, verify the intent and preserve recoverability where practical.

---

## 6. Workflows, Releases and Pull Requests

AI is responsible for managing the GitHub development lifecycle as far as the connected GitHub capabilities allow.

This includes:

- creating/updating workflow files
- diagnosing workflow failures
- rerunning or correcting workflows when supported
- preparing releases
- maintaining release notes
- creating/updating pull requests when appropriate
- reviewing pull requests
- resolving review findings
- checking CI before merge
- maintaining version information
- keeping documentation synchronized with releases

The AI must make the engineering choice that best fits the current repository state rather than blindly using one fixed workflow.

---

## 7. Every Work Stage Must Have a Handover

At the end of every meaningful work stage, update:

`HANDOVER.md`

and create/archive a stage-specific record under:

`handover/STAGE-XXX-handover.md`

where XXX is the next sequential stage number.

Each handover must contain:

- Stage number.
- Date/time.
- Stage objective.
- Work completed.
- Files created/changed/deleted.
- Current architecture.
- Current implementation state.
- Tests run.
- Test results.
- Security checks.
- Bugs discovered.
- Bugs fixed.
- Known bugs remaining.
- Technical debt.
- Decisions made.
- Decisions intentionally deferred.
- Dependencies.
- Workflow/CI status.
- Release status.
- Current Git commit SHA when known.
- What is currently being worked on.
- Exact next recommended action.
- What must NOT be repeated.
- Important context required for the next chat.

`HANDOVER.md` is the **current active handover**.

Older stage handovers are historical records.

---

## 8. Never Lose Project State

Whenever a major architectural or implementation decision is made, record it in the appropriate project file.

Do not leave critical decisions only inside a chat.

If a requirement changes:

1. Update the appropriate specification/blueprint.
2. Update `HANDOVER.md`.
3. Record the change in the stage handover.
4. Update implementation if necessary.
5. Test affected areas.

---

## 9. Error and Bug Tracking

When an error/bug is found, record:

- Unique identifier.
- Severity.
- Component.
- Exact symptom.
- Reproduction method.
- Root cause if known.
- Current workaround if any.
- Fix status.
- Regression test.
- Related commit/PR when known.

Never silently forget a known unresolved bug.

---

## 10. Security-First Development

Security must be considered during design, implementation and verification.

For every security-sensitive change, consider:

- input validation
- memory/resource safety
- permissions/capabilities
- dependency risk
- supply-chain security
- secrets
- injection risks
- sandboxing
- error leakage
- authentication/authorization
- privacy
- denial-of-service/resource exhaustion
- unsafe native/FFI boundaries

Never claim absolute security.

Use measurable evidence and clearly document limitations.

---

## 11. Do Not Rewrite Existing Work Unnecessarily

Before modifying existing code:

- understand why it exists
- inspect dependent code
- check tests
- preserve valid architecture
- make the smallest coherent change unless a larger refactor is justified

Do not delete previously established project requirements merely because a new feature is being added.

---

## 12. AI Agent Workflow

Use the rules in `AGENTS.md`.

The Master Orchestrator must dynamically decide:

- whether agents are needed
- how many
- which specialties
- what each agent does
- who verifies whom
- whether adversarial review is required
- whether additional agents are required later

Do not create unnecessary agents.

For high-risk architecture/security/compiler work, independent verification is strongly preferred.

---

## 13. Code Quality Gate

A change is not considered complete merely because code was written.

Appropriate validation may include:

- formatting
- compilation
- unit tests
- integration tests
- regression tests
- security tests
- fuzzing
- static analysis
- dependency checks
- benchmark checks
- documentation checks

The required checks depend on the change.

---

## 14. Documentation Synchronization

Whenever behavior changes, update relevant documentation.

Important documentation includes:

- blueprint
- language specification
- syntax specification
- security specification
- compiler architecture
- UI specification
- graphics specification
- AI/math specification
- package ecosystem
- diagnostics specification
- roadmap
- handover records

Do not allow documentation to describe a feature as implemented when it is only planned.

Use explicit states such as:

- Planned
- Designed
- Prototype
- Implemented
- Tested
- Stable
- Deprecated

---

## 15. No Hallucinated Project State

If a file, feature, commit, workflow, test result, release, or bug cannot be verified from the repository/tooling, do not pretend it exists.

Use:

- verified
- inferred
- unknown
- blocked

as appropriate.

---

## 16. End-of-Session Checklist

Before ending meaningful work:

- [ ] Repository inspected
- [ ] Requirements checked
- [ ] Relevant code implemented
- [ ] Tests/verification completed
- [ ] Security reviewed
- [ ] Documentation synchronized
- [ ] HANDOVER.md updated
- [ ] Stage handover archived
- [ ] Git status/result verified
- [ ] Next step recorded

---

## 17. Continuation Command

The intended normal user command in a new chat is simply:

> **Start**

AI must then read the repository state and continue from the actual current project state.

---

## 18. Golden Rule

**The user should not have to remember the project state. The repository documentation and handover system must preserve it.**

The AI's job is to maintain that continuity automatically.
