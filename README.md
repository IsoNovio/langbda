# Langbda

A sentence parser and grammaticality checker based on ideas of Lambda Calculus and Universal Grammar.

> [!WARNING]
> This project still needs countless optimizations, especially in the DFS parsing.

## Write simple grammar rules.
```
[Features]
# features
tense = present, past, future
number = sg, pl
person = 1st, 2nd, 3rd
case = subj, obj
clause = question, statement, exclamation

[Functional]
# add feature
D = D-case
D = D-number
Det = Det-number
C = C-statement

# left projection
D = P > D
Verb-transitive = D-obj > V
Prep = D > P
C = P > C

# left projection + agreement
Det-number = N-number > D
C-clause = Punct-clause > S

# right projection
D-subj-clause = V >> C-clause
Adj = N >> N

[Lexical]
# nouns
apple = N-sg
room = N-sg
child = N-sg

# D
who = D-question

# determinaters
the = Det
whose = Det-question

# verbs
ate = Verb-transitive-past

# prepositions
in = Prep

# punctuations
? = Punct-question
. = Punct-statement

# Adj
naughty = Adj
```


## Get your sentence parsed.
```
# "Whose naughty child ate the apple in the room?"
[
    [
        [
            [whose] 
            [
                [naughty] 
                [child]]] 
        [
            [ate] 
            [
                [
                    [the] 
                    [apple]] 
                [
                    [in] 
                    [
                        [the] 
                        [room]]]]]] 
    [?]]

[
    [
        [
            [
                [whose] 
                [
                    [naughty] 
                    [child]]] 
            [
                [ate] 
                [
                    [the] 
                    [apple]]]] 
        [
            [in] 
            [
                [the] 
                [room]]]] 
    [?]]
```

### And see endless features on the syntax tree.
```
There are 1235 parsing results:
Token Stack: 
Todo Stack: 

Tree:

[0 S, tense:past]
    [40 S, tense:past]
        [27 C, clause:question, tense:past]
            [13 C, clause:question, tense:past]
                [12 D, case:subj, clause:question]
                    [4 D, clause:question]
                        [3 Det, clause:question, number:sg]
                            [2 Det, clause:question]
                                [1 whose]   
                        [5 N, number:sg]
                            [8 N, number:sg]
                                [7 Adj]
                                    [6 naughty]  
                                [9 N, number:sg]
                                    [11 N, number:sg]
                                        [10 child]       
                [14 V, tense:past]
                    [17 V, tense:past]
                        [16 Verb, tense:past, transitive]
                            [15 ate]  
                        [18 D, case:obj]
                            [26 D, case:obj]
                                [22 D]
                                    [21 Det, number:sg]
                                        [20 Det]
                                            [19 the]   
                                    [23 N, number:sg]
                                        [25 N, number:sg]
                                            [24 apple]         
            [28 P]
                [31 P]
                    [30 Prep]
                        [29 in]  
                    [32 D]
                        [36 D]
                            [35 Det, number:sg]
                                [34 Det]
                                    [33 the]   
                            [37 N, number:sg]
                                [39 N, number:sg]
                                    [38 room]        
        [41 Punct, clause:question]
            [43 Punct, clause:question]
                [42 ?]     
```
