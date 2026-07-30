import { ToDoItems } from 
"../interfaces/toDoItems";
import { deleteCall } from "./utils";
import { Url } from "./url";

export async function deleteToDoItemCall(
    title: string) {
    return deleteCall<ToDoItems>(
        new Url().deleteUrl(title),
        200
    );
}
