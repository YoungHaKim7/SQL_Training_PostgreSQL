# 직접 만들면서 익히자


# PostgresSQL backup


```bash
pg_basebackup -D backup_folder -Fp -Xs -P
```
# 1. Summary (Clear Difference)

| Goal                  | Command         |
| --------------------- | --------------- |
| CSV → Table           | `\copy FROM`    |
| Table → CSV           | `\copy TO`      |
| DB → SQL file         | `pg_dump`       |
| Full physical DB copy | `pg_basebackup` |
| Edit raw DB files     | ❌ Never         |


# 2. Where PostgreSQL Stores Its Real DB Files

- PostgreSQL stores internal data files in its data directory.

- On Linux typically:

```bash
/var/lib/postgresql/<version>/main/
```

- On macOS (Homebrew):

```bash
/opt/homebrew/var/postgresql@<version>/
```

- Inside you'll see: (global에 저장 되나봄)

```bash
base/
global/
pg_wal/
```

# Visualizing Internal Storage

```bash
data_directory/
├── base/
│   ├── 16384/
│   │   ├── 12345
│   │   ├── 12346
│
├── global/
├── pg_wal/

```
