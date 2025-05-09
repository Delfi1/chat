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
function sender(users: UserPayload[], message: MessagePayload): UserPayload | undefined {
  return users.find((user) => user.id == message.sender);
}

function get_messsage(messages: MessagePayload[], id: number | null): MessagePayload | undefined {
  if (!id) { return undefined };
  return messages.find((message) => message.id == id);
}

export { sender, get_messsage }
export type { UserPayload, MessagePayload, FileRefPayload, SendPayload }