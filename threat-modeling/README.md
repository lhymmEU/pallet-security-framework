# Threat Modeling Purpose
The goal of creating an automated threat modeling tool is to effectively guide down stream resource-intensive tools like model checker and fuzzer.

## The Imagined Approach
- Customize threat models: A dedicated threat modeling form for users to define customized threat models.
- Pattern creation: Transform threat models into patterns that serves as an input to a dedicated threat parser.
- Pallet threat after-parser: Built on the "syn" crate, using generated patterns as rules to analyze the syntax tree.
- AUtomatic guidance generation: Generate assumptions and assertions for Kani to do model checking.
- Manual guidance generation: Create state machines for critical pallets to enhance the assumptions & assertions generated in the previous step.