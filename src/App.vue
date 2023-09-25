<script setup lang="ts">
  import Navbar from './components/Navbar.vue';
  import Home from './pages/Home.vue'
  import Add from './pages/Add.vue';
</script>

<template>
    <Navbar />
    <div :class="$style.container">
      <Add />
    </div>
</template>


<style module>
.container{
  margin-left: 7vw;
  padding: 0;
  display: flex;
  width: 93vw;
}

</style>