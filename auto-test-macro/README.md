## Description
This macro is designed to automatically generate unit tests for Polkadot pallet functions.

## Todo List
- ✅ Provide access to pallet-specific types
- ✅ Implement a generator to generate extreme values for the provided types
- Add a flag to instruct the macro to generate test cases automatically
- ✅ Find an easy way to provide access to chain-specific types
- Add functionality to accept Option<> return values
- Add functionality to accept arbitrary return values
- Find a solution to handle dependency conflicts (frame-support, frame-system, sp-runtime) with consuming chains
- Find a solution to automatically copy & paste runtim-specific types to this macro
- Find a solution to automatically generate extreme values