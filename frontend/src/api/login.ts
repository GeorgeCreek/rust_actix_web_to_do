import axios from 'axios';
import { Url } from "./url";
import { AuthTokenPair, storeTokens, getRefreshToken, clearTokens } from "./tokens";


function loginErrorMessage(error: unknown): string {
    if (!axios.isAxiosError(error)) {
        return 'Login failed. Please try again.';
    }
    const data = error.response?.data;
    if (typeof data === 'string' && data.trim()) {
        return data;
    }
    if (data && typeof data === 'object' && 'message' in data) {
        const message = (data as { message?: unknown }).message;
        if (typeof message === 'string' && message.trim()) {
            return message;
        }
    }
    if (error.response?.status === 401) {
        return 'Invalid email or password.';
    }
    if (error.response?.status === 404) {
        return 'User not found.';
    }
    if (error.code === 'ERR_NETWORK') {
        return 'Cannot reach the server. Is ingress running on :8001?';
    }
    return 'Login failed. Please try again.';
}


export const login = async (
        email: string, 
        password: string
    ): Promise<AuthTokenPair> => {
    const trimmedEmail = email.trim();
    if (!trimmedEmail || !password) {
        const message = 'Email and password are required.';
        alert(message);
        throw new Error(message);
    }
    const authToken = btoa(`${trimmedEmail}:${password}`);
    try {
        const response = await axios({
            method: 'get',
            url: new Url().login,
            headers: {
                'Authorization': `Basic ${authToken}`,
                'Content-Type': 'application/json'
            },
        });
        const tokens = response.data as AuthTokenPair;
        if (!tokens?.access_token || !tokens?.refresh_token) {
            throw new Error('Login response did not include token pair');
        }
        storeTokens(tokens);
        return tokens;
    }
    catch (error) {
        const message = loginErrorMessage(error);
        if (axios.isAxiosError(error)) {
            console.error('Login error:', error.response?.data ?? error.message);
        } else {
            console.error('Unexpected error:', error);
        }
        alert(message);
        throw error;
    }
};


let refreshPromise: Promise<AuthTokenPair> | null = null;


export async function refreshAccessToken(): Promise<AuthTokenPair> {
    if (refreshPromise) {
        return refreshPromise;
    }

    refreshPromise = (async () => {
        const refreshToken = getRefreshToken();
        if (!refreshToken) {
            clearTokens();
            throw new Error("No refresh token");
        }
        try {
            const response = await axios.post<AuthTokenPair>(
                new Url().refresh,
                { refresh_token: refreshToken },
                { headers: { 'Content-Type': 'application/json' } }
            );
            storeTokens(response.data);
            return response.data;
        } catch (error) {
            clearTokens();
            throw error;
        } finally {
            refreshPromise = null;
        }
    })();

    return refreshPromise;
}


export async function logout(): Promise<void> {
    const refreshToken = getRefreshToken();
    if (!refreshToken) {
        clearTokens();
        alert(
            'Logout cleared local session, but no refresh token was stored — ' +
            'the database row was not revoked. Hard-refresh and login again, then use Logout.'
        );
        return;
    }

    try {
        const response = await axios.post(
            new Url().logout,
            { refresh_token: refreshToken },
            {
                headers: { 'Content-Type': 'application/json' },
                validateStatus: () => true,
            }
        );
        if (response.status !== 204 && response.status !== 200) {
            console.error('Logout failed:', response.status, response.data);
            alert(
                `Logout revoke failed (HTTP ${response.status}). ` +
                'Local session will still be cleared.'
            );
        }
    } catch (error) {
        console.error('Logout error:', error);
        alert('Logout request failed. Local session will still be cleared.');
    } finally {
        clearTokens();
    }
}
