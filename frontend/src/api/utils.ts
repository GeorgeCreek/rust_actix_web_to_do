// File: frontend/src/api/utils.ts
import axios, {AxiosResponse} from "axios";
import { getAccessToken } from "./tokens";
import { refreshAccessToken } from "./login";


type ApiResult<X> = {
    status: number;
    data?: X;
    error?: string;
};


async function handleRequest<X>(
    makeRequest: () => Promise<AxiosResponse<X>>,
    expectedResponse: number,
    retrying = false
): Promise<ApiResult<X>> {
    let response: AxiosResponse<X>;
    try {
        response = await makeRequest();
    } catch (error) {
        return {
            status: 500,
            error: "Network Error",
            data: JSON.stringify(error) as unknown as X
        };
    }

    if (response.status === 401 && !retrying) {
        try {
            await refreshAccessToken();
            return handleRequest(makeRequest, expectedResponse, true);
        } catch {
            return {
                status: 401,
                error: "Session expired. Please log in again.",
            };
        }
    }

    if (response.status === expectedResponse) {
        return {
            status: response.status,
            data: response.data as X
        };
    }
    return {
        status: response.status,
        error: `expected status ${expectedResponse} got ${response.status}`,
        data: response.data as X
    };
}


function authHeaders(): Record<string, string> {
    const headers: Record<string, string> = {
        'Content-Type': 'application/json',
    };
    const token = getAccessToken();
    if (token) {
        headers.token = token;
    }
    return headers;
}


export async function postCall<T, X>(
    url: string, body: T, expectedResponse: number) {
    return handleRequest(
        () => axios.post<X | string>(url, body, {
            headers: authHeaders(),
            validateStatus: () => true
        }) as Promise<AxiosResponse<X>>,
        expectedResponse
    );
}


export async function getCall<X>(
    url: string, 
    expectedResponse: number) {
    return handleRequest(
        () => axios.get<X | string>(url, {
            headers: authHeaders(),
            validateStatus: () => true
        }) as Promise<AxiosResponse<X>>,
        expectedResponse
    );
}


export async function deleteCall<X>(
    url: string, 
    expectedResponse: number) {
    return handleRequest(
        () => axios.delete<X | string>(url, {
            headers: authHeaders(),
            validateStatus: () => true
        }) as Promise<AxiosResponse<X>>,
        expectedResponse
    );
}


export async function putCall<T, X>(
    url: string, body: T, 
    expectedResponse: number) {
    return handleRequest(
        () => axios.put<X | string>(url, body, {
            headers: authHeaders(),
            validateStatus: () => true
        }) as Promise<AxiosResponse<X>>,
        expectedResponse
    );
}

export async function patchCall<T, X>(
    url: string, body: T, 
    expectedResponse: number) {
    return handleRequest(
        () => axios.patch<X | string>(url, body, {
            headers: authHeaders(),
            validateStatus: () => true
        }) as Promise<AxiosResponse<X>>,
        expectedResponse
    );
}
