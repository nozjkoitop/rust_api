CREATE TABLE IMAGES (
                        id          UUID PRIMARY KEY,
                        car_id      UUID NOT NULL
                            REFERENCES cars(id)
                                ON DELETE CASCADE,
                        url         TEXT NOT NULL,
                        created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);