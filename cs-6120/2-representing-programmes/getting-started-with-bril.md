# Getting started with Bril

[Bril homepage](https://capra.cs.cornell.edu/bril)

[Bril GitHub](https://github.com/sampsyo/bril)

Big red intermediate language.

Get going with implementation more easily.
Indicative of real world compilers but sweeping rough edges under the carpet.
The nice thing is that it is extremely regular based on instructions and you can easily add extensions.

## A simple example

The canonical representation is a JSON document.
This means that we don't have to write a parser because most languages have already implemented a JSON parser.

Top level is a dictionary it has one key: `functions`.
The `main` function is the first one to be invoked.
The `.functions` list defines our functions, which in turn are objects with attributes like `name`, `instrs` and `args`.
Each `instr` is just an object usually containing an `op` key.
For precise details see the Bril homepage above.

So for instance the function

```
@main {
    y0: int = const 1;
    y1: int = const 2;
    y2: int = add y0 y1;
    print y2;
}
```

can be represented as

```json
{
  "functions": [
    {
      "name": "main",
      "instrs": [
        { "op": "const", "type": "int", "dest": "y0", "value": 1 },
        { "op": "const", "type": "int", "dest": "y1", "value": 2 },
        { "op": "add", "type": "int", "dest": "y2", "args": ["y0", "y1"] },
        { "op": "print", "args": ["y2"] }
      ]
    }
  ]
}
```

and indeed this course will provide you with a parser that takes the text representation and gives back the JSON representation.
The main idea of this course is to ignore parsing issues and focus on the JSON representation.

(Install Bril using Deno following the instructions in the GitHub repo.
Also install `bril2txt` and `bril2json` using `flit` and `Python3`.)

Tasks:-

1. Write a programme (in any language) to get the CFG of a Bril programme.
2. Write one additional analysis tool (e.g. count number of add instructions).
3. Write a new 'benchmark'. This is just a programme in the text representation that computes something appropriately mathematical.

Test against `bril/test/interp/core/jmp.bril`, which you will have to pipe into `bril2json`.

Ideas for CFGs:

- Form blocks from the `instrs` list in all functions.
- Calling function is not considered a terminator because the call will always return. Everything still executes. Hitting `ret` will terminate the basic block. This reflects the idea that a basic block is a list of instructions that will run all at once. (However the instructions within a function call are not counted.)
