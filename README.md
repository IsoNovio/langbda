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
Note that the parser supports branching.
```
Interpreting "the child ate an apple in the room." as Sentence in English
LANGBDA found 2 interpretations.
Tree:
[0] Sentence--clause:statement
    [46] ClauseP--clause:statement
        [32] TenseP
            [10] TenseP
                [9] TenseS--number:sg
                    [8] DetP--case:subj--number:sg
                        [3] DetH--number:sg
                            [1] the
                        [5] NounP--number:sg
                            [6] child
                [11] TenseB
                    [14] TenseH--tense:past--transitivity:transitive
                        [13] ate
                    [16] LittleVP
                        [12] --> [9] LittleVS
                        [19] LittleVB
                            [17] --> [14] LittleVH--transitivity:transitive
                            [21] VerbP
                                [22] --> [17] VerbH--transitivity:transitive
                                [24] DetP--case:obj--number:sg
                                    [26] DetH--number:sg
                                        [25] an
                                    [28] NounP--number:sg
                                        [29] apple
            [33] PrepP
                [35] PrepH
                    [34] in
                [37] DetP--number:sg
                    [40] DetH--number:sg
                        [38] the
                    [42] NounP--number:sg
                        [43] room
    [48] Punct--clause:statement
        [49] .

Tree:
[0] Sentence--clause:statement
    [46] ClauseP--clause:statement
        [10] TenseP
            [9] TenseS--number:sg
                [8] DetP--case:subj--number:sg
                    [3] DetH--number:sg
                        [1] the
                    [5] NounP--number:sg
                        [6] child
            [11] TenseB
                [14] TenseH--tense:past--transitivity:transitive
                    [13] ate
                [16] LittleVP
                    [12] --> [9] LittleVS
                    [19] LittleVB
                        [17] --> [14] LittleVH--transitivity:transitive
                        [21] VerbP
                            [22] --> [17] VerbH--transitivity:transitive
                            [24] DetP--case:obj--number:sg
                                [27] DetP--number:sg
                                    [26] DetH--number:sg
                                        [25] an
                                    [28] NounP--number:sg
                                        [29] apple
                                [32] PrepP
                                    [34] PrepH
                                        [33] in
                                    [36] DetP--number:sg
                                        [39] DetH--number:sg
                                            [37] the
                                        [41] NounP--number:sg
                                            [42] room
    [48] Punct--clause:statement
        [49] .
```
