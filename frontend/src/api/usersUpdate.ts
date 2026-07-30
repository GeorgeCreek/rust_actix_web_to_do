import { UpdateUser, User } from "../interfaces/users";
import { putCall } from "./utils";
import { Url } from "./url";


export async function updateUserCall(
    id: number,
    email: string,
    password?: string
) {
    const user: UpdateUser = { id, email };
    if (password) {
        user.password = password;
    }
    return putCall<UpdateUser, User[]>(
        new Url().usersUpdate,
        user,
        200
    );
}
