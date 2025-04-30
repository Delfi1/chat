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
const connect_error = ref<string>();

const user = ref<UserPayload>();
const users = ref([] as UserPayload[]);
const messages = ref([] as MessagePayload[]);

function login(name: string, password: string) {
  invoke('login', { name, password });
}

function signup(name: string, password: string) {
  invoke('signup', { name, password });
}

onBeforeMount(() => {
  listen('on_connect', (_ev) => {
    connected.value = true;
  });

  listen('on_connect_error', (ev) => {
    connected.value = false;
    connect_error.value = ev.payload as string;

    /// Try reconnect
    invoke('connect');
  });

  listen('on_disconnect', (_ev) => {
    connected.value = false;
    loginned.value = false;
    user.value = undefined;
    users.value = [];
    messages.value = [];

    /// Try reconnect
    invoke('connect');
  });

  // Users updates
  listen('users_updated', () => {

  });

  // Messages updates
  listen('messages_updated', () => {

  });

  // Message inserted
  listen<UserPayload>('loginned', (ev) => {
    loginned.value = true;
    user.value = ev.payload;
  });

  /// Connect to STDB
  invoke('disconnect');
  invoke('connect');
});
</script>

<template>
<ConnectPage :errorMsg="connect_error" v-if="!connected"></ConnectPage>
<AuthPage v-if="connected && !loginned" @onLogin="login" @onSignup="signup"></AuthPage>
<MainPage v-if="connected && loginned" :user="user" :messages="messages" :users="users"></MainPage>

</template>

<style>

</style>