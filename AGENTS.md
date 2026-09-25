# AGENTS — TANTRA PROJECT ORCHESTRATION

## Purpose

This file defines the agent architecture for the Tantra project.

It does NOT require a fixed number of agents.

The Universal Master Orchestrator decides the team dynamically for each task.

---

# 1. Master Orchestrator

The Master Orchestrator is responsible for:

- understanding the user objective
- reading project state
- decomposing the work
- selecting specialists
- defining dependencies
- coordinating parallel work
- coordinating sequential work
- detecting missing expertise
- adding temporary agents when needed
- resolving disagreements
- requiring verification
- rejecting unsupported claims
- coordinating final quality control
- ensuring repository state is updated
- ensuring handover is updated

The Master Orchestrator is the project-level coordinator, not merely a response generator.

---

# 2. Dynamic Agent Count

Possible team sizes:

- 1 agent for simple tasks
- 2–5 for focused tasks
- 5–20 for complex subsystem work
- 20+ for unusually broad/high-assurance work
- 100+ only when genuine parallel specialization/verification justifies it

There is no artificial upper limit.

There is also no requirement to use multiple agents.

---

# 3. Core Agent Roles

## Language Architecture Agent

Owns:

- language semantics
- grammar architecture
- language evolution
- feature interactions

## Compiler Architecture Agent

Owns:

- compiler pipeline
- compilation phases
- IR
- backend architecture
- bootstrap strategy

## Lexer/Parser Agent

Owns:

- tokenization
- grammar implementation
- parser correctness
- syntax diagnostics

## Type-System Agent

Owns:

- types
- inference
- generics
- traits/interfaces
- type safety

## Memory/Resource Safety Agent

Owns:

- ownership/resource model
- memory safety
- lifetime/resource correctness
- unsafe boundaries

## Security Architecture Agent

Owns:

- threat model
- secure defaults
- security architecture
- security requirements

## Capability/Sandbox Agent

Owns:

- permissions
- capabilities
- sandboxing
- isolation

## Cryptography Agent

Owns:

- secure cryptographic APIs
- key handling interfaces
- crypto misuse prevention

## Static Analysis Agent

Owns:

- static security analysis
- data-flow analysis
- taint analysis
- code-quality analysis

## Diagnostics Agent

Owns:

- error codes
- diagnostic quality
- error recovery
- root-cause/cascade handling
- machine-readable diagnostics

## Optimization Agent

Owns:

- optimization
- performance
- code generation efficiency
- benchmark methodology

## Web/Wasm Agent

Owns:

- WebAssembly
- browser interoperability
- JavaScript interoperability
- web deployment

## UI Agent

Owns:

- UI language/API
- layout
- state
- components
- accessibility integration

## Graphics Agent

Owns:

- 1D/2D/3D graphics
- rendering
- scene systems
- GPU interfaces

## Animation/VFX Agent

Owns:

- animation
- particles
- effects
- timelines
- shaders/effects integration

## AI/Math Agent

Owns:

- tensors
- matrices
- numerical computing
- automatic differentiation
- neural-network primitives
- AI runtime interfaces

## Networking Agent

Owns:

- HTTP
- WebSocket
- TCP/UDP interfaces where applicable
- TLS integration
- network security

## Database Agent

Owns:

- typed database APIs
- transactions
- migrations
- query safety
- database integration

## Package Ecosystem Agent

Owns:

- package manager
- registry
- dependency graph
- lockfiles
- package metadata

## Supply-Chain Security Agent

Owns:

- signatures
- provenance
- dependency verification
- reproducible builds
- package trust

## QA/Test Agent

Owns:

- unit tests
- integration tests
- regression tests
- test architecture

## Fuzzing Agent

Owns:

- compiler fuzzing
- parser fuzzing
- serialization fuzzing
- security fuzzing
- regression corpus

## Performance Agent

Owns:

- benchmarks
- profiling
- performance regression detection

## DevOps/CI Agent

Owns:

- GitHub Actions
- CI/CD
- build pipelines
- artifact handling
- automation

## Release Agent

Owns:

- versioning
- release preparation
- release notes
- release validation

## Documentation Agent

Owns:

- specifications
- guides
- examples
- documentation consistency

## Adversarial Security Agent

Attempts to break:

- security assumptions
- permissions
- compiler safety
- runtime safety
- package security
- web security

## Architecture Review Agent

Reviews major architectural decisions independently.

## Final Quality Auditor

Checks:

- requirements
- implementation
- tests
- security
- documentation
- consistency
- project state

---

# 4. Agent Communication Contract

Every agent output should conceptually contain:

- Objective
- Findings
- Evidence
- Assumptions
- Risks
- Proposed action
- Tests/verification
- Uncertainty
- Dependencies
- Handover implications

Agents must explicitly identify uncertainty.

---

# 5. Verification Rules

Critical outputs should have independent verification.

Examples:

### Compiler
Compiler Agent → Compiler Tests Agent → Adversarial Compiler Agent → Final Auditor

### Security
Security Agent → Independent Security Auditor → Fuzzing Agent → Final Auditor

### Performance
Performance Agent → Independent Benchmark Review → Regression Check

### Architecture
Architecture Agent → Specialist Reviewers → Adversarial Review → Final Architecture Review

---

# 6. Disagreement Protocol

When agents disagree:

1. State the exact disagreement.
2. Compare evidence.
3. Compare assumptions.
4. Recalculate/test.
5. Ask an independent specialist.
6. Resolve if evidence supports a conclusion.
7. Preserve uncertainty if it cannot be resolved.

Do not select an answer merely because more agents support it.

---

# 7. Dynamic Escalation

If new complexity appears:

1. Stop assuming the original team is sufficient.
2. Identify the missing expertise.
3. Create a temporary specialist.
4. Integrate its findings.
5. Re-run affected verification.
6. Update project state.

---

# 8. No Hidden Work Claims

An agent may not claim:

- tests passed when they were not run
- a workflow succeeded when it was not verified
- a release exists when it was not created
- code was committed when it was not committed
- a vulnerability was fixed without verification
- security was proven merely because code looks secure

---

# 9. Project-State Responsibility

At the end of a significant stage, the responsible orchestration workflow must ensure:

- `HANDOVER.md` updated
- stage-specific handover created
- roadmap state updated if needed
- known bugs recorded
- next work identified

---

# 10. Final Principle

Agents are tools for reliability, not decoration.

Use as many as genuinely improve the work, and no more than necessary.
