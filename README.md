# Langbda

A program that models the cognitive process of a listener receiving a linear sequence of tokens and constructing a syntax tree.

## Roadmap

- [ ] Projection onto `MOVED()`
- [ ] Support branching on `receive()` and `decide()`
  - `(A -> (B -> C)) <=> (B -> (A -> C))`
    - Redefine `Node::Lambda` as `{ from: Vec<Value>, to: Value }`
  - `((A -> B) -> (C -> D)) <=> ((C -> A), (B -> D)) <=> (A, (B -> (C -> D)))`
    - Default to `(A, (B -> (C -> D)))`
    - Add `Node::From` to denote dangling `(A ->)`
      - `(A ->)` is produced as a decomposition of `([A, B] -> B))`
      - `(A ->)` automatically attaches to a newly inserted node
- [ ] Not in plan: Add a NOVALUE value to every category
  - Possibly with new syntax `!`

### Theory
- [ ] Write a documentation to explain the theory
- [ ] Collin Phillips (1996, MIT)

### User Experience
- [ ] Not in plan: A language server for .lexicon files
  - Detect unreachable entries
  - Detect possible cycles

## Define a lexicon

[Example lexicon for English](assets/lexicons/en.lexicon)

## Get your sentence parsed

Below shows how the model captures the two possible interpretations of the sentence "The child ate an apple in the room.", which is created by the scoping ambiguity of the prepositional phrase "in the room".

![](assets/examples/the-child-ate-an-apple-in-the-room-_tree-1.png "\"in the room\" modifies the TP")
![](assets/examples/the-child-ate-an-apple-in-the-room-_tree-2.png "\"in the room\" modifies the DP")
