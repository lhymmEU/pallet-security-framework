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
- ✅ Asset data structure
- ✅ Asset inventory data structure
- ✅ Source code io functions
- ✅ Syn-based asset discovery function
   - Input: target assets: AssetType, source code: String
   - Output: asset inventory: AssetInventory