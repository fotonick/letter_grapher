Letter grapher
==============

This is a tool to help with puzzles where an input phrase is given and you must fill in a graph of letter adjacency.

Example
-------

```
$ letter_grapher 'FULL FATHOM FIVE THY FATHER LIES'
{'M': {'O', 'F'}, 'I': {'L', 'V', 'F', 'E'}, 'R': {'L', 'E'}, 'V': {'I', 'E'}, 'Y': {'H', 'F'}, 'E': {'V', 'I', 'H', 'T', 'R', 'S'}, 'U': {'L', 'F'}, 'L': {'R', 'I', 'U', 'F'}, 'H': {'T', 'O', 'Y', 'E'}, 'S': {'E'}, 'A': {'T', 'F'}, 'T': {'A', 'E', 'H'}, 'F': {'Y', 'L', 'A', 'M', 'U', 'I'}, 'O': {'M', 'H'}}
strict graph {
  M -- O;
  M -- F;
  I -- L;
  I -- V;
  I -- F;
  I -- E;
  R -- L;
  R -- E;
  V -- E;
  Y -- H;
  Y -- F;
  E -- H;
  E -- T;
  E -- S;
  U -- L;
  U -- F;
  L -- F;
  H -- T;
  H -- O;
  A -- T;
  A -- F;
}
```

Letter adjacency is shown in two representations. The first is each letter and all of its neighbors (deduplicated), and the second is as a set of graph edges. If you paste the generated code ``strict graph { ... }`` into [http://www.webgraphviz.com/](http://www.webgraphviz.com/) or run it through GraphViz locally, then you would get a pretty graph.

License
-------

This project is licensed under the MIT license.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in letter_grapher by you, shall be licensed as MIT, without any additional terms or conditions.