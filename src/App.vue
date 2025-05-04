<script setup lang="ts">
import { Window, LogicalSize } from '@tauri-apps/api/window';
import { onBeforeMount, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { LazyStore } from '@tauri-apps/plugin-store';
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
const store = new LazyStore('user.json');

function connect(addr: string) {
  invoke('connect', {"addr": addr});
  store.set('addr', addr);
  connecting.value = true;
  console.log("connect");
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

function scroll_area() {
  var area = document.getElementById("messages-area") as HTMLElement;
  if (area.scrollTo) {
    area.scrollTo(0, area.scrollHeight);
  }
}

function update_lists() {
  invoke<UserPayload[]>('get_users').then((result) => {
    users.value = result;
  });

  invoke<MessagePayload[]>('get_messages', {"start": 0, "end": 10000}).then((result) => {
    messages.value = result;
    scroll_area();
  });

  invoke<number>('messages_len').then((result) => {
    messages_len.value = result;
    scroll_area();
  });
}

function main_state() {
  appWindow.setResizable(true);
  appWindow.setMaximizable(true);
  appWindow.setMaxSize(undefined);
  appWindow.setMinSize(new LogicalSize(1100, 600));
  appWindow.setSize(new LogicalSize(1100, 600));

  connected.value = true;
  connecting.value = false;
  connectErrorMsg.value = '';
}

function load_connect() {
  store.get<string>('addr').then((addr) => {
    if (addr) { connect(addr) };
  });
}

function connect_state() {
  appWindow.setResizable(false);
  appWindow.setMaximizable(false);
  appWindow.setMinSize(undefined);
  appWindow.setMaxSize(new LogicalSize(400, 600));
  appWindow.setSize(new LogicalSize(400, 600));

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

  // load address and if exists - connect 
  load_connect();
  
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
@import url('https://fonts.googleapis.com/css2?family=Archivo:ital,wght@0,100..900;1,100..900&display=swap');

/* Colors palette */
/*
#131313
#2e333d
#6b8afd
#ffffff
*/

*, *:before, *:after{
  padding: 0;
  margin: 0;
  box-sizing: border-box; 
}
* {
  font-family: Archivo;
  color: #fff;
}

button, h1, h2, h3, h4, h5, p {
  user-select: none;
}

.main {
  width: 100%;
  height: 100%;
  position: absolute;
  background-color: #131313;
}

</style>