-- Add migration script here
ALTER TABLE tasks
  ALTER COLUMN work_id SET NOT NULL;