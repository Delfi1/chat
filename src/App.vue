<script setup lang="ts">
import { Window, LogicalSize } from '@tauri-apps/api/window';
import { onBeforeMount, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import 'primeicons/primeicons.css'

import ConnectPage from './ConnectPage.vue';
import AuthPage from './AuthPage.vue';
import MainPage from './MainPage.vue';
import { UserPayload, MessagePayload } from './api.ts';

const appWindow = Window.getCurrent();
const loginned = ref(false);
const connected = ref(false);
const connecting = ref(false);
const loginErrorMsg = ref('')
const connectErrorMsg = ref('');

const self = ref<UserPayload>();
const users = ref([] as UserPayload[]);

const messages_len = ref(0);
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

function logout() {
  invoke('logout');
  console.log("Logout");
  loginned.value = false;
  self.value = undefined;
}

function update_lists() {
  invoke<UserPayload[]>('get_users').then((result) => {
    users.value = result;
  });

  invoke<MessagePayload[]>('get_messages', {"start": 0, "end": 100}).then((result) => {
    messages.value = result;
    var area = document.getElementById("messages-area") as HTMLElement;
    area.scrollTo(0, area.scrollHeight);
  });

  invoke<number>('messages_len').then((result) => {
    messages_len.value = result;
    var area = document.getElementById("messages-area") as HTMLElement;
    area.scrollTo(0, area.scrollHeight);
  });
}

function main_state() {
  appWindow.setSize(new LogicalSize(800, 600));
  appWindow.setMinSize(new LogicalSize(800, 600));
  appWindow.setMaxSize(undefined);
  appWindow.setResizable(true);
  appWindow.setMaximizable(true);

  connected.value = true;
  connecting.value = false;
  connectErrorMsg.value = '';
}

function connect_state() {
  appWindow.setSize(new LogicalSize(400, 600));
  appWindow.setMinSize(undefined);
  appWindow.setMaxSize(new LogicalSize(400, 600));
  appWindow.setResizable(false);
  appWindow.setMaximizable(false);

  connected.value = false;
  loginned.value = false;
  self.value = undefined;
  users.value = [];
  messages.value = [];
}

onBeforeMount(() => {
  listen('on_connect', (_ev) => {
    main_state();
  });

  listen<string>('on_connect_error', (ev) => {
    connectErrorMsg.value = ev.payload;
    connecting.value = false;
    connect_state();
  });

  listen<string>('on_login_error', (ev) => {
    loginErrorMsg.value = ev.payload;
  });

  listen('on_disconnect', (_ev) => {
    connect_state();
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
    self.value = ev.payload;
    loginErrorMsg.value = '';

    update_lists();
  });

  listen('user_removed', (_ev) => {
    update_lists();
  });

  listen('message_removed', (_ev) => {
    update_lists();
  });

  // Set current state as connect
  connect_state();
});
</script>

<template>
<div class="main">
  <ConnectPage :connecting="connecting" :connected="connected" :errorMsg="connectErrorMsg" @on_connect="connect" v-if="!connected"></ConnectPage>
  <AuthPage v-if="connected && !loginned" :error-msg="loginErrorMsg" @onLogin="login" @onSignup="signup"></AuthPage>
  <MainPage v-if="connected && loginned" @logout="logout" :self="self" :messages="messages" :users="users"></MainPage>
</div>
</template>

<style>
*, *:before, *:after{
  padding: 0;
  margin: 0;
  box-sizing: border-box; 
}
* {
  font-family: Avenir;
  -webkit-user-select: none;
  -moz-user-select: -moz-none;
  -o-user-select: none;
  user-select: none;
}
.main {
  width: 100%;
  height: 100%;
  position: absolute;
  background-color: #dff1ff;

}

</style>