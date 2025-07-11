-- Add migration script here
-- 1. Drop the existing foreign key constraint on tasks.work_id
ALTER TABLE tasks DROP CONSTRAINT fk_work_id;

-- 2. Alter the column type (only safe if all values fit in 32-bit signed int)
ALTER TABLE tasks
ALTER COLUMN work_id TYPE INTEGER
USING work_id::INTEGER;

-- 3. Recreate the foreign key constraint (now pointing to INTEGER id)
ALTER TABLE tasks
ADD CONSTRAINT fk_work_id FOREIGN KEY (work_id)
REFERENCES work_space(id) ON DELETE CASCADE;