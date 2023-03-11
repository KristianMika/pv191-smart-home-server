CREATE TABLE measurement(
    id serial PRIMARY KEY,
    temperature real,
    humidity integer,
    voc_index integer,
    measurement_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

