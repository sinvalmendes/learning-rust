# What Every Programmer Should Know About Memory - Ulrich Drepper

## CPU x RAM
The limits for modern hardware is memory speed. RAM did not increase its speed as much as the CPU all these years.
And there is a good reason for that, in order to make faster RAM to keep up with the modern CPU there are several orders of 
magnitude in terms of cost, so the RAM is still with just a few enhancements and with the dynamic model (DRAM).

However it is possible to add high-speed memory (SRAM) to computer without increasing the cost too much. Now the processors
will manipulate a certain amount of blazing fast memory to make temporary copies (cache) of data in the main memory.

This is possible because program code and data has temporal and spatial locality. This means that, over short periods of time,
there is a good chance that the same code or data gets reused (cache hit).

Realizing that locality exists is key to the concept of CPU caches as we use them today.

`Example`
```
Assume access to main memory takes 200 cycles and access to the cache memory take 15 cycles. Then code using 100 data elements 100 times each will spend 2,000,000 cycles on memory operations if there is no cache and only 168,500 cycles if all data can be cached. That is an improvement of 91.5%.
```

`CPU x Cache x Memory Big Picture`
```
    CPU <------> Cache <------> |BUS| <------> Main Memory
```

There is a difference between code and data. Both are stored in different parts of the Main Memory, and it seems that since 1993 Intel is using the same approach to manage Cache lanes, it means that there are different and independent caches for code and for data.

Depending on the processor layout (multi core, multi processor) there are different Cache layouts as well. The 3 level is a very common cache model, where L1, L2 and L3 are diferent cache lanes, with different sizes and distance to the CPU and Main Memory.

`3 level Cache`
```
    CPU <------> L1 <-> L2 <-> L3 <------> |BUS| <------> Main Memory

```

Temporal Locality
During the program execution, the processor will request data and code that are not in cache. The needed data will be retrieved from Main Memory. Because the chance of executing and processing the same data is high a copy will be made and stored in cache memory. Hence the processor will leverage the Temporal Locality, something will probably be used in the future (temporal) so it will get a copy closer to the processor (locality).

Spacial Locality
During the program execution, the processor will request data and code that are not in cache. The needed data will be retrieved from Main Memory. However, not only the requested data will be copied to the cache memory but also the neighboor data will be copied because the chance of executing and processing the same data and the neighboor is high. Hence the processor will leverage the Spacial Locality, something close (spacial) to the current data will probably be used in the future so it will get a copy closer to the processor (locality).