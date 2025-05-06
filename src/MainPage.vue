<script setup lang="ts">
import { onBeforeMount, ref } from 'vue';
import { UserPayload, MessagePayload, sender, SendPayload } from './api';
import Message from './components/Message.vue';
//import User from './components/User.vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { listen } from '@tauri-apps/api/event';

// todo: message loading

const props = defineProps<{
  self: UserPayload | undefined
  users: UserPayload[],
  messages: MessagePayload[]
}>();
const emit = defineEmits(['logout']);

const Pages = {
  chat: 'chat',
  account: 'account',
  settings: 'settings'
};
const page = ref(Pages.chat);

const text = ref('');
function send() {
  //invoke('send_file');
  invoke('send_message', { "text": text.value } );
  text.value = '';
}

const sending = ref(false);
const sending_state = ref(0);

function attach() {
  open().then((path) => {
    console.log(path);

    if (path) {
      invoke('attach_file', { "path": path });
    }
  })
}

function remove(id: number) {
  invoke('remove_message', { "id": id });
}

onBeforeMount(() => {
  listen<SendPayload>('send_status', (event) => {
    sending.value = event.payload.ready != event.payload.lenght;
    sending_state.value = (event.payload.ready / event.payload.lenght) * 100;
  });
});
</script>

<template>
  <div class="container">
    <div class="left-menu">
      <div class="logo">
        <h1>De</h1>
      </div>
      <div class="menu-buttons">
        <div class="top">
          <div class="menu-btn">
            <button @click="page = Pages.chat"><i class="pi pi-comments"></i></button>
            <p>Chat</p>
          </div>

          <div class="menu-btn">
            <button @click="page = Pages.account"><i class="pi pi-user"></i></button>
            <p>Account</p>
          </div>
        </div>
        <div class="bottom">
          <div class="menu-btn">
            <button @click="page = Pages.settings"><i class="pi pi-cog"></i></button>
            <p>Settings</p>
          </div>
        </div>
      </div>
    </div>
    
    <div class="central-box">
      <div v-if="page == Pages.chat" class="chat-page">
        <div class="chat-box">
          <div class="messages-box" id="messages-area">
            <Message v-for="message in props.messages" :self="self" :user="sender(props.users, message)" :payload="message" @remove="remove"></Message>
          </div>
          <div class="input-box">
            <button @click="attach" class="file-input">
              <i class="pi pi-file-arrow-up"></i>
            </button>
            <input placeholder="Send message" v-model="text" v-on:keyup.enter="send"/>
            <button @click="send">Send</button>
          </div>
        </div>
        <div class="details">
          <progress v-if="sending" :value="sending_state" max="100">70 %</progress>
        </div>
      </div>

      <div v-if="page == Pages.account" class="account-page">
        <button @click="emit('logout')">Logout</button>
      </div>

      <div v-if="page == Pages.settings" class="settings-page">
      
      </div>
    </div>
  </div>
</template>

<style>
.container {
  width: 100%;
  height: 100%;
  display: flex;
}

.left-menu {
  width: 80px;
  height: 100%;
  display: inline;
}

.left-menu .logo {
  margin: 10px;
  width: 55px;
  height: 55px;
  border-radius: 12px;
  display: grid;
  align-content: center;
}

.left-menu .logo h1 {
  font-weight: 500;
  text-align: center;
  transition: all 0.6s ease;
}

.left-menu .logo h1:hover {
  font-weight: 600;
  font-size: x-large;
  color: #6b8afd;
}

.left-menu .menu-buttons {
  margin-top: 10px;
  width: 100%;
}

.menu-buttons .menu-btn {
  height: 100%;
  align-content: center;
  text-align: center;
  font-size: 12px;
}

.menu-buttons .bottom {
  position: absolute;
  bottom: 0;
}

.menu-buttons .menu-btn {
  margin: 15px;
}

.menu-btn button {
  width: 45px;
  height: 45px;
  background-color: transparent;
  outline: none;
  border: none;
  border-radius: 12px;
}

.menu-btn button i {
  font-size: 1.4rem;
  transition: all 0.3s ease;
}

.menu-btn button i:hover {
  font-size: 1.3rem;
  color: #6b8afd;
}

.menu-btn button i:active {
  color: #587cff;
  font-size: 1.1rem
}

.central-box {
  width: 100%;
  height: 100%;
}

.chat-page {
  width: 100%;
  height: 100%;
  display: flex;
}

/* Chat page Central panel, chat */
.chat-box {
  height: 100%;
  width: 100%;
  border-radius: 16px;
  background-color: #202329;
  display: flex;
  flex-flow: column;
}

.messages-box {
  width: 100%;
  height: 100%;
  overflow-y: scroll;
}

.input-box {
  width: 100%;
  height: 50px;
  display: flex;
}

.input-box input[type="file"] {
  display: none;
}

.input-box .file-input {
  align-items: center;
  text-align: center;
  align-content: center;
  width: 35px;
  height: 35px;
  padding: 4px;
  margin-left: 15px;
  margin-top: 5px;
  background-color: #131313;
  border: #131313;
  border-radius: 12px;
}

.input-box .file-input i {
  transition: all 0.3s ease;
  font-size: 1.2rem;
}

.input-box .file-input i:hover {
  color: #6b8afd;
  font-size: 1.1rem;
}

.input-box .file-input i:active {
  color: #587cff;
  font-size: 1rem;
}

.input-box input {
  height: 80%;
  width: 80%;
  background-color: transparent;
  outline: none;
  border: none;

  margin-left: 15px;
  margin-top: 5px;
  font-size: 16px;
}

.input-box button {
  height: 80%;
  font-size: 16px;
  background-color: transparent;
  width: 60px;
  border-radius: 4px;
  padding: 4px;
  margin-left: 8px;
  margin-top: 5px;
}

/* Chat details - users online, etc */
.details {
  height: 100%;
  width: 300px;
  right: 0;
}

.account-page button {
  width: 60px;
  height: 25px;
  margin: 10px;
  background-color: #202329;
}

</style>