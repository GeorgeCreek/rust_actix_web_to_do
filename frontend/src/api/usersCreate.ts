import { NewUser, User } from "../interfaces/users";
import { postCall } from "./utils";
import { Url } from "./url";


export async function createUserCall(email: string, password: string) {
    const user: NewUser = { email, password };
    return postCall<NewUser, User>(
        new Url().usersCreate,
        user,
        201
    );
}
