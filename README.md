<div align="center">

  <img src="docs/SeaQL logo dual.png" width="320"/>

  <h1>SeaSchema</h1>

  <p>
    <strong>🌿 SQL schema definition and discovery</strong>
  </p>

  [![crate](https://img.shields.io/crates/v/sea-schema.svg)](https://crates.io/crates/sea-schema)
  [![docs](https://docs.rs/sea-schema/badge.svg)](https://docs.rs/sea-schema)
  [![build status](https://github.com/SeaQL/sea-schema/actions/workflows/rust.yml/badge.svg)](https://github.com/SeaQL/sea-schema/actions/workflows/rust.yml)

  <sub>Built with ❤️ by 🌊🦀🐚</sub>

</div>

## About

SeaSchema is a library to help you manage database schema. It provides data structures for 
representing schema definitions and tools to discover and manipulate them.

For now, MySQL support is done and Postgres support is in progress. At the end, we'd want to 
support schema conversion between MySQL and Postgres.

## Architecture

You can use SeaSchema for many purposes, so the crate is divided into 5 features:

+ `def`: type definitions
+ `query`: query builder for querying information_schema
+ `parser`: for parsing information_schema (parsing sqldump is WIP)
+ `writer`: for writing `Schema` as `TableStatement` and SQL
+ `discovery`: query, parse and construct a `Schema`

JSON de/serialize on type definitions can be enabled with `with-serde`.

## Example

Take the MySQL [Sakila Sample Database](tests/sakila/sakila-schema.sql) as example, given the following table:

```SQL
CREATE TABLE film_actor (
  actor_id SMALLINT UNSIGNED NOT NULL,
  film_id SMALLINT UNSIGNED NOT NULL,
  last_update TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY  (actor_id,film_id),
  KEY idx_fk_film_id (`film_id`),
  CONSTRAINT fk_film_actor_actor FOREIGN KEY (actor_id) REFERENCES actor (actor_id) ON DELETE RESTRICT ON UPDATE CASCADE,
  CONSTRAINT fk_film_actor_film FOREIGN KEY (film_id) REFERENCES film (film_id) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

```

The [discovered schema result](tests/discovery/schema.rs):

```rust
TableDef {
    info: TableInfo {
        name: "film_actor",
        engine: InnoDb,
        auto_increment: None,
        char_set: Utf8Mb4,
        collation: Utf8Mb40900AiCi,
        comment: "",
    },
    columns: [
        ColumnInfo {
            name: "actor_id",
            col_type: SmallInt(
                NumericAttr {
                    maximum: None,
                    decimal: None,
                    unsigned: Some(
                        true,
                    ),
                    zero_fill: None,
                },
            ),
            null: false,
            key: Primary,
            default: None,
            extra: ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: false,
                generated: false,
                default_generated: false,
            },
            expression: None,
            comment: "",
        },
        ColumnInfo {
            name: "film_id",
            col_type: SmallInt(
                NumericAttr {
                    maximum: None,
                    decimal: None,
                    unsigned: Some(
                        true,
                    ),
                    zero_fill: None,
                },
            ),
            null: false,
            key: Primary,
            default: None,
            extra: ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: false,
                generated: false,
                default_generated: false,
            },
            expression: None,
            comment: "",
        },
        ColumnInfo {
            name: "last_update",
            col_type: Timestamp(
                TimeAttr {
                    fractional: None,
                },
            ),
            null: false,
            key: NotKey,
            default: Some(
                ColumnDefault {
                    expr: "CURRENT_TIMESTAMP",
                },
            ),
            extra: ColumnExtra {
                auto_increment: false,
                on_update_current_timestamp: true,
                generated: false,
                default_generated: true,
            },
            expression: None,
            comment: "",
        },
    ],
    indexes: [
        IndexInfo {
            unique: false,
            name: "idx_fk_film_id",
            parts: [
                IndexPart {
                    column: "film_id",
                    order: Ascending,
                    sub_part: None,
                },
            ],
            nullable: false,
            idx_type: BTree,
            comment: "",
            functional: false,
        },
        IndexInfo {
            unique: true,
            name: "PRIMARY",
            parts: [
                IndexPart {
                    column: "actor_id",
                    order: Ascending,
                    sub_part: None,
                },
                IndexPart {
                    column: "film_id",
                    order: Ascending,
                    sub_part: None,
                },
            ],
            nullable: false,
            idx_type: BTree,
            comment: "",
            functional: false,
        },
    ],
    foreign_keys: [
        ForeignKeyInfo {
            name: "fk_film_actor_actor",
            columns: [
                "actor_id",
            ],
            referenced_table: "actor",
            referenced_columns: [
                "actor_id",
            ],
            on_update: Cascade,
            on_delete: Restrict,
        },
        ForeignKeyInfo {
            name: "fk_film_actor_film",
            columns: [
                "film_id",
            ],
            referenced_table: "film",
            referenced_columns: [
                "film_id",
            ],
            on_update: Cascade,
            on_delete: Restrict,
        },
    ],
}
```
