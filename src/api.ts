//import { invoke } from "@tauri-apps/api/core";

interface UserPayload {
    id: number,
    name: string,
    id_admin: boolean,
    online: boolean
}

interface MessagePayload {
    id: number,
    sender: number,
    sent: number,
    text: string,
    file: FileRefPayload | null
}

interface FileRefPayload {
    id: number,
    name: string,
    size: number
}

interface SendPayload {
    ready: number,
    lenght: number
}

// Message sender
function sender(users: UserPayload[], message: MessagePayload): UserPayload | undefined {
    return users.find((user) => user.id == message.sender);
}

export { sender }
export type { UserPayload, MessagePayload, FileRefPayload, SendPayload }