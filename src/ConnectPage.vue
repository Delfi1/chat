<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const props = defineProps<{
  connected: boolean,
  errorMsg: string,
  connecting: boolean
}>();
const emit = defineEmits(['on_connect']);

const addr = ref('localhost');

function connect() {
  emit('on_connect', addr.value);
}
</script>

<template>
  <div v-if="!connecting" class="card">
    <input placeholder="Address" type="text" v-model="addr" />
    <button @click="connect">Connect</button>
    <p v-if="props.errorMsg">Error: {{ props.errorMsg }}</p>
  </div>
	<div v-if="connecting" class="card">
		<div class="loader"></div>
		<h2>Loading...</h2>
  </div>
</template>

<style>
* {
  font-family: Avenir;
}
.loader {
  border: 8px solid #f3f3f3; /* Light grey */
  border-top: 8px solid #3498db; /* Blue */
  border-radius: 50%;
  margin-top: 60px;
  margin-left: 60px;
  width: 60px;
  height: 60px;
  animation: spin 2.5s linear infinite;
}
.card {
  width: 200px;
  position: relative;
  left: calc(50vw - 100px);
  text-align: center;
  padding-top: 6vw;
}
input {
  width: calc(100% - 16px);
  margin-top: 12px;
  padding: 8px;
  background-color: #e6f7ff;
  outline: none;
  border: 1px solid #e6f7ff;
}
button {
  margin-top: 12px;
  width: 100%;
  padding: 8px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>