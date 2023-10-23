<script setup lang="ts">
</script>

<template>
    <div :class="$style.container">

        <p :class="$style.title">Add Simulation</p>

        <div :class="$style.inputSection">
            <input type="file" :class="$style.input"/>
            <button :class="$style.uploadBtn">
                <font-awesome-icon :icon="['fas', 'plus']" />
                Upload
            </button>
            <p :class="$style.info">
                You can Drag & Drop
            </p>
        </div>

        <div :class="$style.fileSection">    
            <p :class="$style.fileItem">
                <font-awesome-icon :class="$style.icon" :icon="['fas', 'file']" />
                File1.pdf
                <font-awesome-icon :class="$style.trash" :icon="['fas', 'trash']" />
            </p>
            <p :class="$style.fileItem">
                <font-awesome-icon :class="$style.icon" :icon="['fas', 'file']" />
                File2.pdf
                <font-awesome-icon :class="$style.trash" :icon="['fas', 'trash']" />
            </p>
        </div>

    </div>
</template>

<style module>
.container{
    display:flex;
    width:70%;
    flex-direction: column;
    margin: auto;
    max-width: 700px;
    margin-top: 15vh;
}
.inputSection{
    background-color: rgba(100,100,100,0.1);
    display: flex;
    border-radius: 5px;
    height: 150px;
    min-height: 20vh;
    border-style: dashed;
    flex-direction: column;
    border-width: 3px;
    border-color: rgba(0, 0, 0, 0.2);
    cursor: pointer;
}
.input{
    display: none;
}
.info{
    font-size: 12px;
    margin-left: auto;
    margin-right: auto;
    margin-top: 0px;
    margin-bottom: auto;
}
.fileSection{
    display: flex;
    flex-direction: column;
    margin-top: 10px;
}
.title{
    font-weight: 600;
}
.uploadBtn{
    width: 50%;
    background-color: rgb(249,130,108);
    border:none;
    height: 40px;
    margin: auto;
    margin-bottom: 5px;
    font-size: 18px;
    cursor: pointer;
    font-weight: 600;
    border-radius: 5px;
}

.fileItem{
    background-color: rgba(249,130,108,0.2);
    color: #000;
    font-weight: 500;
    border-radius: 5px;
    margin-top: 10px;
    margin-bottom: 5px;
    padding: 10px;
    align-items: center;
    display: flex;
    transition: 0.5s;
}
.fileItem:hover{
    margin-top: 7px;
    margin-bottom: 8px;
}

.icon{
    color:rgb(249,130,108);
    margin-left: 5px;
    margin-right: 20px;
}
.trash{
    color:rgb(249,130,108);
    margin-right: 5px;
    cursor: pointer;
    margin-left: auto;
}
</style>