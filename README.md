How to create a graph database

# Phase 1
You can add nodes.
Nodes have a name (string) and data (various types).
You can link a node to another node once (no duplicate links).
Links carry no metadata other than existence.
Links are directional.
You can query whether a node exists.
You can query whether a link exists from node A to node B.
You can get a list of all nodes N where X -> N.
You can get a list of all nodes N where N -> X.

No effort is made to ensure that simultaneous queries and writes from multiple threads are consistent, ordered, etc. You won't get a crash (because all types involved are Send and Sync), but you may get incorrect/inconsistent results from queries if state is being mutated and queried simultaneously.

# Phase 2

