CREATE SCHEMA secret
    AUTHORIZATION postgres
    CREATE TABLE memo (
        id INTEGER PRIMARY KEY,
        secret_body TEXT
    )
;
