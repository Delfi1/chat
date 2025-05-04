<script setup lang="ts">
  //import { ref } from 'vue';
  import { MessagePayload, UserPayload } from '../api';
  import { marked } from 'marked';

  const props = defineProps<{
    self: UserPayload | undefined,
    user: UserPayload | undefined,
    payload: MessagePayload,
  }>();
  const emit = defineEmits(['edit', 'reply', 'remove']);
  import { openUrl } from '@tauri-apps/plugin-opener';

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

  /*
  function can_remove(): boolean {
    return is_owner() || props.self?.id_admin as boolean;
  }

  function remove() {
    console.log("Remove", props.payload.id);
    emit("remove", props.payload.id);
  }
  */

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
  <div v-if="!is_owner()" class="message received">
    <div @click="on_click" v-html="marked(props.payload.text)" class="text"></div>
    <div class="time" v-text="time()"></div>
  </div>
  <div v-if="is_owner()" class="message sent">
    <div @click="on_click" v-html="marked(props.payload.text)" class="text"></div>
    <div class="time" v-text="time()"></div>
  </div>
</template>

<style>
  .message {
    background-color: #2e343d;
    clear: both;
    line-height: 18px;
    min-width: 80px;
    font-size: 15px;
    padding: 8px;
    position: relative;
    margin: 8px 0;
    max-width: 75%;
    white-space: initial;
    overflow-wrap: anywhere;
    z-index: 1;
  }

  .message:after {
    position: absolute;
    content: "";
    width: 0;
    height: 0;
    border-style: solid;
  }
  
  .time {
    color: #d6d6d6;
    font-size: 10px;
  }
  
  .sent .time {
    text-align: end;
  }

  .message:first-child {
    margin: 16px 0 8px;
  }

  .message:last-child {
    margin: 8px 0 8px 16px;
  }

  .message.received {
    margin-left: 8px;
    border-radius: 0px 5px 5px 5px;
    float: left;
  }

  .message.received .metadata {
    padding: 0 0 0 16px;
  }

  .message.received:after {
    border-width: 0px 10px 10px 0;
    border-color: transparent #2e343d transparent transparent;
    top: 0;
    left: -8px;
  }

  .message.sent {
    background-color: #6b8afd;
    margin-right: 8px;
    border-radius: 5px 0px 5px 5px;
    float: right;
  }

  .message.sent:after {
    border-width: 0px 0 10px 10px;
    border-color: transparent transparent transparent #6b8afd;
    top: 0;
    right: -8px;
  }

</style>