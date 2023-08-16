\--- Day 9: All in a Single Night ---
----------

Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations. He can start and end at any two (different) locations he wants, but he must visit each location exactly once. What is the *shortest distance* he can travel to achieve this?

For example, given the following distances:

```
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141

```

The possible routes are therefore:

```
Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982

```

The shortest of these is `London -> Dublin -> Belfast = 605`, and so the answer is `605` in this example.

What is the distance of the shortest route?

Your puzzle answer was `251`.

The first half of this puzzle is complete! It provides one gold star: \*

\--- Part Two ---
----------

The next year, just to show off, Santa decides to take the route with the *longest distance* instead.

He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.

For example, given the distances above, the longest route would be `982` via (for example) `Dublin -> London -> Belfast`.

What is the distance of the longest route?

Answer:

Although it hasn't changed, you can still [get your puzzle input](9/input).

You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=I%27ve+completed+Part+One+of+%22All+in+a+Single+Night%22+%2D+Day+9+%2D+Advent+of+Code+2015&url=https%3A%2F%2Fadventofcode%2Ecom%2F2015%2Fday%2F9&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.