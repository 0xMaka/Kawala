
## Version logs 

##### v0.1.1 
- Initial commit
- - Porting over some of the python tools I wrote, as a single module
- - Data dump of initial working library { lib.rs, kwl32.rs, bai.rs }

##### v0.1.2 
- Removed traits
- - Codebase wasn't written with enough respect for them, so they aren't being used

- Modified `View` method `summary()` (still too crude)
- - Now prints each k,v to a single line if less than `SUMMARY_COUNT`
- - Changed summary key _"Signature"_ to _"sig"_

- Fixed printing error in example `basic_stream`

- Added `data()` and `len()` wrappers to `Signature` and  `Word` types for convenience
- Added more common `pop` to`View`, returns the hex `String` as opposed to `Word` type. 
- Swapped some magic numbers for constants

##### v0.1.3
- Added edge case handlers to `pad32l` and `pad32r` 
- - Functions needed to be more robust as the ones accepting arbitrary length 
- Tests added to cover most of bai and kwl32
- Modules are now public

##### v.1.4-pre
- Added `chunk32` and `chunks32` to kwl32

