<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import { FileRefPayload } from '../api';

const props = defineProps<{
  payload: FileRefPayload,
  downloading: boolean
}>();
const emit = defineEmits(['open_menu', 'download', 'open', 'reveal']);

const download_item = ref([
  { label: 'Download', icon: 'pi pi-download', command: () => emit('download', props.payload) },
]);

const exists_items = ref([
  { label: 'Open', icon: 'pi pi-file-o', command: () => emit('open', filepath.value) },
  { label: 'Reveal', icon: 'pi pi-folder-open', command: () => emit('reveal', filepath.value) }
]);

function formatSize(size: number): string {
  var i = size == 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024));
  var data = (size / Math.pow(1024.0, i)).toFixed(1);
  var pref = ['B', 'Kb', 'Mb', 'Gb', 'Tb'][i];

  return `${data} ${pref}`;
}

const filepath = ref<string | null>(null);

function onFileClick(event: MouseEvent) {
  invoke<string | null>('file_path', { "payload": props.payload }).then((path) => {
    var items = path == null ? download_item : exists_items;

    filepath.value = path;
    emit("open_menu", event, items.value);
  });
};
</script>

<template>
  <div v-if="!props.downloading" class="file" @contextmenu="onFileClick">
    <p class="filename" v-text="props.payload.name"></p>
    <p class="filesize" v-text="formatSize(props.payload.size)"></p>
  </div>

  <div v-if="props.downloading" class="downloading-file">
    <p class="filename" v-text="props.payload.name"></p>
    <p class="filesize" v-text="formatSize(props.payload.size)"></p>
  </div>
</template>

<style>
.file {
  min-width: 40px;
  margin-top: 6px;
  min-height: 40px;
  padding: 4px;
  background-color: #222;
}

.downloading-file {
  min-width: 40px;
  margin-top: 6px;
  min-height: 40px;
  padding: 4px;
  background-color: #2f2f2f;
}

p.filename {
  text-align: center;
  font-size: 12px;
  color: #fff;
}

.filesize {
  user-select: none;
}

</style>