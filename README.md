cs4414-ps4
==========

**Goal:** Develop a module for Rust 0.8 that can be used for 
true random number generation. Compare the performance of our
random number generators with traditional pseudo-random number 
generators. Compare the strength of our random number generators
with traditional pseudo-random number generators. 

**Deliverables:** A Rust module containing four publicly callable
functions: 
	wrand() -> uint, 
	wrandN(n: uint) -> BigUint 
	srand() -> Option<uint>
	srandN(n: uint) -> Option<BigUint>
where the wrand*() functions (for weak random) return random numbers 
generated from the local operating system.  Entropy in these functions 
is collected from user I/O timings, interrupt timings, and other like sources.
The srand*() functions (for strong random), rely on assembly instructions
available on Intel Ivy Bridge processors to return random numbers generated
from entropy collected by monitoring the quantum effects of silicon on-board
the chip. If no Ivy Bridge processor is available, these functions return a 
None value. 

**Implementation:** On Windows, the wrand() functions use the CryptGenRandom()
function callable through the advapi32.dll. These function calls are externed
in the Rust module (FUNCTIONALITY NOT YET WORKING). On Mac/Unix, the wrand()
functions rely on reads from /dev/random. These calls block until enough
entropy is available to fulfill the request and tend to be very slow. The
srand() functions are composed of the Intel RdRand assembly instruction,
only available on Intel Ivy Bridge processors.

**Data:**

Variable-Bit Timings
<img src="https://github.com/wbkostan/cs4414-ps4/blob/master/img/VariableBitChart.PNG">

32-Bit Timings
<img src="https://github.com/wbkostan/cs4414-ps4/blob/master/img/32BitAll.PNG">

32-Bit Zoomed In
<img src="https://github.com/wbkostan/cs4414-ps4/blob/master/img/32BitRelevent.PNG">

