export type PostType = 'offer' | 'request';

// Comprehensive category enum with 100+ categories
export const CATEGORIES = [
    // Technology & Programming
    'Web Development', 'Mobile App Development', 'Software Engineering', 'Data Science', 'Artificial Intelligence',
    'Machine Learning', 'Cybersecurity', 'Cloud Computing', 'DevOps', 'Database Management',
    'UI/UX Design', 'Game Development', 'Blockchain', 'IoT Development', 'System Administration',
    
    // Creative & Design
    'Graphic Design', 'Video Editing', 'Photography', 'Digital Art', 'Animation',
    'Content Writing', 'Copywriting', 'Social Media Management', 'Brand Strategy', 'Logo Design',
    'Web Design', 'Print Design', 'Illustration', '3D Modeling', 'Audio Production',
    
    // Business & Finance
    'Business Strategy', 'Marketing', 'Sales', 'Accounting', 'Financial Planning',
    'Project Management', 'Product Management', 'Operations Management', 'Supply Chain', 'Consulting',
    'Market Research', 'Investment Advice', 'Tax Preparation', 'Bookkeeping', 'Business Analytics',
    
    // Education & Training
    'Tutoring', 'Language Teaching', 'Online Course Creation', 'Curriculum Development', 'Academic Writing',
    'Exam Preparation', 'Skills Training', 'Professional Development', 'Corporate Training', 'Educational Technology',
    'Research Assistance', 'Thesis Writing', 'Presentation Skills', 'Study Techniques', 'Career Counseling',
    
    // Health & Wellness
    'Fitness Training', 'Nutrition Counseling', 'Mental Health Support', 'Yoga Instruction', 'Meditation Guidance',
    'Physical Therapy', 'Life Coaching', 'Wellness Coaching', 'Stress Management', 'Sleep Optimization',
    'Diet Planning', 'Exercise Programs', 'Mindfulness Training', 'Addiction Recovery', 'Health Education',
    
    // Home & Lifestyle
    'Home Improvement', 'Interior Design', 'Gardening', 'Cooking', 'Cleaning Services',
    'Handyman Services', 'Electrical Work', 'Plumbing', 'Carpentry', 'Painting',
    'Landscaping', 'Pet Care', 'Childcare', 'Elder Care', 'Event Planning',
    
    // Transportation & Logistics
    'Driving Lessons', 'Vehicle Maintenance', 'Moving Services', 'Delivery Services', 'Travel Planning',
    'Logistics Coordination', 'Transportation Services', 'Car Repair', 'Bike Maintenance', 'Navigation Help',
    
    // Arts & Crafts
    'Music Lessons', 'Art Classes', 'Crafting', 'Jewelry Making', 'Pottery',
    'Woodworking', 'Sewing', 'Knitting', 'Embroidery', 'Painting Classes',
    'Dance Lessons', 'Theater Arts', 'Creative Writing', 'Poetry', 'Storytelling',
    
    // Language & Communication
    'Translation Services', 'Interpretation', 'Public Speaking', 'Communication Skills', 'Presentation Design',
    'Technical Writing', 'Proofreading', 'Editing Services', 'Voice Training', 'Interview Preparation',
    
    // Legal & Administrative
    'Legal Advice', 'Document Preparation', 'Notary Services', 'Immigration Help', 'Contract Review',
    'Administrative Support', 'Data Entry', 'Virtual Assistant', 'Research Services', 'Government Forms',
    
    // Miscellaneous
    'Event Photography', 'Wedding Planning', 'Relationship Counseling', 'Spiritual Guidance', 'Community Organizing',
    'Volunteer Coordination', 'Fundraising', 'Grant Writing', 'Non-profit Management', 'Other'
] as const;

export type Category = typeof CATEGORIES[number];

export interface Post {
    id: number;
    description: string;
    categories: Category[];
    user_id: number;
    post_type: PostType;
    pin_code?: string;
    user_name?: string;
    profile_picture?: string;
}

export interface NewPost {
    description: string;
    categories: Category[];
    post_type: PostType;
    pin_code?: string;
}

export interface NewUser {
    email: string;
    password: string;
    name?: string;
    pin_code?: string;
    profile_picture?: string;
}

export interface UserProfile {
    id: number;
    email: string;
    name?: string;
    pin_code?: string;
    profile_picture?: string;
}

export interface AuthResponse {
    success: boolean;
    message: string;
    user_id?: number;
}

export interface LoginRequest {
    email: string;
    password: string;
}