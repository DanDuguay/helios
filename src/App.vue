<script setup lang="ts">
  import Navbar from './components/Navbar.vue';
  import Home from './pages/Home.vue'
  import ExpandedNav from './components/ExpandedNav.vue';
</script>

<template>
    <div :class="$style.container">
      <Navbar />
      <ExpandedNav />
      <Home />
    </div>
</template>


<style module>
.container{
  margin: 0;
  padding: 0;
}

</style>