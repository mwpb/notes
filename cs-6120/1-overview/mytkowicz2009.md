# Producting wrong data without doing anything obviously wrong!

Tood Mytkowicz
Amer Diwan
Matthias Hauswirth
Peter F. Sweeney

## Jargon

chipset = the traffic controller between the CPU, GPU, RAM, storage, and peripherals

SPEC CPU2006 = a standard set of benchmarks designed to stress a machine's CPU, memory, compilers and chipset.

O1, O2, O3 = subsets of optimisations available to C compilers. O3 contains all of O2 and O2 contains all of O1.

## Abstract

Measurement bias is common in computer system evaluation.
It occurs across multiple architectures, compilers and standard benchmarking sets.
Furthermore recent literature has invariably treated measurement bias in an inadequate way.
Measurement bias is also significant.
It can lead us to over-state and effect or even incorrect conclusions.

We describe a method for detecting measurement bias called casual analysis and a method for avoiding measurement bias called setup randomisation.

## Introduction

Experiments (and therefore measurements) are used to identify bottlenecks and determine what optimisations are effective.
For the latter we use measurements to compare the original system S with an augmented system S+O, which includes some additional attempted optimisations O.

However the setup of the experiment itself may create bias, which might cause a researcher to draw incorrect conclustions.

In this paper we consider the specific O3 optimisations and consider a two dimensional matrix of different experiments.
Firstly we vary the total size of the environment variables provided.
Secondly we vary the order of the linking of the `.o` files.
We measure the 'speedup', which we define as the runtime under O2 divided by the runtime under O3.

We also vary microprocessors and use both GCC and Intel's C compiler.

We show that varying the experimental setup leads to differing conclusions about the effectiveness of the optimisation.
Moreover there is no obvious pattern or order to the variation of this effectiveness.
Since hardware manufacturers do not reveal full details of their hardware it is also difficult to make theoretical predictions.

A literature survey of 133 recent papers does not adequately take measurement bias into account.
Many researchers use a multiple and varied workloads to try to account for measurement bias.
However this is often insufficient because of bias inherent to the standard benchmarking suites.
In this paper we show that SPEC CPU2006 is not diverse enough to cancel out measurement bias.

Therefore it is necessary to consider the randomisation of the experimental setups themselves, which is called 'setup randomisation'.
It is possible to detect measurement bias by performing a 'causal analysis', which evaluates the accuracy of a conclusion even in the presenceof measurement bias.
