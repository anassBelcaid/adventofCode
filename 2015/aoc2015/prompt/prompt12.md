\--- Day 12: JSAbacusFramework.io ---
----------

Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

They have a [JSON](http://json.org/) document which contains a variety of things: arrays (`[1,2,3]`), objects (`{"a":1, "b":2}`), numbers, and strings. Your first job is to simply find all of the *numbers* throughout the document and add them together.

For example:

* `[1,2,3]` and `{"a":2,"b":4}` both have a sum of `6`.
* `[[[3]]]` and `{"a":{"b":4},"c":-1}` both have a sum of `3`.
* `{"a":[-1,1]}` and `[-1,{"a":1}]` both have a sum of `0`.
* `[]` and `{}` both have a sum of `0`.

You will not encounter any strings containing numbers.

What is the *sum of all numbers* in the document?

Your puzzle answer was `111754`.

The first half of this puzzle is complete! It provides one gold star: \*

\--- Part Two ---
----------

Uh oh - the Accounting-Elves have realized that they double-counted everything *red*.

Ignore any object (and all of its children) which has any property with the value `"red"`. Do this only for objects (`{...}`), not arrays (`[...]`).

* `[1,2,3]` still has a sum of `6`.
* `[1,{"c":"red","b":2},3]` now has a sum of `4`, because the middle object is ignored.
* `{"d":"red","e":[1,2,3,4],"f":5}` now has a sum of `0`, because the entire structure is ignored.
* `[1,"red",5]` has a sum of `6`, because `"red"` in an array has no effect.

Answer:

Although it hasn't changed, you can still [get your puzzle input](12/input).

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=I%27ve+completed+Part+One+of+%22JSAbacusFramework%2Eio%22+%2D+Day+12+%2D+Advent+of+Code+2015&url=https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F12&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.