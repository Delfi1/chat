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

export type { UserPayload, MessagePayload }