-- Up Migration
CREATE TABLE cards (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  language VARCHAR(50) NOT NULL,
  rarity VARCHAR(20) NOT NULL,
  attack INT NOT NULL DEFAULT 10,
  defense INT NOT NULL DEFAULT 10,
  image_url VARCHAR(255)
);

CREATE TABLE user_cards (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES brawlers(id),
  card_id INT NOT NULL REFERENCES cards(id),
  level INT NOT NULL DEFAULT 1,
  experience INT NOT NULL DEFAULT 0,
  obtained_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE battles (
  id SERIAL PRIMARY KEY,
  attacker_id INT NOT NULL REFERENCES brawlers(id),
  defender_id INT NOT NULL REFERENCES brawlers(id),
  winner_id INT REFERENCES brawlers(id),
  log TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Seed Data
INSERT INTO cards (name, language, rarity, attack, defense) VALUES
('Rusty Crab', 'Rust', 'Legendary', 95, 90),
('Py Snake', 'Python', 'Common', 45, 35),
('JS Ninja', 'JavaScript', 'Rare', 65, 45),
('Go Gopher', 'Go', 'Epic', 80, 75),
('Java Knight', 'Java', 'Rare', 70, 85),
('C++ Titan', 'C++', 'Epic', 85, 60),
('TypeScript Archer', 'TypeScript', 'Rare', 60, 50),
('PHP Elephant', 'PHP', 'Common', 40, 40);

-- Down Migration
DROP TABLE battles;
DROP TABLE user_cards;
DROP TABLE cards;
