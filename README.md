A primitive sort of Graph Database in Rust, teaching myself Rust, graphs, and databases.

# Phase 1
* You can add nodes.
* Nodes have a name (string) and data (various types).
* You can link a node to another node once (no duplicate links).
* Links carry no metadata other than existence.
* Links are directional.
* You can query whether a node exists.
* You can query whether a link exists from node A to node B.
* You can get a list of all nodes N where X -> N.
* You can get a list of all nodes N where N -> X.

No effort is made to ensure that simultaneous queries and writes from multiple threads are consistent, ordered, etc. You won't get a crash (because all types involved are Send and Sync), but you may get incorrect/inconsistent results from queries if state is being mutated and queried simultaneously.

# Phase 2

* Links can carry metadata.
* Multiple links can exist with the same origin node and destination node.
* Links have their own identity.
* Links have types.
* Queries can return link information.

# Phase ?

* Queries can span multiple hops.
* Query DSL.
* Query parser.
* User-defined schemas.
* Schema validation and enforcement.
* Referential integrity.
* Transactions.
* REST API.
