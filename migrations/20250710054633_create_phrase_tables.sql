-- Create phrase_lists table
CREATE TABLE phrase_lists (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    target_lang VARCHAR(50) NOT NULL,
    source_lang VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, name)
);

-- Create phrases table
CREATE TABLE phrases (
    id SERIAL PRIMARY KEY,
    list_id INTEGER NOT NULL REFERENCES phrase_lists(id) ON DELETE CASCADE,
    target_lang_text TEXT NOT NULL,
    source_lang_text TEXT NOT NULL
);

-- Insert test data
-- Note: The password for all users is 'password'
-- The bcrypt hash for 'password' is '$2a$12$Z2AE8fyYcmnPcE/iV2wjX.bV56yp2mGZyVxEpwaD.kWG1u.DVwlTW'

-- User 'gwrhyr' (assuming this user already exists from previous migrations)
-- If not, you can uncomment the following line to create it:
-- INSERT INTO users (username, password_hash) VALUES ('gwrhyr', '$2a$12$Z2AE8fyYcmnPcE/iV2wjX.bV56yp2mGZyVxEpwaD.kWG1u.DVwlTW');

-- Create another user for testing
INSERT INTO users (username, password_hash) VALUES ('testuser', '$2a$12$Z2AE8fyYcmnPcE/iV2wjX.bV56yp2mGZyVxEpwaD.kWG1u.DVwlTW');

-- Get user IDs
DO $$
DECLARE
    gwrhyr_id INTEGER;
    testuser_id INTEGER;
BEGIN
    -- Get user IDs
    SELECT id INTO gwrhyr_id FROM users WHERE username = 'gwrhyr';
    SELECT id INTO testuser_id FROM users WHERE username = 'testuser';

    -- Insert phrase lists for 'gwrhyr'
    INSERT INTO phrase_lists (user_id, name, target_lang, source_lang) VALUES
        (gwrhyr_id, 'Japanese Basics', 'ja', 'en'),
        (gwrhyr_id, 'Spanish Greetings', 'es', 'en');

    -- Insert phrase lists for 'testuser'
    INSERT INTO phrase_lists (user_id, name, target_lang, source_lang) VALUES
        (testuser_id, 'French for Beginners', 'fr', 'en');

    -- Insert phrases for 'Japanese Basics'
    INSERT INTO phrases (list_id, target_lang_text, source_lang_text) VALUES
        ((SELECT id FROM phrase_lists WHERE name = 'Japanese Basics' AND user_id = gwrhyr_id), 'こんにちは。', 'Hello.'),
        ((SELECT id FROM phrase_lists WHERE name = 'Japanese Basics' AND user_id = gwrhyr_id), 'おはよう。', 'Good morning.'),
        ((SELECT id FROM phrase_lists WHERE name = 'Japanese Basics' AND user_id = gwrhyr_id), '日本へようこそ。', 'Welcome to Japan.');

    -- Insert phrases for 'Spanish Greetings'
    INSERT INTO phrases (list_id, target_lang_text, source_lang_text) VALUES
        ((SELECT id FROM phrase_lists WHERE name = 'Spanish Greetings' AND user_id = gwrhyr_id), 'Hola.', 'Hello.'),
        ((SELECT id FROM phrase_lists WHERE name = 'Spanish Greetings' AND user_id = gwrhyr_id), 'Buenos días.', 'Good morning.');

    -- Insert phrases for 'French for Beginners'
    INSERT INTO phrases (list_id, target_lang_text, source_lang_text) VALUES
        ((SELECT id FROM phrase_lists WHERE name = 'French for Beginners' AND user_id = testuser_id), 'Bonjour.', 'Hello.'),
        ((SELECT id FROM phrase_lists WHERE name = 'French for Beginners' AND user_id = testuser_id), 'Bienvenue en France.', 'Welcome to France.');
END $$;