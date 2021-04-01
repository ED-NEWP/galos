[![CI](https://github.com/ED-NEWP/galos/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/ED-NEWP/galos/actions/workflows/ci.yml)

Somewhere between reality and the flight sim E:D.

### Testing

```sh
# All members of this workspace.
cargo test --all -- --test-threads=1
# Only `galos`.
cargo test -- --test-threads=1
# Only `elite_journal`.
cargo test -p elite_journal

# Individual tests

# Tests that use the database need to set test-threads to 1.
cargo test --tests -- --test-threads=1

cargo test --lib
cargo test --doc
```


### Database Setup

```sh
cargo install sqlx-cli

# Create the database and run the migrations.
cargo sqlx database setup --source galos_db/migrations/

# Run any pending migrations.
cargo sqlx migrate run --source galos_db/migrations/

# Drop, create, and migrate the whole thing.
cargo sqlx database reset --source galos_db/migrations/
```

### Database Backup and Restore

```sh
# Create a backup.
pg_dump -Fc elite_development > latest.dump

# Restore from backup.
pg_restore -Cd postgres < latest.dump
```
