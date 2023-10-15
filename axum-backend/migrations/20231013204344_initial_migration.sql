CREATE TABLE IF NOT EXISTS users (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  mail VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(100) UNIQUE NOT NULL,
  password_hash VARCHAR(100) NOT NULL,
  role VARCHAR(50) NOT NULL DEFAULT 'User',
  verified BOOLEAN NOT NULL DEFAULT FALSE,
  verification_code VARCHAR(255),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
); 

CREATE INDEX IF NOT EXISTS users_verification_code_idx ON users (verification_code);

CREATE TABLE IF NOT EXISTS categories (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name VARCHAR(100) UNIQUE NOT NULL,
  reimburstment_percentage DECIMAL(5,2) NOT NULL,
  max_reimburstment NUMERIC NOT NULL
);

CREATE TABLE IF NOT EXISTS claims (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users (id),
  total_cost NUMERIC,
  reimburstment NUMERIC,
  status VARCHAR(30) NOT NULL DEFAULT 'Pending'
);

CREATE INDEX IF NOT EXISTS claims_user_id_idx ON claims (user_id);

CREATE TABLE IF NOT EXISTS items (
  id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  claim_id INTEGER NOT NULL REFERENCES claims (id),
  category_id INTEGER NOT NULL REFERENCES categories (id),
  cost NUMERIC NOT NULL CHECK (cost > 0),
  reimburstment NUMERIC NOT NULL CHECK (reimburstment > 0)
);

CREATE INDEX IF NOT EXISTS items_claim_id_idx ON items (claim_id);
CREATE INDEX IF NOT EXISTS items_category_id_idx ON items (category_id);


