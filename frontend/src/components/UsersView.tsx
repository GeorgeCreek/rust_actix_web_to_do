import React, { useEffect, useState } from 'react';
import { getAllUsers } from '../api/usersGet';
import { User } from '../interfaces/users';
import { UserItem } from './UserItem';
import { CreateUserForm } from './CreateUserForm';


interface UsersViewProps {
    onLogout: () => void;
    onNavigateTodos: () => void;
}


export const UsersView: React.FC<UsersViewProps> = (
    { onLogout, onNavigateTodos }
) => {
    const [users, setUsers] = useState<User[] | null>(null);
    const [error, setError] = useState<string | null>(null);

    const loadUsers = async () => {
        const response = await getAllUsers();
        if (response.error) {
            setError(String(response.error));
            return;
        }
        if (response.data && Array.isArray(response.data)) {
            setUsers(response.data);
            setError(null);
        }
    };

    useEffect(() => {
        loadUsers();
    }, []);

    const reRenderUsers = async (response: {
        error?: string;
        data?: User | User[];
    }) => {
        if (response.error) {
            alert(JSON.stringify(response));
            return;
        }
        if (Array.isArray(response.data)) {
            setUsers(response.data);
            setError(null);
            return;
        }
        // create returns a single user — refresh the list
        await loadUsers();
    };

    if (error) {
        return (
            <div className="App">
                <div className="mainContainer">
                    <div style={{ color: 'red' }}>Error: {error}</div>
                    <button type="button" onClick={onLogout}>
                        Logout
                    </button>
                </div>
            </div>
        );
    }

    if (!users) {
        return <div>Loading...</div>;
    }

    return (
        <div className="App">
            <div className="mainContainer">
                <div className="header">
                    <p>users: {users.length}</p>
                    <button type="button" onClick={onNavigateTodos}>
                        Todos
                    </button>
                    <button type="button" onClick={onLogout}>
                        Logout
                    </button>
                </div>
                <h1>Users</h1>
                <div>
                    {users.map((user) => (
                        <UserItem
                            key={user.id}
                            user={user}
                            passBackResponse={reRenderUsers}
                        />
                    ))}
                </div>
                <CreateUserForm passBackResponse={reRenderUsers} />
            </div>
        </div>
    );
};
