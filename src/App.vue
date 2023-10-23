<script setup lang="ts">
  import Navbar from './components/Navbar.vue';
  import { RouterView } from 'vue-router';
</script>

<template>
    <Navbar />
    <div :class="$style.container">
      <RouterView />
    </div>
</template>


<style module>
.container{
  margin-left: 7vw;
  padding: 0;
  display: flex;
  flex-direction: column;
  width: 93vw;
}

</style>