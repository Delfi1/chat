<script setup lang="ts">
import { FileRefPayload } from '../api';

const props = defineProps<{
  payload: FileRefPayload,
}>();

function formatSize(size: number): string {
  var i = size == 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024));
  var data = (size / Math.pow(1024.0, i)).toFixed(1);
  var pref = ['B', 'Kb', 'Mb', 'Gb', 'Tb'][i];

  return `${data} ${pref}`;
}
</script>

<template>
  <div class="file">
    <i></i>
    <p class="filename" v-text="props.payload.name"></p>
    <p class="filesize" v-text="formatSize(props.payload.size)"></p>
  </div>
</template>

<style>
.file {
  min-width: 40px;
  margin-top: 4px;
  height: 40px;
  padding: 4px;
  background-color: #222;
}

p.filename {
  text-align: center;
  font-size: 12px;
  color: #fff;
}

</style>