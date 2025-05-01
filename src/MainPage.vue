<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { sender, UserPayload, MessagePayload } from './api';
import Message from './components/Message.vue';

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
</script>

<template>
  <input v-model="text" />
  <button @click="send">Send</button>
  <Message v-for="message in props.messages" :user="sender(props.users, message)" :value="message" />
</template>