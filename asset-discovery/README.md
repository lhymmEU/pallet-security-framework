# Critical Asset Classification
Security-critical components of a pallet in descending order of severity:

1. Pallet Call Interfaces
   - Direct interaction points with users
   - Primary attack surface consideration

2. Public Function Implementations
   - External accessibility vectors
   - State mutation capabilities

3. Storage Item Definitions
   - Persistent state management
   - Data integrity and compliance considerations

4. External Function Call Patterns
   - Cross-pallet interaction vectors
   - System call dependencies

## TODO
- ✅ Extract storage items
- ✅ Extract constants
- ✅ Extract events
- ✅ Extract errors
- ✅ Consolidate codes using the new procedural macro finding algorithm
- Update asset-related data structures for threat modeling procedure to use
- ✅ Make the code robust