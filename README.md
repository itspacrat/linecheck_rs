# linecheck
#### a mathemaical single-dimension data checker written in [rust](https://www.rust-lang.org/)
### navigation:
[about](#what-is-this-about)
## what it this about?
- i needed a library that abstracted a few frequent checks that i do, but having a lot of these checks together might benefit somoene else down the road so i figured "why not publish it for everyone to criticize and mock while i'm at it"
## who is this for?
- me, honestly.
- anyone who likes the tools i have wrapped up in a litle bow with this project.
## what stuff can i do with it *right now*?
you can currently do the following:
- check if the length of an iterator is a perfect square [da5d64a](https://github.com/itspacrat/linecheck_rs/commit/da5d64abe23935b9491198e360b018a5fa78eb94)
- check if the value of an integer is a perfect square [da5d64a](https://github.com/itspacrat/linecheck_rs/commit/da5d64abe23935b9491198e360b018a5fa78eb94)
- check if the value of an integer is a perfect cube
- check if the length of an iterator is a perfect cube
## what stuff is planned for *later*?
- set checks, which will be really useful for sudoku or other puzzle applicaitons
## where'd you get the audacity to make this?
> i got the idea to make a 3d sudoku-like puzzle in rust from my girlfriend (which is going to now cost me all of my hair i fear)
> i realized a lot of the checks were for single-row data so i needed a more maintainable way to do checks over iterators and stuff.
> - references:
>   - [geeksforgeeks article](https://www.geeksforgeeks.org/check-if-given-number-is-perfect-square-in-cpp/) for the int & linear square checks
