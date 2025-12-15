-- Step 1: Begin transitioning from todos to skill-sharing posts
-- Rename todos table to posts and add post_type to distinguish between offers and requests

-- Rename the table
ALTER TABLE todos RENAME TO posts;

-- Rename the description column to be more generic
ALTER TABLE posts RENAME COLUMN descript TO description;

-- Rename 'done' to 'completed' for clarity
ALTER TABLE posts RENAME COLUMN done TO completed;

-- Add post_type to distinguish between skill offers and help requests
-- 'offer' = I can help with this skill
-- 'request' = I need help with this
ALTER TABLE posts ADD COLUMN post_type VARCHAR(20) NOT NULL DEFAULT 'request';

-- Update the index name
DROP INDEX IF EXISTS idx_todos_user_id;
CREATE INDEX idx_posts_user_id ON posts(user_id);

-- Add index for post_type for efficient filtering
CREATE INDEX idx_posts_type ON posts(post_type);
