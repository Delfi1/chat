<script setup lang="ts">
import { onBeforeMount, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import ConnectPage from './ConnectPage.vue';
import AuthPage from './AuthPage.vue';
import MainPage from './MainPage.vue';
import { UserPayload, MessagePayload } from './api.ts';

const loginned = ref(false);
const connected = ref(false);
const connecting = ref(false);
const connect_errorMsg = ref<string>('');

const user = ref<UserPayload>();
const users = ref([] as UserPayload[]);
const messages = ref([] as MessagePayload[]);

function connect(addr: string) {
  invoke('connect', {"addr": addr});
  connecting.value = true;
}

function login(name: string, password: string) {
  invoke('login', { name, password });
}

function signup(name: string, password: string) {
  invoke('signup', { name, password });
}

function update_lists() {
  invoke<UserPayload[]>('get_users').then((result) => {
    users.value = result;
  });

  invoke<MessagePayload[]>('get_messages').then((result) => {
    messages.value = result;
  });
}

onBeforeMount(() => {
  listen('on_connect', (_ev) => {
    connected.value = true;
    connecting.value = false;
    connect_errorMsg.value = '';
  });

  listen('on_connect_error', (ev) => {
    connected.value = false;
    connect_errorMsg.value = ev.payload as string;
    connecting.value = false;
  });

  listen('on_disconnect', (_ev) => {
    connected.value = false;
    loginned.value = false;
    user.value = undefined;
    users.value = [];
    messages.value = [];
  });

  // Users updates
  listen('users_updated', () => {
    update_lists();
  });

  // Messages updates
  listen('messages_updated', () => {
    update_lists();
  });

  listen('user_inserted', () => {
    update_lists();
  });

  listen('message_inserted', () => {
    update_lists();
  });

  // Loginned User is inserted
  listen<UserPayload>('loginned', (ev) => {
    loginned.value = true;
    user.value = ev.payload;

    update_lists();
  });
});
</script>

<template>
<ConnectPage :connecting="connecting" :connected="connected" :errorMsg="connect_errorMsg" @on_connect="connect" v-if="!connected"></ConnectPage>
<AuthPage v-if="connected && !loginned" @onLogin="login" @onSignup="signup"></AuthPage>
<MainPage v-if="connected && loginned" :user="user" :messages="messages" :users="users"></MainPage>

</template>

<style>

</style>