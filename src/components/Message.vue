<script setup lang="ts">
  import { ref } from "vue";
  import { FileRefPayload, MessagePayload, UserPayload } from '../api';
  import { marked } from 'marked';
  import File from './File.vue';

  import { invoke } from "@tauri-apps/api/core";
  import { openPath, revealItemInDir } from '@tauri-apps/plugin-opener';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { MenuItem } from "primevue/menuitem";

  const props = defineProps<{
    self: UserPayload | undefined,
    user: UserPayload | undefined,
    payload: MessagePayload,
  }>();
  const emit = defineEmits(['open_menu', 'edit', 'reply', 'remove']);

  function download(file: FileRefPayload) {  
    invoke('download_file', { "payload": file });
  }

  function open(path: string) {
    openPath(path);
  }

  function reveal(path: string) {
    revealItemInDir(path);
  }

  function copy_text() {
    writeText(props.payload.text);
  };

  const editing = ref(false);
  const edited_text = ref(props.payload.text);
  function edit() {
    editing.value = true;
  }

  function update() {
    editing.value = false;
    emit('edit', props.payload.id, edited_text.value);
  }

  function cancel() {
    editing.value = false;
    edited_text.value = props.payload.text;
  }

  // Message context menus
  const owner_items = ref([
    { label: 'Remove', icon: 'pi pi-trash', command: () => emit("remove", props.payload.id) },
    { label: 'Copy text', icon: 'pi pi-copy', command: copy_text },
    { label: 'Edit', icon: 'pi pi-file-edit', command: edit }
  ]);

  const items = ref([
    { label: 'Copy text', icon: 'pi pi-copy', command: copy_text },
  ]);

  function onReceivedClick(event: MouseEvent) {
    emit("open_menu", event, items.value);
  };

  function onSentClick(event: MouseEvent) {
    emit("open_menu", event, owner_items.value);
  };

  function file_menu(event: MouseEvent, items: MenuItem[]) {
    emit("open_menu", event, items);
  }

  // time formatter
  function time(): string {
    var current = new Date();
    var time = props.payload.edited == null ? props.payload.sent : props.payload.edited;
    var date = new Date(time);
    var info = props.payload.edited == null ? '' : 'edited ';

    // if today
    if (current.toLocaleDateString() == date.toLocaleDateString()) {
      return info + date.toLocaleTimeString(undefined, { hour: "2-digit", minute: "2-digit" });
    }
    
    return info + date.toLocaleDateString();

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
  <div v-if="!is_owner()" class="message-container received" @contextmenu="onReceivedClick">
    <div class="avatar"></div>
    <div class="message">
      <p class="name" v-text="props.user?.name"></p>
      <div @click="on_click" v-html="marked(props.payload.text)" class="text"></div>
      <File v-if="payload.file" @open_menu="file_menu" @download="download" @open="open" @reveal="reveal" :payload="payload.file"></File>
      <div class="time" v-text="time()"></div>
    </div>
  </div>
  
  <div v-if="is_owner()" class="message-container sent" @contextmenu="onSentClick">
    <div class="message">
      <p class="name" v-text="props.user?.name"></p>
      <div v-if="!editing" @click="on_click" v-html="marked(props.payload.text)" class="text"></div>
      <textarea v-if="editing" v-model="edited_text" class="editor" v-on:keyup.enter.exact="update" v-on:keyup.escape.exact="cancel"></textarea>
      <File v-if="payload.file" @open_menu="file_menu" @download="download" @open="open" @reveal="reveal" :payload="payload.file"></File>
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
  font-size: 11px;
}

.sent .time {
  text-align: end;
}

.text p {
  white-space: pre-wrap;
}

.message .editor {
  width: 100%;
  padding: 5px;
  height: 50px;
  resize: none;
  background-color: #7794ff;
}

</style>