CREATE TABLE IF NOT EXISTS lists (
  id uuid primary key,
  version uuid NOT NULL,
  title text
);

CREATE TABLE IF NOT EXISTS sights (
  id uuid primary key,
  version uuid NOT NULL,
  title text NOT NULL,
  lat FLOAT8 NOT NULL,
  long FLOAT8 NOT NULL,
  description text,
  list_id uuid references lists(id) NOT NULL
);
