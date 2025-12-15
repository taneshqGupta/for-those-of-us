import { PUBLIC_BACKEND_URL } from '$env/static/public';
import type { Post, NewPost, AuthResponse, LoginRequest, UserProfile, Category } from '$lib/types';

export async function createPost(description: string, categories: Category[], post_type: 'offer' | 'request', pin_code?: string): Promise<Post> {
    const formData = new URLSearchParams();
    formData.append('description', description);
    
    // Send categories as JSON string since FormData doesn't handle arrays well
    formData.append('categories', JSON.stringify(categories));
    formData.append('post_type', post_type);
    if (pin_code) formData.append('pin_code', pin_code);

    const response = await fetch(`${PUBLIC_BACKEND_URL}posts/create`, {
        method: "POST",
        credentials: "include",
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: formData.toString()
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to create post: ${response.status} ${response.statusText} - ${errorText}`);
    }
    return response.json();
}

export async function getMyPosts(): Promise<Post[]> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}posts`, {
        method: "GET",
        credentials: "include"
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to fetch posts: ${response.statusText}`);
    }
    return response.json();
}

export async function getUserPosts(id: number): Promise<Post[]> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}foreignposts/${id}`, {
        method: "GET",
        credentials: "include"
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to fetch posts: ${response.statusText}`);
    }
    return response.json();
}

export async function getCommunityPosts(): Promise<Post[]> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}community`, {
        method: "GET",
        credentials: "include"
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to fetch community posts: ${response.statusText}`);
    }
    return response.json();
}

export async function getCommunityOffers(): Promise<Post[]> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}community/offers`, {
        method: "GET",
        credentials: "include"
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to fetch community offers: ${response.statusText}`);
    }
    return response.json();
}

export async function getCommunityRequests(): Promise<Post[]> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}community/requests`, {
        method: "GET",
        credentials: "include"
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Failed to fetch community requests: ${response.statusText}`);
    }
    return response.json();
}

export async function updatePost(postToUpdate: Post): Promise<Post> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}posts/update`, {
        method: 'POST',
        credentials: 'include',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(postToUpdate)
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to update post: ${response.status} ${response.statusText} - ${errorText}`);
    }
    return response.json();
}

export async function deletePost(id: number): Promise<void> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}posts/delete/${id}`, {
        method: "DELETE",
        credentials: "include"
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to delete post: ${response.status} ${response.statusText} - ${errorText}`);
    }
}

export async function login(email: string, password: string): Promise<AuthResponse> {
    const formData = new URLSearchParams();
    formData.append('email', email);
    formData.append('password', password);

    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/login`, {
        method: "POST",
        credentials: "include",
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: formData.toString()
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Login failed: ${response.statusText}`);
    }
    return response.json();
}

export async function register(email: string, password: string, name?: string, pinCode?: string, profilePicture?: string): Promise<AuthResponse> {
    const formData = new URLSearchParams();
    formData.append('email', email);
    formData.append('password', password);
    if (name) formData.append('name', name);
    if (pinCode) formData.append('pin_code', pinCode);
    if (profilePicture) formData.append('profile_picture', profilePicture);

    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/register`, {
        method: "POST",
        credentials: "include",
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: formData.toString()
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Registration failed: ${response.statusText}`);
    }
    return response.json();
}

export async function logout(): Promise<AuthResponse> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/logout`, {
        method: "POST",
        credentials: "include"
    });

    if (!response.ok) {
        const errorData = await response.json().catch(() => ({}));
        throw new Error(errorData.message || `Logout failed: ${response.statusText}`);
    }
    return response.json();
}

export async function checkAuth(): Promise<AuthResponse> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/check`, {
        method: "GET",
        credentials: "include",
    });

    if (!response.ok) {
        throw new Error(`Auth check failed: ${response.statusText}`);
    }
    return response.json();
}

export async function getMyProfile(): Promise<UserProfile> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/myprofile`, {
        method: "GET",
        credentials: "include",
    });

    if (!response.ok) {
        throw new Error(`Failed to get profile: ${response.statusText}`);
    }
    return response.json();
}

export async function getMyUserId(): Promise<number> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/my_userid`, {
        method: "GET",
        credentials: "include",
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to get user ID: ${response.statusText} - ${errorText}`);
    }
    return response.json();
}

export async function getUserProfile(userId: number): Promise<UserProfile> {
    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/userprofile/${userId}`, {
        method: "GET",
        credentials: "include",
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Failed to get user profile: ${response.statusText} - ${errorText}`);
    }
    return response.json();
}

export async function updateProfilePicture(profilePicture: string): Promise<any> {
    const formData = new URLSearchParams();
    formData.append('profile_picture', profilePicture);

    const response = await fetch(`${PUBLIC_BACKEND_URL}auth/myprofile/picture`, {
        method: "POST",
        credentials: "include",
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: formData.toString()
    });

    if (!response.ok) {
        throw new Error(`Failed to update profile picture: ${response.statusText}`);
    }
    return response.json();
}