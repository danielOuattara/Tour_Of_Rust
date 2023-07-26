/*
Memory
========

Rust programs have 3 memory regions where data is stored:

data memory : for data that is fixed in size and static, i.e. always available
-----------   through the life of program. Consider the text in your program
              e.g. "Hello World!". This text's bytes are only ever read from
              one place and therefore can be stored in this region.
              Compilers make lots of optimizations with this kind of data, and
              they are generally considered very fast to use since locations
              are known and are fixed.

stack memory : for data that is declared as variables within a function.
------------   The location of this memory never changes for the duration of
               a function call; because of this compilers can optimize code
               so stack data is very fast to access.

heap memory : for data that is created while the application is running.
------------  Data in this region may be added, moved, removed, resized, etc.
              Because of its dynamic nature it's generally considered slower
              to use, but it allows for much more creative usages of memory.
              When data is added to this region we call that an allocation.
              When data is removed from this section we call it a deallocation.
*/

pub fn main() {}
