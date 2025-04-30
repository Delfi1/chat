<script setup lang="ts">
import { onBeforeMount, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';

import AuthPage from './AuthPage.vue';
import { invoke } from '@tauri-apps/api/core';
import ConnectPage from './ConnectPage.vue';
import ChatPage from './ChatPage.vue';
import { MessagePayload, UserPayload } from './ChatPage.vue';

const user = ref();
const connected = ref(false);
const connect_error = ref('');

onBeforeMount(() => {
  listen('on_connect', (_ev) => {
    connected.value = true;
    connect_error.value = '';
  });

  listen('on_connect_error', (ev) => {
    connected.value = false;
    connect_error.value = ev.payload as string;

    /// Try reconnect
    invoke('connect');
  });

  listen('on_disconnect', (_ev) => {
    connected.value = false;
    /// Try reconnect
    invoke('connect');
  });

  // Message inserted
  listen<MessagePayload>('message_inserted', (_ev) => {

  });

  // Message inserted
  listen<UserPayload>('loginned', (ev) => {
    user.value = ev.payload;
  });

  /// Connect to STDB
  invoke('disconnect');
  invoke('connect');
});

function login(name: string, password: string) {
  invoke('login', { name, password });
}

function signup(name: string, password: string) {
  invoke('signup', { name, password });
}
</script>

<template>
<ConnectPage v-if="!connected"></ConnectPage>
<h1 v-if="connect_error.length > 0">Connect error: {{ connect_error }}</h1>

<AuthPage v-if="connected && !user" @OnSignup="signup" @OnLogin="login"></AuthPage>
<ChatPage v-if="user"></ChatPage>
</template>

<style>

</style>