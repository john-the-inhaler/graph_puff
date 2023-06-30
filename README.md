# graph_puff
## simple graph processing library
Because why not. This library implements a suite of simple graph structures for various uses.
Although how graphs are stored and processed looks like it's optimised, it isn't. We're using methods that simplify processing, and that includes removing linked data structures (like linked lists) and using arrays. It's difficult to make traversing a linked-list style graph iterative (And no, using an seperate stack isn't a solution, that's called observer recursion).
Graphs that do not have tagged edges (edges with meanings or data attributed to them) will mostly be implemented using some form of table data structure (like a hashmap) since that makes lookups trivial. Graphs with tagged edges will be implemented as arrays of edges since it's easiest.

Though, since graph processing ends up becoming complicated (especially in semantic graphs), you are allowed to optimise the implementation. I'll occasionally optimise the library, but feel free to optimise it for me.
## todo
- [ ] graph-space graphs (integer only graphs)
    + [ ] graph
        + [ ] tagged edge
        + [ ] untagged edge
    + [ ] digraph
        + [ ] tagged edge
        + [ ] untagged edge
    + [ ] scoping (for both graphs and digraphs)
- [ ] graph-space manipulation
    + [ ] querying
    + [ ] probing
    + [ ] adding nodes and edges
    + [ ] deriving (using rules to generate new edges)
- [ ] graph wrappers (for using your own datatypes in the graph)
    + [ ] graph
        + [ ] tagged edge
        + [ ] untagged edge
    + [ ] digraph
        + [ ] tagged edge
        + [ ] untagged edge
    + [ ] scoping
- [ ] (NOT FINAL) `u8` based graph-space tagged digraph.
