<template>
  <div class="m-1.5 mb-2" style="width: 70px">
    <a href="https://github.com/panstromek/zebra-rs" class="text-gray-200 text-xs" style="display: flex; align-items: center;" target="_blank">
      <img src="/GitHub-Mark-Light-32px.png" alt="Github Icon" width="16" height="16" class="mr-2">
      Source
    </a>
  </div>
  <HelloWorld />
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import HelloWorld from './components/HelloWorld.vue'

export default defineComponent({
  name: 'App',
  components: {
    HelloWorld
  }
})
</script>
