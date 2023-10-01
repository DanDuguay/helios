<script setup lang="ts">


</script>

<template>

    <div :class="$style.container">

                <ul :class="$style.list">

                    <li :class="[$style.item , $style.logo]">
                        <router-link to="/">
                            <font-awesome-icon :class="$style.icon" :icon="['fas', 'fire']" />
                        </router-link>
                    </li>
                    
                    <li :class="[$style.item]">
                        <router-link to="/simulations">
                            <font-awesome-icon :class="$style.icon" :icon="['fas', 'chart-bar']" />
                        </router-link>
                    </li>

                    <li :class="$style.item">
                        <router-link to="/add">
                            <font-awesome-icon :class="$style.icon" :icon="['fas', 'plus']" />
                        </router-link>
                    </li>

                    <li :class="$style.item">
                        <router-link to="/groups">
                            <font-awesome-icon :class="$style.icon" :icon="['fas', 'layer-group']" />
                        </router-link>
                    </li>

                    <li :class="$style.item">
                        <router-link to="/setting">
                            <font-awesome-icon :class="$style.icon" :icon="['fas', 'gear']" />
                        </router-link>
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