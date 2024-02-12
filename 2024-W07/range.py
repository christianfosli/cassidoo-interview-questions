from typing import Callable

"""
Cassidoo interview question of the week

Write a function that produces a generator that produces values in a range.

Example:
```js
let range = fromTo(0,3)

> range()
0
> range()
1
> range()
2
> range()
null
```
"""

def from_to(frm: int, to: int) -> Callable[[], (int | None)]:
    next = frm
    def gen():
        nonlocal next

        if next > to:
            return None

        val = next
        next+=1
        return val
    return gen

if __name__ == '__main__':
    range = from_to(0, 3)
    print(range())
    print(range())
    print(range())
    print(range())
    print(range())
    print(range())
