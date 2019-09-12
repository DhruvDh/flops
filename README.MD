# Assignment 1: Benchmarking Flops and Iops

## I. Model

1. What is a compute node of mamba composed of? What kind of processor it has? Which architecture is the processor built on? (For instance, some core i7 are based on the ivy-bridge architecture, some on the haswell architecture, ...)

Dual Intel Xeon 3.2 GHz 8-core processors – E5-2667 v3 or v4 - Haswell (v3) and Broadwell (v4)

2. What is the maximum number of floating point operations this machine can perform per second?

Both Haswell and Broadwell can do -

- 16 64-bit FLOPs/cycle (two 4-wide FMA instructions)
- 32 32-bit FLOPs/cycle (two 8-wide FMA instructions)

Base frequency is 3.2 GHz for v3 and v4 also, so 3.2 x 10^9 cycles per second, and thusly either 16 x 3.2 x 10^9 64-bit FLOPs per cycle per core or 32 x 3.2 x 10^9 32-bit FLOPs per cycle per core; which comes to 51.2 GFLOPs or 102.4 FLOPs per cycle per core.

In all, accounting for the 8 cores brings the number to 409.6 GFLOPs/sec (64-bit) or 819.2 GFLOPs/sec (32-bit) at base frequency, assuming perfect thermals and no thermal throttling. Trying to account for thermals is pointless at best on account of so many variables involved.

3. What is the maximum number of integer operations this machine can perform per second?

Both Haswell and Broadwell can do -

- 13 64-bit IOPs/cycle (three 4-wide vector ALU instructions + 1 integer ALU instruction)
- 25 32-bit IOPs/cycle (three 8-wide vector ALU instructions + 1 integer ALU instruction)

In all, accounting for the 8 cores brings the number to 332.8 GIOPs/sec (64-bit) or 665.6‬ GIOPs/sec (32-bit) at base frequency, assuming perfect thermals and no thermal throttling. Trying to account for thermals is pointless at best on account of so many variables involved.

## II. Benchmarking

### Floating point operations

To achieve max flops, in a loop that iterations 10 billion times, I ran 8 FMA operations on 32-bit 8 lane SIMD arrays, coming to (`8 lanes * 8 FMAs * 2 flop per FMA`) `128` flops per iteration; one such loop per thread, for 16 threads, coming to `16 * 128 * 10^10` ( = `2.048*10^14`) flops in total.

And `2.048e14` flops took `16.788439` seconds on MAMBA, coming to `1219.8871` GFLOPs. 