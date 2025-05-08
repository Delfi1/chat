<script setup lang="ts">
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
.loader {
  border: 8px solid #2e333d;
  border-top: 8px solid #6b8afd;
  border-radius: 50%;
  margin-top: 10px;
  width: 60px;
  height: 60px;
  animation: spin 2.5s ease-in-out infinite;
}

.card {
  justify-content: center;
  text-align: center;
  display: grid;
  place-items: center;
}

.card input{
  height: 24px;
  width: 260px;
  color: #fff;
  background: #2e333d;
  border-radius: 3px;
  padding: 0 8px;
  margin-top: 8px;
  text-align: center;
  font-size: 14px;
  font-weight: 300;
}
::placeholder{
  color: #3f627d;
}

.card button{
  margin-top: 15px;
  margin-bottom: 20px;
  width: 280px;
  background-color: #2e333d;
  color: #fff;
  transition: background-color 0.5s ease;
  padding: 15px 0;
  font-size: 18px;
  font-weight: 600;
  border-radius: 5px;
  cursor: pointer;
}
.card button:hover {
  background-color: #19202b;
}

.card p {
  width: 80%;
}

.card h2 {
  width: 120px;
  margin: 20px;
  text-align: center;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  50% { transform: rotate(180deg); }
  100% { transform: rotate(360deg); }
}
</style>