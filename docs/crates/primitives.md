# Chronicle-Primitives
-------------------------


**Chronicle-Primitives** is a shared library that contains common utilities and data structures used across the Chronicle.

This holds data structures and functions classified into:
1. DB
2. Errors
3.Indexer
4. Interfaces
and so on


This `indexer` categorizes the following data structures:
1. `ChronicleEvent`: This is a data structure that represents the event data. All event logs coming from other chains is translated into this data structure before it is been stored in the DB.
2. `DisplayChronicleEvent`: This sturcture is very similar to `ChronicleEvent` but it is used to display the event data in the GraphQL interface a nd this is the stucture that is been stored in the DB.
3. `ChronicleTransaction`: This is a data structure that represents the transaction data. All transactions coming from other chains is translated into this data structure before it is been stored in the DB.
4. `ChronicleIndexingMode`: This is an enum that is used to indicate that an indexing task is indexing and event or a transaction.


The `Interfaces`these are just traits defining the interface of the Indexer bother for Events and Transactions. These traits are implemented by the `EventIndexer` and `TransactionIndexer` respectively.
