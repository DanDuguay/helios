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
    left: 6.5vw;
    display: flex;
    flex-direction: column;
    width: 25vw;
    height: 100vh;
    border: none;
    color: #fff;
    background-color: #1f2428;
}
.title{
    text-align: center;
    padding-top: 1px;
    padding-bottom: 10px;
    border-bottom-style: solid;
    font-weight: 400;
    border-bottom-color: #333;
    width: 100%;
    border-bottom-width: 1px;
}
</style>

