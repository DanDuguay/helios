<script setup lang="ts">
</script>


<template>
    <div :class="$style.container">
        <div :class="$style.title">
            Available Simulations
        </div>
    </div>
</template>

<style module>
.container{
    position: fixed;
    margin: 0;
    top: 0;
    left: -25vw;
    display: flex;
    flex-direction: column;
    width: 25vw;
    height: 100vh;
    border: none;
    color: #fff;
    background-color: #1f2428;
}
.title{
    margin: auto;
    margin-top: 15px;
}
</style>

