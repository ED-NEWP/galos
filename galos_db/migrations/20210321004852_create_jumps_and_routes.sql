CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE Fuel AS ENUM ('Tritium', 'Hydrogen');

CREATE TABLE jumps (
    id                      uuid       PRIMARY KEY,
    current_system_address  bigint     NOT NULL REFERENCES systems(address),
    distance                real,
    fuel_used               real,
    fuel_level              real,
    fuel_type               Fuel,
    future                  boolean    NOT NULL,
    timestamp               timestamp,

    /* jumps, systems, and routes.
     *
     * r1: name
     * v start_jump_id   v     v     v
     * s1                s2    s3    s4
     * ^ current_system  ^     ^     ^
     * j1 -next_jump_id> j2 -> j3 -> j4
     *
     * Loop.
     * j1 -next_jump_id> j2 -> j3 -> ... -> j1
     */
    next_jump_id            uuid    REFERENCES jumps(id)
);

/* Named jump route. */
CREATE TABLE routes (
    id             uuid     PRIMARY KEY,
    name           varchar,
    start_jump_id  uuid     NOT NULL REFERENCES jumps(id)
);
