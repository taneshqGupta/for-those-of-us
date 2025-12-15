-- Step 6: Add names to users for community posts
-- This allows people to see who's posting skills and requests

-- Add name field to users table
ALTER TABLE users ADD COLUMN name VARCHAR(255);

-- For existing users without names, we could set a default based on email
-- but for now we'll leave it nullable and let users update later
