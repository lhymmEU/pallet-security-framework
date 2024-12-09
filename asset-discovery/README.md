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
- ✅ Find pallet calls
- Extract function signatures of found pallet calls
- Find external function calls & extract their signatures
- Extract storage items
- Find cross-pallet interaction vectors (e.g. AssetsFungibles and etc.)
- Generate two types of report: JSON (for other tools to use), and PDF (for security personnel to review)