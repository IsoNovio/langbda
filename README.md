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

TenseS = ((MOVED(LittleVS) >> TenseB) >> TenseP)
TenseP = ClauseP

LittleVS = (LittleVB >> LittleVP)
LittleVP = TenseB

VerbH-transitive = (DetP-obj > VerbP)
VerbP = LittleVB

ClauseP = ClauseP-clause
ClauseP-clause = (Punct-clause > Sentence-clause)
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

# verbs
ate = VerbH-past-transitive

# punctuations
. = Punct-statement
```


## Get your sentence parsed.
```
# Interpreting "the child ate an apple." as Sentence in English
[0][parent: 0] Sentence--clause:statement
    [★ 29][parent: 30] ClauseP--clause:statement
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
    [31][parent: 30] Punct--clause:statement
        [32][parent: 33] .
```
