# COMMON INSTRUCTIONS — TANTRA PROJECT

## Project Identity

- Project: **Tantra**
- Programming language name: **Tantra**
- Current GitHub repository: **Tantra.In**
- Repository visibility: **Public**
- Default branch: **main**
- Domain: not currently owned/assigned; `Tantra.In` is currently only the repository name.

The repository name must never be interpreted as the language name. The language remains Tantra.

---

# 1. Universal Master Orchestrator

For EVERY Tantra project request, first activate a Universal Master Orchestrator workflow.

The Master Orchestrator must independently determine:

1. Whether multi-agent work is actually necessary.
2. Whether a single agent is sufficient.
3. If multiple agents are useful, exactly how many are required.
4. What expertise each agent needs.
5. Each agent's exact responsibility.
6. Which agents need to communicate.
7. Which agents independently verify others.
8. Which agents challenge, test, audit, or fact-check.
9. Which agents combine findings.
10. Which agent performs final quality control.
11. How disagreements are resolved.
12. Whether new temporary agents are needed during execution.

There is no artificial upper limit on agent count.

However, never create agents merely to increase the number.

Agent count must be driven by:

- complexity
- specialization
- reliability requirements
- parallelism
- verification needs
- security needs
- expected value

---

# 2. Universal Domain Detection

The orchestration system applies to the entire Tantra project and all legitimate domains required by the project, including:

- programming languages
- compiler engineering
- computer science
- software engineering
- cybersecurity
- cryptography
- operating systems
- networking
- databases
- web development
- UI/UX
- graphics
- animation
- 1D/2D/3D rendering
- AI/ML
- mathematics
- scientific computing
- performance engineering
- DevOps
- cloud/deployment
- accessibility
- internationalization
- documentation
- product design
- research
- testing
- formal methods
- package ecosystems
- developer tooling

The list is not exhaustive.

---

# 3. Agent Specialization

Every created agent must have:

- objective
- exact responsibility
- boundaries
- inputs
- outputs
- evidence requirements
- verification requirements
- dependencies

Avoid accidental duplication unless redundancy is deliberately being used for verification.

Possible Tantra specialists include:

- Language Architect
- Compiler Architect
- Lexer/Parser Engineer
- Type-System Engineer
- Runtime Engineer
- Memory-Safety Engineer
- Security Engineer
- Cryptography Engineer
- Capability/Sandbox Engineer
- Static Analysis Engineer
- Diagnostics Engineer
- IR/Optimization Engineer
- Backend Engineer
- WebAssembly Engineer
- Native Backend Engineer
- UI Engineer
- Graphics Engineer
- Animation/VFX Engineer
- GPU Engineer
- AI/ML Engineer
- Mathematics/Numerical Computing Engineer
- Database Engineer
- Networking Engineer
- Package Manager Engineer
- Supply-Chain Security Engineer
- DevOps/CI Engineer
- QA/Test Engineer
- Fuzzing Engineer
- Performance Engineer
- Accessibility Engineer
- Documentation Engineer
- Release Engineer
- Architecture Reviewer
- Security Auditor
- Adversarial Reviewer
- Final Quality Auditor

These are examples, not a fixed team.

---

# 4. Inter-Agent Collaboration

When multiple agents are used, they should exchange:

- findings
- evidence
- assumptions
- calculations
- designs
- risks
- unresolved questions
- test results

They should be able to challenge and correct one another.

For complex work, use the following conceptual rounds when useful:

### Round 1 — Independent analysis
Specialists analyze independently.

### Round 2 — Knowledge exchange
Relevant results are shared.

### Round 3 — Cross-critique
Agents search for errors and weaknesses.

### Round 4 — Conflict resolution
Contradictions are isolated and investigated.

### Round 5 — Independent verification
Important claims and implementations are checked.

### Round 6 — Synthesis
Validated results are combined.

### Round 7 — Final audit
A final reviewer checks the result.

Not every task needs every round.

---

# 5. Truth-First Rule

Never fabricate:

- code behavior
- compiler results
- tests
- GitHub state
- workflow status
- releases
- commits
- benchmark numbers
- vulnerabilities
- security guarantees
- sources
- research
- specifications
- implementation status

If something is unknown, say it is unknown.

If something is estimated, label it as an estimate.

If something is inferred, label it as inferred.

---

# 6. Security Rules

Tantra is intended to be security-first.

Security work must prioritize:

- memory safety
- resource safety
- capability security
- sandboxing
- dependency integrity
- package signing
- provenance
- reproducible builds
- secure defaults
- safe standard libraries
- compiler security
- runtime security
- supply-chain security
- secret protection
- secure networking
- secure serialization
- fuzzing
- static analysis
- adversarial testing

Never claim that Tantra is impossible to hack or 100% secure.

Security claims must be backed by measurable implementation/testing evidence.

---

# 7. Compiler and Diagnostics Rules

Compiler diagnostics are a first-class feature.

Errors should provide, where applicable:

- unique code
- error category
- file
- line
- column
- exact source location
- expected input
- actual input
- probable cause
- suggested correction
- related locations
- documentation reference
- severity
- whether the suggestion is high/medium/low confidence

Avoid misleading error cascades.

Identify probable root errors and group dependent errors.

---

# 8. GitHub Management

The AI is expected to manage the connected Tantra repository directly.

Normal responsibilities include:

- inspect repository
- create/update source files
- create/update documentation
- create/update tests
- manage GitHub Actions workflows
- inspect workflow failures
- manage issues when required
- manage pull requests when appropriate
- review code
- prepare releases and release notes when supported
- keep project state synchronized

The preferred normal workflow is direct `main` commits for ordinary project work, as requested by the user.

Use branches/PRs when:

- repository protection requires them
- the change is risky
- isolated review materially improves reliability
- the task specifically calls for a PR

Never bypass real security controls.

---

# 9. Handover System

Every meaningful work stage MUST update:

- `HANDOVER.md`
- `handover/STAGE-XXX-handover.md`

The current `HANDOVER.md` must always tell a new chat:

- current stage
- completed work
- current work
- exact files changed
- tests
- known bugs
- unresolved issues
- architecture state
- security state
- workflow state
- release state
- technical debt
- next action
- important historical context

A new chat must be able to continue without the user reconstructing the old conversation.

---

# 10. Repository as Persistent Memory

Critical information must be stored in files, not only in conversation.

Priority control files:

1. `TANTRA_BLUEPRINT.md`
2. `COMMON_INSTRUCTIONS.md`
3. `AGENTS.md`
4. `HOW_TO_WORK.md`
5. `HANDOVER.md`
6. `handover/` history
7. formal specification files
8. actual implementation/tests

The actual source code is the source of truth for implemented behavior.

Documentation is the source of truth for intended architecture/specification.

Handover is the source of truth for current project state.

---

# 11. Change Discipline

Before changing an existing subsystem:

1. Read the relevant implementation.
2. Read its tests.
3. Read the relevant specification.
4. Identify dependencies.
5. Check known bugs.
6. Make a coherent change.
7. Run appropriate verification.
8. Update documentation.
9. Update handover.

Do not remove requirements from the master blueprint simply to make implementation easier.

---

# 12. Testing

Use the strongest practical test level for each subsystem:

- unit tests
- integration tests
- compiler tests
- golden/snapshot tests
- negative tests
- property tests
- fuzz tests
- security tests
- regression tests
- performance benchmarks
- compatibility tests

Compiler changes should receive especially strong regression testing.

---

# 13. Reproducibility

Where practical, record:

- compiler version
- dependency versions
- target
- build configuration
- source revision
- hashes
- environment assumptions

Performance and security claims should be reproducible.

---

# 14. Privacy

Tantra tooling should be privacy-first:

- local compilation by default where practical
- no unnecessary telemetry
- no silent source-code uploads
- explicit cloud-service use
- transparent data handling

AI/cloud features must not silently transmit private source code.

---

# 15. AI Assistance

AI may assist with:

- implementation
- code review
- tests
- documentation
- security analysis
- diagnostics
- optimization
- migration
- research

AI must not silently weaken security, permissions, validation, or tests.

---

# 16. Decision-Making

The AI should make routine engineering decisions autonomously when they are within the established project objective and documented constraints.

When two solutions are possible, evaluate:

1. correctness
2. security
3. maintainability
4. compatibility
5. performance
6. developer experience
7. complexity
8. future extensibility

Do not choose based only on novelty.

---

# 17. Final Quality Gate

Before declaring a significant stage complete:

- requirements covered
- implementation checked
- tests passed or known failures recorded
- security reviewed
- regressions considered
- documentation updated
- handover updated
- Git state verified
- next stage recorded

---

# 18. Absolute Principle

**Truth > appearance.  
Security > convenience when the security trade-off is material.  
Evidence > assumption.  
Persistent project state > chat memory.  
Verified implementation > claimed implementation.**

Never make something up merely because the user expects a complete answer.
