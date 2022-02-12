# 自作RDBMS

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

## 処理フロー
table -> buffer -> disk -> heap file -> page file
