# pallet-security
A collection of tools designed specifically for Polkadot pallets.

## Framework Overview
This framework represents the foundational layer of a robust security analysis framework engineered specifically for Polkadot pallet verification. The framework implements a bifurcated testing strategy: conducting analysis at the individual pallet level, followed by runtime security verification. This methodology is predicated on the understanding that threat models exhibit high context specificity. Rather than attempting to construct a universal threat model, the framework emphasizes adaptability through modular threat model generation.

## Security Analysis Architecture

### Pallet-Level Verification
- **Primary Mechanism**: Formal model checking for invariant verification
- **Secondary Layer**: Targeted fuzzing strategies for edge case detection
- **Core Focus**: Derivation of mathematical correctness proofs

### Runtime-Level Verification
- **Primary Mechanism**: State machine fuzzing for interaction analysis
- **Secondary Layer**: Bounded model checking on critical execution paths
- **Core Focus**: Integration validation and cross-pallet interaction verification

## Implementation Workflow

### Phase 1: Asset Discovery
Automated identification and classification of security-critical assets within pallet implementations.

### Phase 2: Threat Model Definition
Development of pallet-specific threat model utilizing natural language processing, incorporating severity quantification metrics.

### Phase 3: Formal Verification
Generation of mathematical correctness proofs based on identified threat vectors.

### Phase 4: Dynamic Analysis
Implementation of customized test harness for targeted fuzzing of high-severity assets.

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