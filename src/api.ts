//import { invoke } from "@tauri-apps/api/core";

interface UserPayload {
    id: number,
    name: string,
    online: boolean
}

interface MessagePayload {
    sender: number,
    sent: number,
    text: string
}

// Message sender
function sender(users: UserPayload[], message: MessagePayload): UserPayload | undefined {
    return users.find((user) => user.id == message.sender);
}

export { sender }
export type { UserPayload, MessagePayload }