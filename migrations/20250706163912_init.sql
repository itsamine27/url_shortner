CREATE TABLE url (
    id SERIAL PRIMARY KEY,
    long_url VARCHAR(2048) UNIQUE NOT NULL,
    short_url VARCHAR(7) NOT NULL
);

CREATE INDEX index_name ON url (short_url);