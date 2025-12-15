-- Make user_id NOT NULL for todos table
-- First, we need to handle existing todos without user_id
-- For now, we'll delete any todos without user_id (since this is development)
DELETE FROM todos WHERE user_id IS NULL;

-- Now make the column NOT NULL
ALTER TABLE todos ALTER COLUMN user_id SET NOT NULL;
