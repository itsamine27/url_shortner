CREATE TABLE work_space (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL
);

CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL,
    description VARCHAR(200) NOT NULL,
    work_id BIGINT,
    CONSTRAINT fk_work_id FOREIGN KEY (work_id) REFERENCES work_space(id) ON DELETE CASCADE
);

CREATE TABLE work_user (
    user_id INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    work_id INTEGER NOT NULL REFERENCES work_space(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, work_id)
);

CREATE INDEX indx_task_work_id ON tasks(work_id);