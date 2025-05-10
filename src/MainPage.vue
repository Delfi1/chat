<script setup lang="ts">
  import { onBeforeMount, ref } from 'vue';
  import { UserPayload, MessagePayload, sender, get_messsage, SendPayload } from './api';
  import Message from './components/Message.vue';
  //import User from './components/User.vue';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { listen } from '@tauri-apps/api/event';

  import ProgressBar from 'primevue/progressbar';
  import ContextMenu from 'primevue/contextmenu';
  import { MenuItem } from "primevue/menuitem";

  const props = defineProps<{
    self: UserPayload | undefined
    users: UserPayload[],
    messages: MessagePayload[]
  }>();
  const emit = defineEmits(['logout']);

  const menu = ref();
  const Pages = {
    chat: 'chat',
    account: 'account',
    settings: 'settings'
  };
  const page = ref(Pages.chat);

  const text = ref('');
  const attached = ref<string | null>(null);

  function attach() {
    open().then((path) => {
      if (path) {
        attached.value = path;
      }
    })
  }

  function remove_attach() {
    attached.value = null;
  }

  function send() {
    invoke('send_message', { "text": text.value, "reply": replying.value?.id, "attached": attached.value } );
    
    text.value = '';
    replying.value = null;
    attached.value = null;
  }

  const replying = ref<MessagePayload | null>(null);
  function reply(message: MessagePayload) {
    console.log(message);
    replying.value = message;
  }

  const sending = ref(false);
  const sending_state = ref(0);

  function remove(id: number) {
    invoke('remove_message', { "id": id });
  }

  function edit(id: number, text: string) {
    invoke('edit_message', { "id": id, "text": text });
  }

  const menu_items = ref<MenuItem[]>();
  function open_menu(event: MouseEvent, items: MenuItem[]) {
    menu_items.value = items;
    menu.value.show(event);
  }

  // todo messages view

  onBeforeMount(() => {
    listen<SendPayload>('send_status', (event) => {
      sending.value = true;

      console.log(event.payload);
      if (event.payload.ready == event.payload.lenght) {
        // Wait before hide progressbar
        setTimeout(() => {
          sending.value = false;
          sending_state.value = 0;
        }, 1200);
      }

      sending_state.value = Math.floor((event.payload.ready / event.payload.lenght) * 100);
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
      <ContextMenu ref="menu" :model="menu_items"/>
      <div v-if="page == Pages.chat" class="chat-page">
        <div class="chat-box">
          <div class="messages-box" id="messages-area">
            <Message v-for="message in props.messages" :self="self" :user="sender(props.users, message)" :payload="message" :reply="get_messsage(props.messages, message.reply)" @open_menu="open_menu" @reply="reply" @edit="edit" @remove="remove"></Message>
          </div>
          <div id="input-box" class="input-box">
            <div class="send-data-box">
              <div v-if="replying" class="replying-box">
                <p class="reply-message">Replying to: {{ sender(props.users, replying)?.name }}</p>
              </div>
              <ProgressBar v-if="sending" :value="sending_state" />
              <p v-if="attached" class="attached-file" @click="remove_attach" v-text="attached"></p>  
            </div>
            <div class="send-box">
              <button @click="attach" class="file-input">
                <i class="pi pi-file-arrow-up"></i>
              </button>
              <textarea placeholder="Send message" v-model="text" v-on:keyup.enter.exact="send"></textarea>
              <button @click="send">Send</button>
            </div>
          </div>
        </div>
        <div class="details">
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

.p-progressbar {
  width: 140px;
  max-height: 12px;
  margin-left: 12px;
  margin-top: 5px;
}

.left-menu {
  width: 80px;
  height: 100%;
  display: inline;
  user-select: none;
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

/* Chat page Central panel, messages */
.chat-box {
  height: 100%;
  width: 100%;
  background-color: #202329;
  display: flex;
  flex-flow: column;
}

.messages-box {
  width: 100%;
  height: 100%;
  position: relative;
  overflow-y: scroll;
}

.input-box {
  width: 100%;
  min-height: 70px;
  position: relative;
  display: inline;
  vertical-align: bottom;
  background-color: #131313;
}

.input-box p.attached-file {
  margin-left: 5px;
  height: 17px;
  padding-top: 5px;
  font-size: 12px;
  margin-bottom: 80px;
}

.input-box p.attached-file:hover {
  color: rgb(130, 27, 27);
}

.input-box p.reply-message {
  margin-left: 25px;
}

.replying-box {
  width: 100%;
  position: relative;
  height: 30px;
  padding-top: 8px;
}

.send-data-box {
  width: 100%;
  margin-bottom: 60px;
}

.send-box {
  width: 100%;
  display: flex;
  position: absolute;
  bottom: 0px;
}

.send-box .file-input {
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

.send-box .file-input i {
  transition: all 0.3s ease;
  font-size: 1.2rem;
}

.send-box .file-input i:hover {
  color: #6b8afd;
  font-size: 1.1rem;
}

.send-box .file-input i:active {
  color: #587cff;
  font-size: 1rem;
}

.send-box textarea {
  min-height: 80%;
  width: 80%;
  overflow-y: hidden;
  background-color: transparent;
  outline: none;
  border: none;
  resize: none;

  margin-left: 15px;
  margin-top: 5px;
  padding-top: 10px;
  font-size: 16px;
}

.send-box button {
  width: 60px;
  font-size: 16px;
  background-color: transparent;
  border-radius: 4px;
  padding: 4px;
  margin-bottom: 10px;
  margin-left: 8px;
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