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

// Setup document
document.querySelector("html")!.classList.toggle("darkmode");
document.oncontextmenu = (event) => { event.preventDefault() };

const appWindow = Window.getCurrent();
const connected = ref(false);
const connecting = ref(false);
const loginErrorMsg = ref('')
const connectErrorMsg = ref('');

const self = ref<UserPayload>();
const users = ref<Map<number, UserPayload>>(new Map());

const messages = ref<Map<number, MessagePayload>>(new Map());
const store = new LazyStore('user.json');

function connect(addr: string) {
  invoke('connect', {"addr": addr});
  store.set('addr', addr);
  connecting.value = true;
  console.log("connecting...");
}

function login(name: string, password: string) {
  invoke('login', { name, password });
}

function signup(name: string, password: string) {
  invoke('signup', { name, password });
}

function logout() {
  invoke('logout');
  self.value = undefined;
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
  self.value = undefined;
  users.value = new Map();
  messages.value = new Map();
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

  // Users
  listen<UserPayload>('user_inserted', (ev) => {
    if (ev.payload) {
      users.value.set(ev.payload.id, ev.payload);
    }
  });

  listen<UserPayload>('user_updated', (ev) => {
    if (ev.payload) {
      users.value.set(ev.payload.id, ev.payload);
    }
  });

  // Messages
  listen<MessagePayload>('message_inserted', (ev) => {
    if (ev.payload) {
      messages.value.set(ev.payload.id, ev.payload);
    }
  });

  listen<MessagePayload>('message_updated', (ev) => {
    if (ev.payload) {
      messages.value.set(ev.payload.id, ev.payload);
    }
  });

  // Loginned User is inserted
  listen<UserPayload>('loginned', (ev) => {
    self.value = ev.payload;
    loginErrorMsg.value = '';
  });

  listen<UserPayload>('user_removed', (ev) => {
    console.log(ev.payload);

    if (ev.payload) {
      users.value.delete(ev.payload.id);
    }
  });

  listen<MessagePayload>('message_removed', (ev) => {
    console.log(ev.payload);

    if (ev.payload) {
      messages.value.delete(ev.payload.id);
    }
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
  <AuthPage v-if="connected && !self" :error-msg="loginErrorMsg" @onLogin="login" @onSignup="signup"></AuthPage>
  <MainPage v-if="connected && self" @logout="logout" :self="self" :messages="messages" :users="users"></MainPage>
</div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Archivo:ital,wght@0,100..900;1,100..900&display=swap');

*, *:before, *:after{
  padding: 0;
  margin: 0;
  box-sizing: border-box; 
}

* {
  font-family: Archivo;
  color: #fff;
}

.main {
  width: 100%;
  height: 100%;
  position: fixed;
  background-color: #131313;
}

</style>