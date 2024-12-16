Find SQLite
===========

Find any valid SQLite database files.

Examples
--------

### Just files

```
$ find-sqlite ~/.config/discord
"/home/xand/.config/discord/databases/Databases.db"
"/home/xand/.config/discord/Cookies"
"/home/xand/.config/discord/WebStorage/QuotaManager"
"/home/xand/.config/discord/Trust Tokens"
"/home/xand/.config/discord/Shared Dictionary/db"
```

### With schemas

```
$ find-sqlite ~/.config/discord -s
"/home/xand/.config/discord/databases/Databases.db"
CREATE INDEX origin_index ON Databases (origin)
CREATE TABLE Databases (id INTEGER PRIMARY KEY AUTOINCREMENT, origin TEXT NOT NULL, name TEXT NOT NULL, description TEXT NOT NULL, estimated_size INTEGER NOT NULL)
CREATE TABLE meta(key LONGVARCHAR NOT NULL UNIQUE PRIMARY KEY, value LONGVARCHAR)
CREATE TABLE sqlite_sequence(name,seq)
CREATE UNIQUE INDEX unique_index ON Databases (origin, name)

"/home/xand/.config/discord/Cookies"
CREATE TABLE cookies(creation_utc INTEGER NOT NULL,host_key TEXT NOT NULL,top_frame_site_key TEXT NOT NULL,name TEXT NOT NULL,value TEXT NOT NULL,encrypted_value BLOB NOT NULL,path TEXT NOT NULL,expires_utc INTEGER NOT NULL,is_secure INTEGER NOT NULL,is_httponly INTEGER NOT NULL,last_access_utc INTEGER NOT NULL,has_expires INTEGER NOT NULL,is_persistent INTEGER NOT NULL,priority INTEGER NOT NULL,samesite INTEGER NOT NULL,source_scheme INTEGER NOT NULL,source_port INTEGER NOT NULL,is_same_party INTEGER NOT NULL,last_update_utc INTEGER NOT NULL)
CREATE TABLE meta(key LONGVARCHAR NOT NULL UNIQUE PRIMARY KEY, value LONGVARCHAR)
CREATE UNIQUE INDEX cookies_unique_index ON cookies(host_key, top_frame_site_key, name, path)

"/home/xand/.config/discord/WebStorage/QuotaManager"
CREATE INDEX buckets_by_expiration ON buckets(expiration)
CREATE INDEX buckets_by_host ON buckets(host, type)
CREATE INDEX buckets_by_last_accessed ON buckets(type, last_accessed)
CREATE INDEX buckets_by_last_modified ON buckets(type, last_modified)
CREATE TABLE "buckets"(id INTEGER PRIMARY KEY AUTOINCREMENT, storage_key TEXT NOT NULL, host TEXT NOT NULL, type INTEGER NOT NULL, name TEXT NOT NULL, use_count INTEGER NOT NULL, last_accessed INTEGER NOT NULL, last_modified INTEGER NOT NULL, expiration INTEGER NOT NULL, quota INTEGER NOT NULL, persistent INTEGER NOT NULL, durability INTEGER NOT NULL) STRICT
CREATE TABLE meta(key LONGVARCHAR NOT NULL UNIQUE PRIMARY KEY, value LONGVARCHAR)
CREATE TABLE sqlite_sequence(name,seq)
CREATE UNIQUE INDEX buckets_by_storage_key ON buckets(storage_key, type, name)

"/home/xand/.config/discord/Trust Tokens"
CREATE TABLE meta(key LONGVARCHAR NOT NULL UNIQUE PRIMARY KEY, value LONGVARCHAR)
CREATE TABLE trust_tokens_issuer_config ( key TEXT, proto BLOB, PRIMARY KEY(key))
CREATE TABLE trust_tokens_issuer_toplevel_pair_config ( key TEXT, proto BLOB, PRIMARY KEY(key))
CREATE TABLE trust_tokens_toplevel_config ( key TEXT, proto BLOB, PRIMARY KEY(key))
CREATE VIEW MmapStatus (value) AS SELECT -1

"/home/xand/.config/discord/Shared Dictionary/db"
CREATE INDEX exp_time_index ON dictionaries(exp_time)
CREATE INDEX isolation_index ON dictionaries(frame_origin,top_frame_site)
CREATE INDEX last_used_time_index ON dictionaries(last_used_time)
CREATE INDEX token_index ON dictionaries(token_high, token_low)
CREATE INDEX top_frame_site_index ON dictionaries(top_frame_site)
CREATE TABLE dictionaries(id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,frame_origin TEXT NOT NULL,top_frame_site TEXT NOT NULL,host TEXT NOT NULL,match TEXT NOT NULL,url TEXT NOT NULL,res_time INTEGER NOT NULL,exp_time INTEGER NOT NULL,last_used_time INTEGER NOT NULL,size INTEGER NOT NULL,sha256 BLOB NOT NULL,token_high INTEGER NOT NULL,token_low INTEGER NOT NULL)
CREATE TABLE meta(key LONGVARCHAR NOT NULL UNIQUE PRIMARY KEY, value LONGVARCHAR)
CREATE TABLE sqlite_sequence(name,seq)
CREATE UNIQUE INDEX unique_index ON dictionaries(frame_origin,top_frame_site,host,match)

```
