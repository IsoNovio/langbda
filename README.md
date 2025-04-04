# Langbda

A sentence parser and grammaticality checker based on ideas of Lambda Calculus and Universal Grammar.

## Roadmap

- [ ] Write docs to explain how the theory works

## Write rules using Generative Grammar theories.

### Define a feature topology

```
[Features]
tense = present, past, future
number = sg, pl
person = 1st, 2nd, 3rd
case = subj, obj
clause = question, statement, exclamation
transitivity = transitive, intransitive
```

### Define how functional nodes merge

```
[Functional]
DetH = DetH-number
DetH-number = (NounP-number > DetP-number)
DetP = DetP-case
DetP-subj = TenseS
DetP-subj = LittleVS
DetP = (PrepP > DetP)

TenseH-transitivity = ((MOVED(LittleVH-transitivity) > LittleVP) >> TenseB)
TenseS = ((MOVED(LittleVS) > TenseB) >> TenseP)
TenseP = ClauseP
TenseP = (PrepP > TenseP)

LittleVH-transitivity = ((MOVED(VerbH-transitivity) > VerbP) >> LittleVB)
LittleVS = (LittleVB >> LittleVP)

VerbH-transitive = (DetP-obj > VerbP)
VerbH-intransitive = VerbP

ClauseP = ClauseP-clause
ClauseP-clause = (Punct-clause > Sentence-clause)

PrepH = (DetP > PrepP)
```

### Define a lexicon

```
[Lexical]
# det
the = DetH
an = DetH-sg

# nouns
child = NounP-sg
apple = NounP-sg
room = NounP-sg

# verbs
ate = TenseH-past-transitive
jumped = TenseH-past-intransitive

# prepositions
in = PrepH

# punctuations
. = Punct-statement
```

## Get your sentence parsed.

Below shows how the model captures the two possible interpretations of the sentence "The child ate an apple in the room.", which is created by the scoping ambiguity of the prepositional phrase "in the room".

![](examples/the-child-ate-an-apple-in-the-room-_tree-1.png "\"in the room\" modifies the TP")
![](examples/the-child-ate-an-apple-in-the-room-_tree-2.png "\"in the room\" modifies the DP")
