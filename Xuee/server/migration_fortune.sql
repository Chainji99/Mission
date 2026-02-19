-- Create fortune_sticks table
CREATE TABLE fortune_sticks (
    id SERIAL PRIMARY KEY,
    number INTEGER NOT NULL,
    poem_text TEXT NOT NULL,
    interpretation TEXT NOT NULL,
    lucky_direction VARCHAR(50)
);

-- Create daily_fortunes table
CREATE TABLE daily_fortunes (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES brawlers(id),
    stick_id INTEGER NOT NULL REFERENCES fortune_sticks(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    date DATE NOT NULL DEFAULT CURRENT_DATE
);

-- Seed Fortune Sticks (Sample 10 sticks for demo)
INSERT INTO fortune_sticks (number, poem_text, interpretation, lucky_direction) VALUES
(1, 'Great fortune comes to those who wait patiently. The sun will rise again.', 'Excellent luck in career and love.', 'East'),
(2, 'Clouds obscure the moon, but wind will clear them soon.', 'Difficulties are temporary. Keep going.', 'North'),
(3, 'A dragon sleeps in the deep, waiting for the spring thunder.', 'Potential is hidden. Prepare yourself.', 'South'),
(4, 'The tiger walks cautiously in the bamboo forest.', 'Be careful with decisions. Avoid risks.', 'West'),
(5, 'Peach blossoms bloom in the warm breeze.', 'Romance is in the air. Good social luck.', 'South-East'),
(6, 'The boat sails against the current, requiring strength.', 'Hard work is needed, but success is possible.', 'North-West'),
(7, 'Golden carp leaps over the dragon gate.', 'Great success in exams or promotion.', 'North-East'),
(8, 'Autumn leaves fall, returning to the roots.', 'Time to reflect and rest. Reconnect with family.', 'South-West'),
(9, 'The phoenix dances in the high sky.', 'Fame and recognition are coming.', 'South'),
(10, 'Heavy rain nourishes the dry earth.', 'Help will come when least expected.', 'North');
