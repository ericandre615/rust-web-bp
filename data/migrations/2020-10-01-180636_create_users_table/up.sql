CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(340) NOT NULL,
    name_first VARCHAR(100),
    name_last VARCHAR(100)
);
