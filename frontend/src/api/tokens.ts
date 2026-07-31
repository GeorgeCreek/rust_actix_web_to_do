export interface AuthTokenPair {
    access_token: string;
    refresh_token: string;
}

export function storeTokens(tokens: AuthTokenPair) {
    localStorage.setItem("token", tokens.access_token);
    localStorage.setItem("refresh_token", tokens.refresh_token);
}

export function clearTokens() {
    localStorage.removeItem("token");
    localStorage.removeItem("refresh_token");
}

export function getAccessToken(): string | null {
    const token = localStorage.getItem("token");
    if (!token || token === "undefined" || token === "null") {
        return null;
    }
    return token;
}

export function getRefreshToken(): string | null {
    const token = localStorage.getItem("refresh_token");
    if (!token || token === "undefined" || token === "null") {
        return null;
    }
    return token;
}
