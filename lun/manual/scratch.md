### Registers

-   One of the registers could be used to store the thread/core number? Timers?
-   Instructions to push/pop entire register set states on and off of the stack

### Hypervisor

-   Enable a "hypervisor" to schedule bounded work on cores (i.e. must return eventually)
-   Limit its access to a certain parts of memory

### Threading

-   Enable a single VM to have multiple "cores"

### Cores

-   In addition to a register bank, I think each core should have some working memory?
    Perhaps like, 4kb?

### Bus

-   What would a minimal "bus" implementation look like?

### Sys

-   What should the Lun binary/executable format look like?

### Instructions

-   Should instructions have condition predicates built in (like ARM)?

-   Instructions should be as wide reaching as possible. If something can already be done with
    a single instruction, don't make a more specific one. Sugar and shorthands can be handled
    at assembly time, so there's no need to make the decoding and runtime logic more complex
    than it needs to be by duplicating functionality.
