# Langbda
A sentence parser and grammaticality checker based on ideas of Lambda Calculus and Universal Grammar.


## Roadmap
- Add support for >= 1 MOVED tags
- Write docs to explain how the theory works


## Write rules using Generative Grammar theories.
### Define a feature topology
```
[Features]
# features
tense = present, past, future
number = sg, pl
person = 1st, 2nd, 3rd
case = subj, obj
clause = question, statement, exclamation
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

TenseS = ((MOVED(LittleVS) >> TenseB) >> TenseP)
TenseP = ClauseP
TenseP = (PrepP > TenseP)

LittleVS = (LittleVB >> LittleVP)
LittleVP = TenseB

VerbH-transitive = (DetP-obj > VerbP)
VerbP = LittleVB

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
ate = VerbH-past-transitive

# prepositions
in = PrepH

# punctuations
. = Punct-statement
```


## Get your sentence parsed.
Note that the parser supports branching.
```
# Interpreting "the child ate an apple in the room." as Sentence in English

Tree:
[0] Sentence--clause:statement
    [★ 42] ClauseP--clause:statement
        [9] TenseS--number:sg
            [★ 3] DetH--number:sg
                [1] the
            [5] NounP--number:sg
                [6] child
        [★ 11] TenseB
            [12] --> [9] LittleVS
            [★ 14] LittleVB--tense:past
                [★ 16] VerbH--tense:past--transitive
                    [15] ate
                [18] DetP--case:obj--number:sg
                    [★ 21] DetP--number:sg
                        [★ 20] DetH--number:sg
                            [19] an
                        [22] NounP--number:sg
                            [23] apple
                    [26] PrepP
                        [★ 28] PrepH
                            [27] in
                        [30] DetP--number:sg
                            [★ 33] DetH--number:sg
                                [31] the
                            [35] NounP--number:sg
                                [36] room
    [44] Punct--clause:statement
        [45] .

Tree:
[0] Sentence--clause:statement
    [★ 42] ClauseP--clause:statement
        [★ 10] TenseP
            [9] TenseS--number:sg
                [★ 3] DetH--number:sg
                    [1] the
                [5] NounP--number:sg
                    [6] child
            [★ 11] TenseB
                [12] --> [9] LittleVS
                [★ 14] LittleVB--tense:past
                    [★ 16] VerbH--tense:past--transitive
                        [15] ate
                    [18] DetP--case:obj--number:sg
                        [★ 20] DetH--number:sg
                            [19] an
                        [22] NounP--number:sg
                            [23] apple
        [29] PrepP
            [★ 31] PrepH
                [30] in
            [33] DetP--number:sg
                [★ 36] DetH--number:sg
                    [34] the
                [38] NounP--number:sg
                    [39] room
    [44] Punct--clause:statement
        [45] .
```
