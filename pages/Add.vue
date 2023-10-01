<script setup lang="ts">
</script>

<template>
    <div :class="$style.container">
        <div :class="$style.form">
            <p :class="$style.title">You can drag and drop files here to add them</p>
            <button :class="$style.add">Add</button>
        </div>
    </div>
</template>

<style module>
.container{
    display:flex;
    width:100%;
    height: 100vh;
}
.input{
    visibility: hidden;
}
.form{
    border-style: dashed;
    background-color: rgba(0,0,0,0.2);
    cursor: pointer;
    border-width: 5px;
    border-color: #f9826C;
    display: flex;
    flex-direction: column;
    margin: auto;
    max-width: 800px;
    max-height: 800px;
    width: 60vw;
    height: 60vw;
    border-radius: 10px;
}
.title{
    font-weight: 700;
    margin: auto;
    margin-bottom: 15px;
    font-size: large;
}
.add{
    width: 300px;
    height: 50px;
    background-color: #f9826C;
    cursor: pointer;
    border: none;
    border-radius: 10px;
    margin: auto;
    margin-top: 15px;
}
</style>