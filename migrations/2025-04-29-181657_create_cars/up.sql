CREATE TABLE CARS (
                      id uuid PRIMARY KEY,
                      make VARCHAR NOT NULL,
                      model VARCHAR NOT NULL,
                      year INT NOT NULL,
                      price NUMERIC(10,2) NOT NULL,
                      created_at TIMESTAMP NOT NULL DEFAULT NOW()
);