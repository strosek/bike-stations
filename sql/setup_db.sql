CREATE DATABASE IF NOT EXISTS stations;

DROP TABLE information;
DROP TABLE status;

CREATE TABLE information
(
    id        STRING NOT NULL PRIMARY KEY,
    name      STRING,
    address   STRING,
    latitude  FLOAT,
    longitude FLOAT
);
CREATE TABLE status
(
    station_id          STRING    NOT NULL PRIMARY KEY,
    is_returning        BOOL,
    is_renting          BOOL,
    is_installed        BOOL,
    num_docks_available INT,
    num_bikes_available INT,
    last_reported       TIMESTAMP
);

-- INSERT INTO information VALUES ('some-id-01', 'some-station-name', 'some-address', 12.3, 45.6);
-- INSERT INTO information VALUES ('some-id-02', 'some-station-name2', 'some-address2', 32.1, 65.4);
--
-- INSERT INTO status VALUES ('some-id-01', true, false, true, 3, 2, 1726066259::timestamp);
-- INSERT INTO status VALUES ('some-id-02', false, true, true, 5, 3, 1726066159::timestamp);


SELECT * FROM information;
SELECT count(*) FROM information LIMIT 10;
SELECT * FROM status LIMIT 10;
SELECT count(*) FROM status;
