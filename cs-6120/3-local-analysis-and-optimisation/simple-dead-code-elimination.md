# Simple dead code elimination

Different kinds of optimisations.

How do we deal with control flow in the programme?

1. Local (intraprocedural) optimisations work within a single basic block.
2. Global (intraprocedural) optimisations work on an entire function and have to deal with control flow. This is much more complicated. Though we don't reason about calls...
3. Interprocedural: involving multiple functions.

Dead code elimination: remove code that cannot possibly have an impact on the output/effects of the programme.

E.g. unused variables.

Q: construct a sentence that summarises the general principle behind optimising Bril programmes?
A: Modify the Bril code so that the programme producdes exactly the same outputs and performs identical side-effects but uses fewer resources.

This is a global optimisation. It is not possible to implement by just looking at a single basic block.

Q: Write the pseduo code for an optimisation that eliminates unused variables.

```python
var_usage: dict[str, int] = {}
for func in prog.functions:
    for instr in func.instrs:
        if instr.op == "call":
            for arg in instr.args:
                var_usage[arg] = var_usage.get(arg, 0) + 1

for func in prog.functions:
    for instr in func.instrs:
        if "dest" in instr and var_usage.get(instr.dest, 0) == 0:
            delete_op(instr)
```

If we consider the following programme

```
main {
    a: int = const 4;
    b: int = const 2;
    c: int = const 1;
    d: int = add a b;
    e: int = add c d;
    print d;
}
```

then the above pseudo code will only eliminate `e`.
Subsequent runs would then further eliminate variables.
(Iterate to convergence method.)
However we could replace the useage count with a usage set of variables.
Then in the middle we could perform a dead code elimination.

Consider

```
main {
    a: int = const 100;
    a: int = const 42;
    print a;
}
```

This will also not eliminate the code.
We need to check that the programme does nothing with this variable after the first assignment. The implementation is a bit more complicated.
We need to reason globally about the control flow of the programme.

We can reformulate this locally though:
we can delete any redefinitions within the same basic block so long as the variable is not used between the redefinitions.

```python
unused_vars: dict[str, int] = {}
final: dict[str, set[int]] = {}
for i, instr in enumerate(basic_block):
    if instr.op == "const" and instr.dest in unused_vars:
        tmp = final.get(instr.dest, set({}))
        tmp.add(unused_vars[instr.dest])
        final[instr.dest] = tmp
    elif instr.op == "const":
        unused_vars[instr.dest] = i
    elif inst.op == "call":
        for arg in instr.args:
            del unused_vars[arg]

for i, instr in enumerate(basic_block):
    if inst.op == "const" and i in final[instr.dest]:
        delete_op(instr)
```

Task: implement both of the optimisations above. Just concentrate on the above two passes.
Test your code on `examples/test/tdce/`.
