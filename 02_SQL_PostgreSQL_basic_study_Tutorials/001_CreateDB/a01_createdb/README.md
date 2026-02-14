# Result

```bash

```

## Solution 4: Try connecting with different methods

Try these alternatives:

```bash
# Method 1: Use PGPASSWORD variable (not secure but quick)
PGPASSWORD=your_password psql -h localhost -d postgre -U gy -p 5432 -f ./make_db.sql

# Method 2: Connect via Unix socket (if available)
psql -d postgre -U gy -f ./make_db.sql

# Method 3: Use connection string
psql "postgresql://gy:your_password@localhost:5432/postgre" -f ./make_db.sql
```

## Quick Diagnosis

What's your actual password? Try this to test the connection:

```bash
# Test connection interactively
psql -h localhost -d postgre -U gy -p 5432
```

This will prompt for password interactively. If this works, then the issue is with how you're passing the password in the script.

# fix

**Which solution would you like to try, or do you know the correct password for user "gy"?**


Fixed! The database `shop_practice02` was created successfully by connecting as the `postgres` superuser.

**Summary:**
- The original command failed because user `gy` lacks `CREATEDB` privilege
- Solution: connect as `postgres` user (or grant `CREATEDB` to `gy`)

**For future reference**, to grant `CREATEDB` privilege to `gy`:
```bash
sudo -u postgres psql -c "ALTER USER gy CREATEDB;"
```

Then you can run your original command with `-U gy`.
