<script setup lang="ts">
</script>

<template>

    <div :class="$style.container">

        <span>
            <button :class="$style.btn">
                <font-awesome-icon :icon="['fas', 'bars']" />
                <font-awesome-icon icon="fa-solid fa-user-secret" />
            </button>
        </span>

        <ul :class="$style.list">
            <li>option1</li>
            <li>optoin2</li>
            <li>option3</li>
            <li>optoin4</li>
            <li>option5</li>
        </ul>

    </div>

</template>

<style module>
.container{
    padding: 0;
    position: fixed;
    left: 0;
    top: 0;
    background-color: #111;
    display: flex;
    flex-direction: column;
    border: none;
    width: 7vw;
    box-shadow: 5px 0px 10px #666;
    max-width: 55px;
    height: 100vh;
}

.list{
    display: flex;
    flex-direction: column;
    visibility: hidden;
}

.btn{
    margin-top: 0;
    width: 100%;
    height: 55px;
    box-shadow: none;
    border: none;
    background-color: transparent;
    cursor: pointer;
    font-size: large;
    background-color: #222;
    color: #fff;
}
</style>