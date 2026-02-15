# Result

```bash

```

# Check DB

- Based on your code, the database is `customers_db`, the user is `gy`, and the table is `customers`. Here are several ways to check the data:

## 1. Connect and query with psql

```bash
psql -U gy -d customers_db -c "SELECT * FROM customers LIMIT 10;"
```

## 2. Enter interactive psql session

```bash
psql -U gy -d customers_db
```

Then inside psql:
```sql
-- See all tables
\dt

-- Count rows
SELECT COUNT(*) FROM customers;

-- View first 10 rows
SELECT * FROM customers LIMIT 10;

-- View specific columns
SELECT first_name, last_name, email FROM customers LIMIT 5;

-- Exit
\q
```

## 3. Check row count directly

```bash
psql -U gy -d customers_db -c "SELECT COUNT(*) FROM customers;"
```

<hr />

# AI fix

- Done! Successfully imported **1000 customers** from `customers-1000.csv` into PostgreSQL.

**What was fixed:**
1. Updated from old `postgres::Connection` API to modern `postgres::Client::connect` (v0.19)
2. Rewrote code for the correct CSV structure (11 customer columns vs. original 29 flight columns)
3. Created proper `customers` table with matching schema
4. Added `mut` to transaction for multiple execute calls

**Database:** `customers_db` | **Table:** `customers` | **Records:** 1000



