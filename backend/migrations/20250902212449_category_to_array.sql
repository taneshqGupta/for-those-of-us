ALTER TABLE posts ADD COLUMN categories TEXT[] NOT NULL DEFAULT '{}';

UPDATE posts SET categories = ARRAY[category];

ALTER TABLE posts DROP COLUMN category;

CREATE INDEX idx_posts_categories_gin ON posts USING GIN(categories);
