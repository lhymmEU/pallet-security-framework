# Threat Modeling Purpose
The goal of creating an automated threat modeling tool is to effectively guide down stream resource-intensive tools like model checker and fuzzer.

## The Imagined Approach
- Customize threat models: A dedicated threat modeling form for users to define customized threat models.
- Pattern creation: Transform threat models into patterns that serves as an input to a dedicated threat parser.
- Pallet threat analyzer: Built on the "syn" crate, using generated patterns as rules to analyze the syntax tree.
- Automated guidance generation: Generate assumptions and assertions for Kani to do model checking. This should have some similarity to taint analysis.
- (Optional) Manual guidance generation: Create state machines for critical pallets to enhance the assumptions & assertions generated in the previous step.

## TODO
- Define the threat model data structure
- Model one threat and manually inject a threat according to the model into a pallet
- Define a pattern that could catch the injected threat
- Build a translator function to turn threat model into the pattern
- Build a threat analyzer that can mark the injected threat
- Build a modified taint analysis algorithm to generate constraints for Kani