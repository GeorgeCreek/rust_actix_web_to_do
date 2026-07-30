export interface User {
    id: number;
    email: string;
    unique_id: string;
}

export interface NewUser {
    email: string;
    password: string;
}

export interface UpdateUser {
    id: number;
    email: string;
    password?: string;
}
