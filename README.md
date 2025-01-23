# pallet-security
A collection of tools designed specifically for Polkadot pallets.

## Framework Overview
This framework represents the foundational layer of a robust security analysis framework engineered specifically for Polkadot pallet verification. The framework implements a bifurcated testing strategy: conducting analysis at the individual pallet level, followed by runtime security verification. This methodology is predicated on the understanding that threat models exhibit high context specificity. Rather than attempting to construct a universal threat model, the framework emphasizes adaptability through modular threat model generation.

### Problems This Framework Designed to Solve
The general goal is to automate as much as possible the current security auditing process and make it closely aligned with developers workflow (more actionable) to minimize disruptions and maximize effectives.
- Too many documentations scattered around
- Extremely complex to setup different tools for testing
- Too many general descriptions, lack of enough actionable information for develoeprs
- Doc <-> Code separation, security audit docs are too far away from actual codes
- Lack of progress tracking on issue fixing

### Features Designed
- **Doc-bench**: Generate detailed but concise code descriptions. Ideally the description should get generated before writing codes, so this feature supports two generation flows: feature design doc -> description, and code implementation -> description. This feature should have a built-in asset discovery mechanism, and a system model building mechanism.
- **Toolbox UI + SDK**: The framework will provide two functions for managing the complex tool setup process: UI for product teams to generate tests for developers; and an SDK for developers to easily setup more fine-grained test generation procedures in a standalone Rust file.
- **Tips-gen**: The generated security reports should in two formats: document for the team leaders to review, and code comments with fixing suggestions directly applied to the call-sites that cause the problem.
- **Checkmark**: The tool will maintain a synced todo list inside the doc-bench with the code comments generated by tips-gen. Developers can mark the code comments with ⌛️ to indicate work-in-progress, and ✅ to indicate work finished.

## Framework Design Todo List
### Toolbox SDK
- Asset Discovery: Input pallet file, output asset inventory in JSON format
    - ✅ Storage Items
    - ✅ Dispatchable Functions
    - ✅ Helper Functions
    - Hooks
    - Runtime Interface
    - ✅ Events
    - ✅ Errors
    - Dependencies (Cargo.toml)
    - Cryptography Primitives
    - Code Refactor
    - CLI Interface
- Threat Modeling
    - ✅ Pallet model data structure
    - ✅ Threat model data structure
    - Asset-to-threat mapping function
    - Severity ranking function
    - Pallet-specific unit tests generation macro
    - Investigate cucumber & Rust mini-DSLs: https://cucumber-rs.github.io/cucumber/main/
    - State machine generation
    - Economic threats & game theory modeling - consensus modeling
    - Code Refactor
    - CLI Interface
- Fuzzing
    - Chopsticks for network simulation: https://docs.polkadot.com/develop/toolkit/parachains/fork-chains/chopsticks/



## Substack Technical Article Publication Plan
- A gental introduction of general code analysis pipeline.
- Dive into Rust AST: A gental introduction of the syn crate.
- Add meanings to the code: Why & How to decorate an AST.
- The foundation of advanced Rust code analysis: Rust Control Flow Graph Generation.
- Security assets & threat modeling: How to map conceptual security threats with domain-specific codes & provide programmable guidance for downstream security tools [formal verification, static analysis, fuzzing].
- The verification boundry: How to construct proof system for pallet functions & mark the ones that cannot get completely verified.
- Pallet decoration: How to turn web3 threats into AST decorations for pallets.
- Transaction delivery system: How to achieve 100-1000 executions per second for Polkadot runtime fuzzing.
- Domain validity: How to achieve 5-15% well-formed (syntactically valid) transaction generation rate.
- Catching the uncatchable: How to detect non-crashing vulnerabilities using threat models.
- [ Final ] Create your own security tools: vulnerability detection SDK for substrate-based blockchains.