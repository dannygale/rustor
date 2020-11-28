
# Overview
`rustor` provides a library and binary tools to create storage systems

# Key Components
1. `ObjectStore` -- the top level request handler. Manages data flow to fill reuests
2. `Object` -- sort of a meta-class that has the data and its metadata
3. `ObjKey` -- a unique identifier for an `Object`, including uuid, hash, and size
4. `KeyGen` -- generates `ObjKey`s
5. `FreeList` -- tracks available blocks
6. `Manifest` -- a selection of blocks that store the data of an `Object`
7. `KeyStore` -- stores and retrieves `ObjKey`s and their associated `Manifest` by uuid
8. `BlockStore` -- interfaces with `BlockDevice`s to read and write data

# Data Flows
## Single Block Device, Contiguous Objects
### PUT object
1. `ObjectStore` receives `PUT <object>` request
2. `ObjectStore` asks  `KeyGen` to create `ObjKey` for object -- uuid, hash, and size
3. `ObjectStore` asks `FreeList` to allocate space for the object
4. `Freelist` provides `Manifest` to `ObjectStore`
5. `ObjectStore` asks `KeyStore` to store `ObjKey` and `Manifest`
6. `ObjectStore` asks `BlockStore` to write object blocks according to `Manifest`
7. `ObjectStore` returns uuid to requester

### GET object
1. `ObjectStore` receives ``GET <uuid>`` request
2. `ObjectStore` asks `KeyStore` to retrieve `ObjKey` and `Manifest`
3. `ObjectStore` asks `BlockStore` to retrieve object blocks according to `Manifest`
4. `ObjectStore` returns `Object` to requester

### DELETE object
1. `ObjectStore` receives `DELETE <uuid>` request
2. `ObjectStore` asks `KeyStore` to retrieve `ObjKey` and `Manifest`
3. `ObjectStore` asks `FreeList` to release blocks according to `Manifest`
4. <optional> `ObjectStore` asks `BlockStore` to zero blocks according to `Manifest`

# Roadmap
[ ] Add free list B-tree
- Transition Keystore to a database backing
