import { User } from "../interfaces/users";
import { deleteCall } from "./utils";
import { Url } from "./url";


export async function deleteUserCall(id: number) {
    return deleteCall<User[]>(
        new Url().usersDeleteUrl(id),
        200
    );
}
