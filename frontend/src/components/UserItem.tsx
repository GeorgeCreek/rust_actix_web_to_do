import React, { useState } from 'react';
import { updateUserCall } from "../api/usersUpdate";
import { deleteUserCall } from "../api/usersDelete";
import { User } from "../interfaces/users";


interface UserItemProps {
    user: User;
    passBackResponse: (response: {
        error?: string;
        data?: User[];
    }) => void;
}


export const UserItem: React.FC<UserItemProps> = (
    { user, passBackResponse }
) => {
    const [editing, setEditing] = useState<boolean>(false);
    const [email, setEmail] = useState<string>(user.email);
    const [password, setPassword] = useState<string>("");

    const deleteUser = async () => {
        const response = await deleteUserCall(user.id);
        passBackResponse(response);
    };

    const saveUser = async () => {
        if (!email) {
            alert("Email is required");
            return;
        }
        const response = await updateUserCall(
            user.id,
            email,
            password || undefined
        );
        if (!response.error) {
            setEditing(false);
            setPassword("");
        }
        passBackResponse(response);
    };

    if (editing) {
        return (
            <div className="itemContainer userEdit" id={String(user.id)}>
                <input
                    type="text"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                />
                <input
                    type="password"
                    placeholder="new password (optional)"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />
                <button
                    className="actionButton"
                    type="button"
                    onClick={saveUser}
                >
                    save
                </button>
                <button
                    className="actionButton"
                    type="button"
                    onClick={() => {
                        setEditing(false);
                        setEmail(user.email);
                        setPassword("");
                    }}
                >
                    cancel
                </button>
            </div>
        );
    }

    return (
        <div className="itemContainer" id={String(user.id)}>
            <p>[{user.id}] {user.email}</p>
            <div className="itemActions">
                <button
                    className="actionButton"
                    type="button"
                    onClick={() => setEditing(true)}
                >
                    edit
                </button>
                <button
                    className="actionButton"
                    type="button"
                    onClick={deleteUser}
                >
                    delete
                </button>
            </div>
        </div>
    );
};
