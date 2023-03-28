CREATE TABLE userContext(
    id serial PRIMARY KEY,
    first_name varchar NOT NULL,
    user_login varchar UNIQUE NOT NULL,
    user_password_hash varchar NOT NULL
);

