# Lesson 1: welcome and overview

## Why compilers?

Why study compilers?

Easy answer: make programmes go faster. This is the most visible change. Compiler optimisation.

Other answers: ...

Language implementation.
from theory to practice

Q: what goes into an implementation of a programming language?
Not just optimisation.

- Specific applications? What things are easy? What things do not need to be optimised.
- Data structures implemented.
- Target language (assembly/JavaScript etc..).
- Static analysis of programmes. Or just REPL?
- Does the checker need to terminate? E.g. dependently typed languages.
- Garbage collection? Any memory management?
- Bootstrapping compilers/intermediate steps?

Answers:

- Checking for errors at runtime.

This class is not going to be about parsing programmes.
But the deeper problems of semantics of programmes.

### The optimisation question

Proebstring's Law: a bit of a joke but perhaps is true.
The performance created by optimising compilers doubles every 18 years.
Moore's Law: number of transistors on a microchip doubles every two years.
Perspective of compilers vs transistors.

Compiler optimisations are important.
There is a rapid expansion in programming langaues.
E.g. R and tensorflow.
We want to match the way people think.

Also in the different types of machines that people would like to use.
Dozens of specialised hardwares in mobile telephones.

Overall there is an nxm scaling problem, where programming languages is n and number of hardware targets is m.
Both n and m are increasing quite rapidly!

Anecdote: computer arch conferences. Building lots of interesting hardware.
But few people know how to write compilers to unlock the potential.
We don't want to manually write assembly code.

The science of compilers has not caught up with either n or m.

So we should focus beyond speed:-

- correctness
- security
- debuggability

surprisingly hard to do!

## Guess about how this course will work

1. implementation tasks: follow up on things that we will learn
2. paper reading: research papers will be specified
3. leading paper discussions: not for self-guided but could write a blog post
4. course project: not for self-guided but could figure one out

For writing a blog post:

Your opinion.

- Any detailed background you think the audience needs.
- A detailed summary.
- Critical thinking about the merits and shortcomings of the work.
- Discussion of the work's role in history and its connections to the general computing landscape.
- A bulleted list of questions you have about the work.

## Notes

[SIGPLAN empirical evaluation checklist](./empirical-evaluation-guidelines.md)
