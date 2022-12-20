# Representing programmes

What does this mean?
From the perspective of a compiler.
What thing do we consider to be the programme?

One option: Concrete syntax for programmers. Simply the text the programmer puts into the file.

Pros:-

- Human readable.
- Easy to version control.
- ALSO: easy to write.

Cons:-

- Hard for the machine to understand.
- Usually takes up much more disk space than other representations.
- ALSO: not easy to transform according to the semantics of the programme.

Another option: Abstract syntax (tree).
A tree that breaks up the code into 'atoms' and relationships.
Pros:-

- Much easier for the compiler to transform.
- ALSO: perfect for writing an interpreter.

Cons:-

- Still takes up quite a bit of space.
- Doesn't correspond to the target language.
- ALSO: all of the nodes have different types of behviour.

Out final choice: Represent a programme as lists of instructions.
An idealised assembly language.
This is the type of representation that underlies LLVM.
Actually this is another sort of concrete syntax.
We can create an abstract syntax tree but it would be very elementary.
It would be very shallow: essentially a list of children with a small number of grandchildren that are leaves.
Pros:-

- More regular.
- Very few types of entity.
- Instructions are of regular form.

Cons:-

- Very inconvenient for humans.
- Hard to reason about semantically.

## Extracting structure

Control flow graph (CFG).
A directed graph.
Nodes are instructions; edges are possible control flow.
Exactly one entry vertex and one exit vertex.

Therefore imperative flows look like a sequence of composable arrows.
Conditionals look like a node with two children: label the branches True or False for instance.
Usually we label the initial node as ENTRY.

The structure of CFG can tell you that there is some code that is useless and can be deleted.

CFGs look redundant because there are many imperative flows.
We coarsen the CFG by using basic blocks.
A basic block is a sequence of instructions where jumps and branches can only happen once at the end.
(An instruction that is either a jump or a branch is called a terminator.)
Also all other jumps or branches can only transfer control to the beginning of a basic block.
(mwpb: after reading several different treatments of basic blocks, some authors seems to want maximal basic blocks, which for instance do not end at a label that is never used.)

How to extract a CFG from a list of instructions?
First we get the basic blocks.
Let's write some pseudo code for a function that takes a list of instructions and labels and outputs just the blocks.

```python
def is_terminator(instruction: Instruction) -> bool:
    """A terminator is a branch or a jump."""
    return instruction.startswith("br") or instruction.startswith("jp")


def is_label(instruction: Instruction) -> bool:
    pass


def get_basic_blocks(instructions: list[Instruction]) -> list[list[Instruction]]:
    basic_blocks: list[list[Instruction]] = []
    current_block: list[Instruction] = []
    for instruction in instructions:
        if not is_label(instruction):
            current_block.append(instruction)
        if is_terminator(instruction):
            basic_blocks.append(current_block)
            current_block = []

    return basic_blocks
```

In order to create a CFG we need to know in addition a map from labels to basic blocks.
Our output will be a mapping from labels to lists of labels that specifies the successor block(s).
There will be multiple if we have a branch.

```python
def get_cfg(basic_blocks: dict[str, list[Instruction]]) -> dict[str, list[str]]:
    cfg: dict[str, list[str]] = {}
    previous_block = None
    for label, instructions in basic_blocks:
        if previous_block:
            cfg[previous_block] = label
        terminator = instructions[-1]
        if terminator.startswith("br"):
            cfg[label] = [terminator["true"], terminator["false"]]
            prevous_block = None
        elif terminator.startswith("jp"):
            cfg[label] = terminator["target"]
            prevous_block = None
        else:
            prevous_block = label

    return cfg
```
