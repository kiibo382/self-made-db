# Self made RDBMS

Implemented with reference to the following two books.
 - https://www.amazon.co.jp/exec/obidos/ASIN/B085DZM79S/hatena-blog-22/
 - https://gihyo.jp/magazine/wdpress/archive/2021/vol122


## Implemented features
- disk & file
- memory
  - buffer
    - clock-sweep
- table
  - create
- row
  - insert
  - select
- simple query
- index (btree)

## Unimplemented features (Features to be added)
- memory
  - buffer
    - LRU cache
    - Effective Buffer Utilization
- table
  - mulitiple table
  - update
  - read (show table)
  - join
- row
  - update
  - delete
- metadata manage
- query
  - sql
  - tokenize
  - parse (LR(1))
  - plan
  - optimize
- transaction

## Processing Flow
table -> buffer -> disk -> heap file -> page file
