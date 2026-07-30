import { User } from "../interfaces/users";
import { getCall } from "./utils";
import { Url } from "./url";


export async function getAllUsers() {
    return getCall<User[]>(
        new Url().usersGetAll,
        200
    );
}
