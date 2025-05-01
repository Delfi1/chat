<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { UserPayload, MessagePayload } from './api';

const props = defineProps<{
  user: UserPayload | undefined
  users: UserPayload[],
  messages: MessagePayload[]
}>();
const emit = defineEmits(['onSignup', 'onLogin']);
const text = ref('');

function send() {
  invoke('send_message', {"text": text.value});
  text.value = '';
}

function sender(message: MessagePayload): UserPayload | undefined {
  return props.users.find((user) => user.id == message.sender);
}
</script>

<template>
  <input v-model="text" />
  <button @click="send">Send</button>
  <h3 v-for="message in props.messages">{{ sender(message)?.name }}: {{ message.text }}</h3>
</template>