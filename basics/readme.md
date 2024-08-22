## Ownership, borrowing and lifetimes


- `Ownership`: The goal of ownership in Rust is to limit the ways you can reference and change data. This limitation will reduce the number of bugs and it will make the code easier to understand.
1. Every value is 'owned' by a single variable, struct, vector, etc at a time
2. Reassigning the value to another variable, passing it into a function, putting it into a vector etc. `moves` the value. The old variable cannot be used anymore!

- `Borrowing`:
3. You can create many read-only references to a value at the same time
4. You cannot move a value while a reference to the value exists
5. You can make a writable (mutable) reference to a value _only if_ there are no read-only references currently in use. There can be only one mutable reference to a value at a time.
6. You cannot mutate a value through the owner when any reference to the value exists.
7. Some type of values are _copied_ instead of moved (numbers, bools, chars, arrays/tuples with copyable elements)

- `Lifetimes`:
8. When a variable goes out of scope, the value owned by it is _dropped_
9. Values cannot be dropped if there are still active references to it
10. References to a value cannot outlive the value they refer to

General rules:
11. The above rules will dramatically change how you write code compared with other languages.
12. When in doubt, remember that Rust wants to minimize the unexpected updates to data


Need to store the argument somewhere? -> Favor taking ownership (receive a value)

Need to do a calculation with the value? -> favor receiving a read-only ref

Need to change the value? -> favor receiving a mutable ref




Blog on ownership: https://without.boats/blog/references-are-like-jumps/