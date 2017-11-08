
CREATE TABLE location_entries (
    id SERIAL PRIMARY KEY,
    checked_on TIMESTAMPTZ NOT NULL,
    location GEOMETRY(Point, 26910) NOT NULL,
    by_user INTEGER REFERENCES users (user_id)
)

-- Add a spatial index
-- CREATE INDEX location_entries_gix
 --  ON location_entries
 --  USING GIST (location);
