CREATE TABLE IF NOT EXISTS items (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(255) NOT NULL,
  description VARCHAR(255),
  percent_remburst DECIMAL(5,2) NOT NULL,
  max_rembursement DECIMAL(10,2) NOT NULL
);

CREATE INDEX idx_items_id ON items (id);

CREATE TABLE IF NOT EXISTS history (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  item_id INT NOT NULL,
  start_price DECIMAL(10,2) NOT NULL,
  rembursed DECIMAL(10,2) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

  CONSTRAINT fk_history_items FOREIGN KEY(item_id)
    REFERENCES items(id)
);

CREATE INDEX idx_history_id ON history (id);
