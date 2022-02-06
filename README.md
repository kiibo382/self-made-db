# DB for ML

## 実装済みの機能
- table
  - create
- row
  - insert
  - select
- secondry index

## 完成形
- exec sql
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
- buffer
  - clock-sweep -> LRU cache

## 構成
table -> buffer -> disk -> heap file -> page file
