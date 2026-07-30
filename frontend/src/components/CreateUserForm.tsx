import React, { useState } from 'react';
import { createUserCall } from "../api/usersCreate";
import { User } from "../interfaces/users";


interface CreateUserFormProps {
    passBackResponse: (response: {
        error?: string;
        data?: User | User[];
    }) => void;
}


export const CreateUserForm: React.FC<CreateUserFormProps> = (
    { passBackResponse }
) => {
    const [email, setEmail] = useState<string>("");
    const [password, setPassword] = useState<string>("");

    const createUser = async () => {
        if (!email || !password) {
            alert("Email and password are required");
            return;
        }
        const response = await createUserCall(email, password);
        if (response.error) {
            passBackResponse(response);
            return;
        }
        setEmail("");
        setPassword("");
        passBackResponse(response);
    };

    return (
        <div className="inputContainer userForm">
            <input
                type="text"
                placeholder="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
            />
            <input
                type="password"
                placeholder="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
            />
            <button
                className="actionButton"
                type="button"
                onClick={createUser}
            >
                Create
            </button>
        </div>
    );
};
