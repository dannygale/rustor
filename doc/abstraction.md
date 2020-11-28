
# Levels of Abstraction/Complexity:

## Object storage and block placement
1. All objects are stored contiguously in the lowest available group of LBAs
   that can contain the whole object on a single block device 
2. Objects are split to fill gaps left by deleted objects on a single block
   device
3. Blocks of an object are stored across multiple block devices on a single host
4. Blocks of an object are stored across multiple hosts

## Free list
1. Single Vector list of free space by starting LBA and size, 
2. Two Vector lists sharing data -- one ordered by location, the other ordered
   by size
3. Two B-Tree free lists sharing data -- one ordered by location, the other
   ordered by size

## Block Location
What level of abstraction is appropriate here? Should each block device get a
uuid? Probably just use serial number
Within a block device, we need LBA number
So a universal location can be defined by block device ID and LBA#


