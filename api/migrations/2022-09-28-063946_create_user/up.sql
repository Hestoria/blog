-- Your SQL goes here
CREATE TABLE users {
    id uuid DEFAULT uuid_generate_v4 (),
    first_name varchar NOT NULL,
    last_name varchar NOT NULL,
    email varchar NOT NULL,
    PRIMARY KEY (id)
}