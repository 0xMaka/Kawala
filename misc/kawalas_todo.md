## Current to do 

- [ ] Masks   => View methods for simplified masking
- We can already build a mask, but can make the process feel more intuitive

- [ ] Replace => View method to replace from word, consuming replacement
- There is already replace, and long winded ways to do above, just add an abstraction

- [ ] Deque   => View method to pop from the top of the page
- Not an issue just undecided on how I want to handle empty, if at all

- [x] Remove  => View method to remove an element from within the middle of the page
-  Not something I'd use over clearing, but very reasonable to expect to have the option

- [x] Chunk => kwl32 dedicated chunk function
- Unlikley to be utilised by Kawala
- - Kawalas implementation is more forgiving for its less performance concerned `View`.
- - Gives kwl32 viability as its own module, though Kawala will remain dependancy free.

