// File: frontend/src/api/url.ts
export class Url {

    baseUrl: string;
    create: string;
    getAll: string;
    update: string;
    delete: string;
    login: string;
    usersCreate: string;
    usersGetAll: string;
    usersUpdate: string;

    constructor() {
        this.baseUrl = Url.getBaseUrl();
        this.create = `${this.baseUrl}api/v1/create`;
        this.getAll = `${this.baseUrl}api/v1/get/all`;
        this.update = `${this.baseUrl}api/v1/update`;
        this.delete = `${this.baseUrl}api/v1/delete`;
        this.login = `${this.baseUrl}api/v1/auth/login`;
        this.usersCreate = `${this.baseUrl}api/v1/users/create`;
        this.usersGetAll = `${this.baseUrl}api/v1/users/get/all`;
        this.usersUpdate = `${this.baseUrl}api/v1/users/update`;
    }
    
    static getBaseUrl(): string {
        const { origin, href } = window.location;
        // Dev server on :3000 talks to the ingress API on :8001
        if (href.includes("http://localhost:3000")) {
            return "http://localhost:8001/";
        }
        // Use origin (not full href) so path segments don't break API URLs
        return `${origin}/`;
    }

    updateUrl(id: number): string {
        return `${this.baseUrl}api/v1/update`;
    }
    
    deleteUrl(name: string): string {
        return `${this.baseUrl}api/v1/delete/${encodeURIComponent(name)}`;
    }

    usersDeleteUrl(id: number): string {
        return `${this.baseUrl}api/v1/users/delete/${id}`;
    }
}
