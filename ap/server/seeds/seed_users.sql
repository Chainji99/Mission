-- Seed 10 test users
-- Password: Test@123 (hashed with argon2)

INSERT INTO brawlers (username, password, display_name, created_at, updated_at) VALUES
('test_user_1', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 1', NOW(), NOW()),
('test_user_2', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 2', NOW(), NOW()),
('test_user_3', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 3', NOW(), NOW()),
('test_user_4', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 4', NOW(), NOW()),
('test_user_5', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 5', NOW(), NOW()),
('test_user_6', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 6', NOW(), NOW()),
('test_user_7', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 7', NOW(), NOW()),
('test_user_8', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 8', NOW(), NOW()),
('test_user_9', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 9', NOW(), NOW()),
('test_user_10', '$argon2id$v=19$m=19456,t=2,p=1$2sOGKuCf3pDvvnOl3UlNbg$XK1IHIgcWqJp8H8hpKVqY1XFT4Z7V2Q9M3N5P8R1Z0M', 'Test User 10', NOW(), NOW())
ON CONFLICT (username) DO NOTHING;
