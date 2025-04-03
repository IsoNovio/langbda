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
[0][parent: 0] Sentence--clause:statement
    [★ 42][parent: 43] ClauseP--clause:statement
        [★ 10][parent: 28] TenseP
            [9][parent: 10] TenseS--number:sg
                [★ 3][parent: 4] DetH--number:sg
                    [1][parent: 2] the
                [5][parent: 4] NounP--number:sg
                    [6][parent: 7] child
            [★ 11][parent: 10] TenseB
                [12] --> [9][parent: 13] LittleVS
                [★ 14][parent: 13] LittleVB--tense:past
                    [★ 16][parent: 17] VerbH--tense:past--transitive
                        [15][parent: 16] ate
                    [18][parent: 17] DetP--case:obj--number:sg
                        [★ 20][parent: 21] DetH--number:sg
                            [19][parent: 20] an
                        [22][parent: 21] NounP--number:sg
                            [23][parent: 24] apple
        [29][parent: 28] PrepP
            [★ 31][parent: 32] PrepH
                [30][parent: 31] in
            [33][parent: 32] DetP--number:sg
                [★ 36][parent: 37] DetH--number:sg
                    [34][parent: 35] the
                [38][parent: 37] NounP--number:sg
                    [39][parent: 40] room
    [44][parent: 43] Punct--clause:statement
        [45][parent: 46] .

Tree:
[0][parent: 0] Sentence--clause:statement
    [★ 42][parent: 43] ClauseP--clause:statement
        [9][parent: 10] TenseS--number:sg
            [★ 3][parent: 4] DetH--number:sg
                [1][parent: 2] the
            [5][parent: 4] NounP--number:sg
                [6][parent: 7] child
        [★ 11][parent: 10] TenseB
            [12] --> [9][parent: 13] LittleVS
            [★ 14][parent: 13] LittleVB--tense:past
                [★ 16][parent: 17] VerbH--tense:past--transitive
                    [15][parent: 16] ate
                [18][parent: 17] DetP--case:obj--number:sg
                    [★ 21][parent: 25] DetP--number:sg
                        [★ 20][parent: 21] DetH--number:sg
                            [19][parent: 20] an
                        [22][parent: 21] NounP--number:sg
                            [23][parent: 24] apple
                    [26][parent: 25] PrepP
                        [★ 28][parent: 29] PrepH
                            [27][parent: 28] in
                        [30][parent: 29] DetP--number:sg
                            [★ 33][parent: 34] DetH--number:sg
                                [31][parent: 32] the
                            [35][parent: 34] NounP--number:sg
                                [36][parent: 37] room
    [44][parent: 43] Punct--clause:statement
        [45][parent: 46] .
```
