# 自作RDBMS

下記の2つの書籍を参考に実装。
 - https://www.amazon.co.jp/exec/obidos/ASIN/B085DZM79S/hatena-blog-22/
 - https://gihyo.jp/magazine/wdpress/archive/2021/vol122


## 実装済みの機能
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

## 完成形
- disk & file
- memory
  - buffer
    - LRU cache
    - Effective Buffer Utilization
- table
  - mulitiple table
  - create
  - update
  - read (show table)
  - join
- row
  - insert
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
- index (btree)

## 処理フロー
table -> buffer -> disk -> heap file -> page file
