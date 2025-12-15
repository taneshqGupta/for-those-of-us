-- Add profile_picture column to users table for storing Cloudinary image URLs
ALTER TABLE users ADD COLUMN IF NOT EXISTS profile_picture TEXT;
