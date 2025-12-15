-- Step 5: Add pin code (postal code) support for location-based skill sharing
-- This enables users to specify their location and find nearby skills/requests

-- Add pin_code to posts table
ALTER TABLE posts ADD COLUMN pin_code VARCHAR(10);

-- Add index for efficient location-based queries
CREATE INDEX idx_posts_pin_code ON posts(pin_code);

-- Also add pin_code to users table for their default location
ALTER TABLE users ADD COLUMN pin_code VARCHAR(10);
