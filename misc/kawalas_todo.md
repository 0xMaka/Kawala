## Current to do

- [ ] Tests   => Complete `View` coverage
- Founder mods are minimally to mostly covered, but lib is undercovered
- - Don't wanna play catch up or keep adding without coverage

- [ ] Print   => Refactor `summary` for cleaner default look when printing large pages
- Probably have dedicated method that takes column input and uses a std lib chunk method
- - Not worth a dependancy in termion and not about to botch a wrapper for ioctl over it
- - A heavier client could be a seperate package

- [ ] Masks   => View methods for simplified masking
- We can already build a mask, but can make the process feel more intuitive

- [ ] Replace => View method to replace from word, consuming replacement
- There is already replace, and long winded ways to do above, just add an abstraction

- [ ] Deque   => View method to pop from the top of the page
- Not an issue just undecided on how I want to handle it, if at all

- [x] Remove  => View method to remove an element from within the middle of the page
-  Not something I'd use over clearing, but very reasonable to expect to have the option

- [x] Chunk   => kwl32 dedicated chunk function
- Unlikley to be utilised by Kawala
- - Kawalas implementation is more forgiving for its less performance concerned `View`
- - Gives kwl32 viability as its own module, though Kawala will remain dependancy free
