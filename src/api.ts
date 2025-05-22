interface UserPayload {
  id: number,
  name: string,
  // base64 string
  avatar: string | null,
  is_admin: boolean,
  online: boolean
}

interface MessagePayload {
  id: number,
  sender: number,
  sent: number,
  edited: number | null,
  reply: number | null,
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
function sender(users: Map<number, UserPayload>, message: MessagePayload): UserPayload {
    let result = users.get(message.sender);

    // If user is not exists
    if (!result) {
      let unknow: UserPayload = {
        id: -1,
        name: "-",
        avatar: null,
        is_admin: false,
        online: false
      };

      return unknow
    }

    return result
}

function messagesChunk(messages: Map<number, MessagePayload>): MessagePayload[] {
  return [...messages.values()].sort((a, b) => a.sent - b.sent);
}

function getMesssage(messages: Map<number, MessagePayload>, id: number | null): MessagePayload | undefined {
  if (!id) { return undefined };
  return messages.get(id);
}

function avatarName(user: UserPayload | undefined): string | undefined {
  if (!user) { return undefined };

  return user.name.substring(0, 2)
}

export { sender, getMesssage, messagesChunk, avatarName }
export type { UserPayload, MessagePayload, FileRefPayload, SendPayload }