<script setup lang="ts">
</script>

<template>

    <div :class="$style.container">

                <ul :class="$style.list">
                    <li :class="[$style.item , $style.logo]">
                        <font-awesome-icon :class="$style.icon" :icon="['fas', 'fire']" />
                    </li>
                    <li :class="[$style.item]">
                        <font-awesome-icon :class="$style.icon" :icon="['fas', 'chart-bar']" />
                    </li>
                    <li :class="$style.item">
                        <font-awesome-icon :class="$style.icon" :icon="['fas', 'plus']" />
                    </li>
                    <li :class="$style.item">
                        <font-awesome-icon :class="$style.icon" :icon="['fas', 'layer-group']" />
                    </li>
                    <li :class="$style.item ">
                        <font-awesome-icon :class="$style.icon" :icon="['fas', 'gear']" />
                    </li>
                </ul>


    </div>

</template>

<style module>
.container{
    padding: 0;
    position: fixed;
    left: 0;
    top: 0;
    background-color: #24292E;
    border: none;
    width: 7vw;
    box-shadow: 5px 0px 10px #666;
    max-width: 55px;
    height: 100vh;
}

.list{
    display: flex;
    height: 100%;
    flex-direction: column;
    padding: 0;
    margin: 0;
    list-style: none;
}

.item{
    margin: 0;
    width: 100%;
    height: 55px;
    border: none;
    cursor: pointer;
    display: flex;
    color: #fff;
}
.icon{
    margin: auto;
    font-size: larger;
    transition: 0.5s;
}
.item:hover{
    color: #f9826C;
}
.logo{
    color: #f9826C;
    cursor: default;
}
.active{
    border-left-width: 2px;
    border-left-style: solid;
    border-left-color: #f9826C;
}
</style>