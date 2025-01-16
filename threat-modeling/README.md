### Security Methods

- **Symbolic execution** for single functions within pallets
    - **[hardest] Generates CFG for each function**
    - Generates concrete test cases automatically
- **Model checking** for runtime
    - **[hardest] Generates finite-state machine for runtime**
    - **[hardest] Handle state explosion problem**
    - Check state transitions validity
    - Invariant preservation during runtime upgrades
    - Threat model checking - business logic-related
        - Validity of balance transfers and account management
        - Access control and permission enforcement
        - etc.
- **Generation-based Black-box fuzzer** for random runtime testing

### System Layers

- **Threat description layer**: Describe high-level, domain-specific threats for each feature.
- **Threat translation layer**: Translate requirements into intermediary formats (might require domain-specific language to describe threats similar to ProLeMa).
- **Threat database layer**: Well-formatted known threats.
- **Threat mapping layer**: Map translated domain-specific & known threats to discovered assets.
- **Scheduling layer**:
    - Generate then separate security tasks into three categories
        - Symbolic execution: testing single functions for known threats
        - Formal verification (model-checking for now): for runtime with constraints
        - Fuzzing: for the things exceed the formal verification constraints
    - Schedule and manage progresses of symbolic execution and model checking.
- **Symbolic execution layer**: Customizable execution engine for different use cases (only need a pre-configured one for MVP).
- **Formal verification layer**: Customizable verification engine for different use cases (only need a pre-configured one for MVP).
- **Fuzzing layer**: Fuzzing engine with customizable threat oracles (only need one threat oracle for MVP).
- **TODO layer**: Based-on researches, there are some high probabilities of integrating ML with security testing tools, need to figure out where are the integration points for each tool.

### Tool Workflow
- Take the JSON file produced by asset discovery tool as an input
- Build a system model using the asset discovered
- Take natural language threat description from users
- Translate the threats into an internal data structure
- Create an asset <-> threat mapping using translated threats and built-in threat database
- Use the created mapping to create security tasks for three downstream tools: symbolic execution, model checking, and fuzzing
- Call the tools and manage the progress
- Generate reports

### Todo List Before Implementing the SE Tool
- ✅ JSON file reader
- ✅ Pallet model data structure -> this is used to construct an internal model for pallets
- ✅ Threat data structure -> this is solely used to model threats, will be appended to the pallet model above
- ✅ Connection with LLMs
- [hardest] Deterministic output parser for LLM results
- ✅ Parse discovered assets into desired internal data structure
- PoC unit test generation using Rust macros to handle substrate types
- Asset <-> threat mapping function
- Automated unit test generation through macros
