# Langbda

A sentence parser and grammaticality checker based on ideas of Lambda Calculus and Universal Grammar.

## Roadmap

- [ ] Support branching on `receive()` and `decide()`
  - `(A -> (B -> C)) <=> (B -> (A -> C))`
    - Redefine `Node::Lambda` as `{ from: Vec<Value>, to: Value }`
  - `((A -> B) -> (C -> D)) ==> ((C -> A) -> (B -> D)) OR (A, (B -> (C -> D)))`
    - Default to `(A, (B -> (C -> D)))`
    - Add `Node::From` to denote dangling `(A ->)`
      - `(A ->)` is produced as a decomposition of `([A, B] -> B))`
      - `(A ->)` automatically attaches to a newly inserted node
- [ ] Write docs to explain how the theory works
- [ ] Not planned: Add a Lexicon Debug Helper
  - Detect unreachable entries
  - Detect possible cycles
- [ ] Not planned: Add a NOVALUE value to every category
  - Possibly with new syntax `!`

## Write rules using Generative Grammar theories.

[Example lexicon for English](assets/lexicons/en.lexicon)

## Get your sentence parsed.

Below shows how the model captures the two possible interpretations of the sentence "The child ate an apple in the room.", which is created by the scoping ambiguity of the prepositional phrase "in the room".

![](assets/examples/the-child-ate-an-apple-in-the-room-_tree-1.png "\"in the room\" modifies the TP")
![](assets/examples/the-child-ate-an-apple-in-the-room-_tree-2.png "\"in the room\" modifies the DP")
