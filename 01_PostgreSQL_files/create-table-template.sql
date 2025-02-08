CREATE TABLE test_postgre (  
    id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    create_time DATE,
    name VARCHAR(255)
);
COMMENT ON TABLE  test_postgre IS '';
COMMENT ON COLUMN table_name test_postgre name IS '';
