Find SQLite
===========

Find any valid SQLite database files.

Examples
--------

### Paths only

```
$ find-sqlite ~/.cargo/git
"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/sqlx-bench/test.db"
"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/tests/sqlite/sqlite.db"
```

### Paths with schemas

```
$ find-sqlite ~/.cargo/git -s
"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/sqlx-bench/test.db"
    CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL)

"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/tests/sqlite/sqlite.db"
    CREATE TABLE _sqlx_test ( id INT PRIMARY KEY, text TEXT NOT NULL )
    CREATE TABLE accounts ( id integer not null primary key, name text not null, is_active boolean )
    CREATE TABLE sqlite_sequence(name,seq)
    CREATE TABLE tweet ( id BIGINT NOT NULL PRIMARY KEY, text TEXT NOT NULL, is_sent BOOLEAN NOT NULL DEFAULT TRUE, owner_id BIGINT )
    CREATE VIEW accounts_view as select * from accounts
```

### Paths with metadata and schemas

```
$ find-sqlite ~/.cargo/git -ms
"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/sqlx-bench/test.db"
    meta
        btime 2023-05-26T16:38:10.361680209Z
        mtime 2023-05-26T16:38:10.361680209Z
        atime 2024-12-17T17:49:24.726921813Z
        size 776k
        perm rw-------
        owner 1000:1000
    schema
        CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL)

"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/tests/sqlite/sqlite.db"
    meta
        btime 2023-05-26T16:38:10.391665209Z
        mtime 2023-05-26T16:38:10.391665209Z
        atime 2024-12-17T17:49:24.730921799Z
        size 36k
        perm rw-------
        owner 1000:1000
    schema
        CREATE TABLE _sqlx_test ( id INT PRIMARY KEY, text TEXT NOT NULL )
        CREATE TABLE accounts ( id integer not null primary key, name text not null, is_active boolean )
        CREATE TABLE sqlite_sequence(name,seq)
        CREATE TABLE tweet ( id BIGINT NOT NULL PRIMARY KEY, text TEXT NOT NULL, is_sent BOOLEAN NOT NULL DEFAULT TRUE, owner_id BIGINT )
        CREATE VIEW accounts_view as select * from accounts
```

### Paths with metadata and schemas, pretty-formatted

```
$ find-sqlite ~/.cargo/git -msp
"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/sqlx-bench/test.db"
    meta
        btime 2023-05-26T16:38:10.361680209Z
        mtime 2023-05-26T16:38:10.361680209Z
        atime 2024-12-17T17:49:24.726921813Z
        size 776k
        perm rw-------
        owner 1000:1000
    schema
        CREATE TABLE test (id INTEGER PRIMARY KEY NOT NULL)

"/home/xand/.cargo/git/checkouts/sqlx-59ff20ee512773be/92a2268/tests/sqlite/sqlite.db"
    meta
        btime 2023-05-26T16:38:10.391665209Z
        mtime 2023-05-26T16:38:10.391665209Z
        atime 2024-12-17T17:49:24.730921799Z
        size 36k
        perm rw-------
        owner 1000:1000
    schema
        CREATE TABLE _sqlx_test (id INT PRIMARY KEY, text TEXT NOT NULL)
        CREATE TABLE accounts (
            id integer NOT NULL PRIMARY KEY,
            name text NOT NULL,
            is_active boolean
        )
        CREATE TABLE sqlite_sequence(name, seq)
        CREATE TABLE tweet (
            id BIGINT NOT NULL PRIMARY KEY,
            text TEXT NOT NULL,
            is_sent BOOLEAN NOT NULL DEFAULT TRUE,
            owner_id BIGINT
        )
        CREATE VIEW accounts_view AS
        SELECT
            *
        FROM
            accounts
```
