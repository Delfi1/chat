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
    text: string
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
export type { UserPayload, MessagePayload, SendPayload }