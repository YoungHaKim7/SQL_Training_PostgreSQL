-- change column type
ALTER TABLE roomtwo 
    ALTER COLUMN id TYPE integer;
-- change column name
ALTER TABLE roomtwo 
    RENAME COLUMN id TO id;
-- Change column comment
COMMENT ON COLUMN roomtwo.id IS 'comment';
