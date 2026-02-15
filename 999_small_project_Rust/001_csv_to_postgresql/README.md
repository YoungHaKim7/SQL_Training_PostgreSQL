# CSV import to postgres
- https://users.rust-lang.org/t/csv-import-to-postgres/11714

# Sample CSV data
- https://github.com/datablist/sample-csv-files


# pgcli로 편하게 LSP도움을 받자 

```bash
$ pgcli -h localhost -p 5432 postgres -d postgres
```

# PostgresSQL DB에 있는거 csv로 export

```sql
COPY customers(id, name, email, age)
FROM '/absolute/path/customers.csv'
DELIMITER ','
CSV HEADER;
```
