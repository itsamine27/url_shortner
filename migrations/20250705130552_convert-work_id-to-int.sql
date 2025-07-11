-- Add migration script here
-- drop the old FK
ALTER TABLE tasks DROP CONSTRAINT fk_work_id;

-- convert type from BIGINT to INTEGER
ALTER TABLE tasks
  ALTER COLUMN work_id
  TYPE INTEGER
  USING work_id::INTEGER;

-- re‐add the FK
ALTER TABLE tasks
  ADD CONSTRAINT fk_work_id
  FOREIGN KEY (work_id)
  REFERENCES work_space(id)
  ON DELETE CASCADE;