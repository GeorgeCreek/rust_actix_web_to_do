// File: frontend/src/index.tsx
import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";
import { CreateToDoItem } from './components/CreateItemForm';
import { UsersView } from './components/UsersView';
import "./App.css";
import init, { rust_generate_button_text } from '../rust-interface/pkg/rust_interface.js';
import { LoginForm } from './components/LoginForm';
import { AuthTokenPair, getAccessToken, storeTokens } from './api/tokens';
import { logout as apiLogout } from './api/login';


type AppView = "todos" | "users";


const App = () => {
    const [data, setData] = useState<ToDoItems | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [wasmReady, setWasmReady] = useState<boolean>(false);
    const [
        RustGenerateButtonText, 
        setRustGenerateButtonText
    ] = useState<((input: string) => string) | null>(null);
    const [loggedin, setLoggedin] = useState<boolean>(
        getAccessToken() !== null
    );
    const [view, setView] = useState<AppView>("todos");

    function setToken(tokens: AuthTokenPair) {
        storeTokens(tokens);
        setLoggedin(true);
    }
    async function removeToken() {
        await apiLogout();
        setData(null);
        setError(null);
        setLoggedin(false);
        setView("todos");
    }
    function reRenderItems(response: { error?: string; data?: ToDoItems }) {
        if (response.error) {
            if (response.error.includes("Session expired")) {
                setLoggedin(false);
                setData(null);
            }
            alert(JSON.stringify(response));
            return;
        }
        else if (response.data) {
            setData(response.data);
            setError(null);
        }
        else {
            setError("Unknown error");
        }
    }

    React.useEffect(() => {
        init().then(() => {
            setRustGenerateButtonText(() => rust_generate_button_text);
            setWasmReady(true);
        }).catch(e => console.error(
            "Error initializing WASM: ", e
        ));
    }, []);    

    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll();
            if (response.error) {
                if (String(response.error).includes("Session expired")) {
                    setLoggedin(false);
                    setData(null);
                    return;
                }
                setError(String(response.error));
            } else if (response.data && typeof response.data !== "string") {
                setData(response.data);
            }
        };
        if (wasmReady && loggedin && view === "todos") {
            fetchData();
        }
    }, [wasmReady, loggedin, view]);
    
    if (!loggedin) {
        return (
            <div>
                <LoginForm setToken={setToken} />
            </div>
        );
    }

    if (view === "users") {
        return (
            <UsersView
                onLogout={() => { void removeToken(); }}
                onNavigateTodos={() => setView("todos")}
            />
        );
    }

    if (error) {
        return (
            <div>
                <div style={{ color: 'red' }}>Error: {error}</div>
                <button type="button" onClick={() => { void removeToken(); }}>
                    Logout
                </button>
            </div>
        );
    }
    else if (!data || !RustGenerateButtonText) {
        return <div>Loading...</div>
    }
    return (
        <div className="App">
        <div className="mainContainer">
            <div className="header">
                <p>complete tasks: {data.done.length}</p>
                <p>pending tasks: {data.pending.length}</p>
                <button type="button" onClick={() => setView("users")}>
                    Users
                </button>
                <button type="button" onClick={() => { void removeToken(); }}>
                    Logout
                </button>
            </div>
            <h1>Pending Items</h1>
            <div>
                {data.pending.map((item) => (
                    <ToDoItem key={item.id}
                                title={item.title}
                                status={item.status}
                                id={item.id}
                                buttonMessage={
                                    RustGenerateButtonText(item.status)
                                }            
                                passBackResponse={reRenderItems}/>
                ))}
            </div>
            <h1>Done Items</h1>
            <div>
                {data.done.map((item) => (
                    <ToDoItem key={item.id}
                                title={item.title}
                                status={item.status}
                                id={item.id}
                                buttonMessage={
                                    RustGenerateButtonText(item.status)
                                }   
                                passBackResponse={reRenderItems}/>
                ))}
            </div>
            <CreateToDoItem passBackResponse={reRenderItems} />
        </div>
        </div>
    );
};

const rootElement = document.getElementById('root');
if (!rootElement) {
    throw new Error('Root element not found');
}
const root = ReactDOM.createRoot(rootElement);
root.render(<App />);
