<script setup lang="ts">
  import { ref } from "vue";
  import { MessagePayload, UserPayload } from '../api';
  import { marked } from 'marked';
  import File from './File.vue';

  import ContextMenu from 'primevue/contextmenu';

  const props = defineProps<{
    self: UserPayload | undefined,
    user: UserPayload | undefined,
    payload: MessagePayload,
  }>();
  const emit = defineEmits(['edit', 'reply', 'remove', 'download']);
  import { openUrl } from '@tauri-apps/plugin-opener';

  function copy_text() {

  };

  // Message context menus
  const menu = ref();
  const owner_items = ref([
    { label: 'Remove', icon: 'pi pi-trash', command: () => emit("remove", props.payload.id) },
    { label: 'Copy text', icon: 'pi pi-copy', command: () => copy_text },
    { label: 'Edit', icon: 'pi pi-file-edit' }
  ]);

  const items = ref([
    { label: 'Copy text', icon: 'pi pi-copy', command: () => copy_text },
  ]);

  function onRightClick(event: MouseEvent) {
    menu.value.show(event);
  };

  // time formatter
  function time(): string {
    var date = new Date(props.payload.sent);
    var current = new Date();

    // if today
    if (current.toLocaleDateString() == date.toLocaleDateString()) {
      return date.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" });
    }
    
    return date.toLocaleDateString();
  }

  function is_owner(): boolean {
    return props.user?.id == props.self?.id;
  }

  // prevent url opening in href
  function on_click(event: MouseEvent) {
    var target = event.target;

    if (target) {
      var url = target['href'];

      if (url) {
        event.preventDefault();
        openUrl(url);
      }
    }
  }
</script>

<template>
  <div v-if="!is_owner()" class="message-container received" @contextmenu="onRightClick">
    <ContextMenu ref="menu" :model="items" />
    <div class="avatar"></div>
    <div class="message">
      <p class="name" v-text="props.user?.name"></p>
      <div @click="on_click" v-html="marked(props.payload.text)" class="text"></div>
      <File v-if="payload.file" @click="emit('download', payload.file)" :payload="payload.file"></File>
      <div class="time" v-text="time()"></div>
    </div>
  </div>
  
  <div v-if="is_owner()" class="message-container sent" @contextmenu="onRightClick">
    <ContextMenu ref="menu" :model="owner_items" />
    <div class="message">
      <p class="name" v-text="props.user?.name"></p>
      <div @click="on_click" v-html="marked(props.payload.text)" class="text"></div>
      <File v-if="payload.file" @click="emit('download', payload.file)" :payload="payload.file"></File>
      <div class="time" v-text="time()"></div>
    </div>
    <div class="avatar"></div>
</div>
</template>

<style>
.message-container {
  clear: both;
  position: relative;
  z-index: 1;
}

a.p-contextmenu-item-link:hover {
  color: red;
}

.message-container .message {
  white-space: initial;
  position: relative;
  overflow-wrap: anywhere;
  padding: 8px;
  margin: 8px 8px;
  line-height: 18px;
  min-width: 80px;
  font-size: 15px;
  max-width: 60%;
  float: left;
}

.message-container .name {
  user-select: none;
}

.message-container .time {
  user-select: none;
}

.message-container:last-child {
  display: block;
  padding-bottom: anchor-size(height);
}

.name {
  margin-top: 1px;
  color: #fff;
}

.received .name {
  margin-left: 2px;
  margin-bottom: 2px;
}

.sent .name {
  text-align: end;
  margin-right: 2px;
  margin-bottom: 2px;
} 

.message:after {
  position: absolute;
  content: "";
  width: 0;
  height: 0;
  border-style: solid;
}

.received .message {
  margin-left: 16px;
  border-radius: 0px 5px 5px 5px;
  background-color: #2e343d;
}

.received .message:after {
  border-width: 0px 10px 10px 0;
  border-color: transparent #2e343d transparent transparent;
  top: 0;
  left: -8px;
}

.sent .message {
  background-color: #6b8afd;
  margin-right: 16px;
  border-radius: 5px 5px 0px 5px;
  float: right;
}

.sent .message:after {
  border-width: 10px 0px 0px 10px;
  border-color: transparent transparent transparent #6b8afd;
  bottom: 0;
  right: -8px;
}

.time {
  color: #d6d6d6;
  font-size: 10px;
}

.sent .time {
  text-align: end;
}

.text p {
  white-space: pre-wrap;
}

</style>