<script setup lang="ts">
import { ref } from 'vue';
import { UserPayload, MessagePayload, sender } from './api';
import Message from './components/Message.vue';
import User from './components/User.vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  self: UserPayload | undefined
  users: UserPayload[],
  messages: MessagePayload[]
}>();
const emit = defineEmits(['logout']);
const text = ref('');

function send() {
  invoke('send_message', { "text": text.value } );
  text.value = '';
}

function remove(id: number) {
  invoke('remove_message', { "id": id });
}

function all_online(): UserPayload[] {
  return props.users.filter((u) => u.online);
}
</script>

<template>
  <div class="container">
    <div class="control-box">
      <div class="main-controls">
        <button @click="emit('logout')">Logout</button>
      </div>
      <div class="users-list">
        <h2>Online users: {{ all_online().length }}</h2>
        <User v-for="user in all_online()" :payload="user"></User>
      </div>
    </div>

    <div class="main-box">
      <div class="messages-box" id="messages-area">
        <Message v-for="message in props.messages" :self="self" :user="sender(props.users, message)" :payload="message" @remove="remove"></Message>
      </div>
      <div class="input-panel">
        <div class="input-box">
          <div class="line-box">
            <input v-model="text" placeholder="Send message" v-on:keyup.enter="send" />
            <button @click="send">Send</button>
          </div>
        </div>
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

.container button {
  background-color: #0091ff;
  margin: 4px;
  transition: all 0.4s ease-in-out;
  transition-property: color, background-color;
  color: #080710;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

.container button:hover {
  background-color: #0065b1;
  color: #fff;
}

.control-box {
  height: 100%;
  flex: 0.3;
  margin-left: 10px;
  margin-right: 10px;
  display: inline;
}

.control-box button {
  margin: 10px;
  padding: 5px;
}

.main-controls {
  width: 100%;
  height: 25%;

  position: relative;
  width: 100%;
  top: 1.5%;

  border-radius: 4px;
  background-color: #62bbff;
}

.users-list {
  position: relative;
  width: 100%;
  height: 70%;
  top: 2.5%;
  padding: 5px;
  border-radius: 4px;
  background-color: #87cbff;
}

/* Main part */
.main-box {
  height: 100%;
  display: inline;
  flex: 1;
}

.messages-box {
  width: 100%;

  height: 90%;
  overflow-y: scroll;
}

.input-panel {
  width: 100%;
  height: 8%;
  display: grid;
  place-items: center;
}

.input-box {
  width: 90%;
  height: 90%;
  display: grid;
  place-items: center;
  background-color: #b6dfff;
}

.line-box {
  width: 100%;
  display: flex;
}

.line-box input {
  width: 90%;
  font-size: 16px;
  height: 60%;
  outline: none;
  border: none;
  background-color: transparent;
  align-self: center;
  margin-left: 20px;
  margin-right: 15px;
}

.line-box input::placeholder {
  color: #3f627d;
}

.line-box button {
  width: 7%;
  font-size: 12px;
  height: 30px;
}

</style>