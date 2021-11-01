CREATE TABLE albums
(
    id      SERIAL PRIMARY KEY,
    artist  VARCHAR NOT NULL,
    title   VARCHAR NOT NULL,
    year    VARCHAR NOT NULL
);

insert into albums("artist", "title", "year") values ('foo', 'bar', '1999');