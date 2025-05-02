<script setup lang="ts">
import { ref } from 'vue';
import { UserPayload, MessagePayload, sender } from './api';
import Message from './components/Message.vue';
import User from './components/User.vue';

const props = defineProps<{
  user: UserPayload | undefined
  users: UserPayload[],
  messages: MessagePayload[]
}>();
const emit = defineEmits(['send_message', 'logout']);
const text = ref('');

function send() {
  emit("send_message", text.value);
  text.value = '';
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
        <h1>Users: {{ all_online().length }}</h1>
        <User v-for="user in all_online()" :payload="user"></User>
      </div>
    </div>
    <div class="main-box">
      <div class="messages-box">
        <Message v-for="message in props.messages" :user="sender(props.users, message)" :payload="message"></Message>
      </div>
      <div class="write-box">
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
  display: flex;
  width: 100%;
  height: 100%;
}

.container button {
  margin: 10px;
  width: 80px;
  height: 25px;
}

/* Controls box styles */
.control-box {
  width: 350px;
  height: 100%;
  display: inline;
}

.main-controls {
  width: 100%;
  height: 25%;
  
  background-color: #75c3ff;
}

.users-list {
  width: 100%;
  height: 75%;
  background-color: #87cbff;
}

/* Main box styles */
.main-box {
  width: 100%;
  height: 100%;
  display: inline-flexbox;
}

.messages-box {
  width: 100%;
  height: 90%;
  overflow-y: scroll;
}

.write-box {
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