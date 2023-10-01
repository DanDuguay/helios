<script setup lang="ts">
</script>

<template>
    <div :class="$style.container">

        <div :class="$style.card">
            <p :class="$style.title">Add Summary Group</p>

            <div :class="$style.middleAdd">
                <div :class="$style.miniCard">
                    <label :class="$style.label">Group Name</label>
                    <input :class="$style.sInput"/>
                </div>

                <div :class="$style.miniCard">
                    <label :class="$style.label">Max Limit</label>
                    <input :class="$style.sInput"/>
                </div>

                <div :class="$style.miniCard">
                    <label :class="$style.label">Min Limit</label>
                    <input :class="$style.sInput"/>
                </div>

            </div>

            <div :class="$style.groupDes">
                <label :class="$style.label">Description</label>
                <input :class="$style.Description" />
            </div>

            <div :class="$style.footer">
                <button :class="[$style.btn,$style.add]">Save</button>
                <button :class="[$style.btn,$style.remove]">Clear</button>
            </div>
    
        </div>

        <div :class="$style.card">
            <p :class="$style.title">Summary Group List</p>

            <ul :class="$style.table">
                <li :class="$style.line">
                    <span :class="$style.lineItem">ID</span>
                    <span :class="$style.lineItem">Description</span>
                    <span :class="$style.lineItem">Group</span>
                    <span :class="$style.lineItem">Max Limit</span>
                    <span :class="$style.lineItem">Min Limit</span>
                </li>
            </ul>
        </div>

        <div :class="$style.card">
            <p :class="$style.title">Simulations</p>

            <ul :class="$style.table">
                <li :class="$style.line">
                    <span :class="$style.lineItem">ID</span>
                    <span :class="$style.lineItem">Name</span>
                </li>
            </ul>

        </div>

    </div>
</template>

<style module>

.container{
    display: flex;
    padding: 0;
    flex-direction: column;
    width: 100%;
}
.card{
    margin: auto;
    margin-top: 30px;
    background-color: #ddd;
    border:none;
    border-radius: 5px;
    width: 90%;
    display: flex;
    flex-direction: column;
}

.miniCard{
    margin: auto;
    margin-left: 5px;
}
.sInput{
    height: 25px;
    border-radius: 12.5px;
    width: 15vw;
    min-width: 150px;
    background-color: #fff;
    margin: 15px;
    padding: 5px;
    border: none;
}

.btn{
    background-color: transparent;
    background-color: #fff;
    border:none;
    border-radius: 15px;
    width: 60px;
    transition: 0.5s;
    height: 30px;
    margin: 10px;
    cursor: pointer;
    font-weight: 600;
}

.btn:hover{
    margin-top: 9px;
    color: #fff;
    margin-bottom: 11px;
}
.add:hover{
    background-color: #318c69;
}

.remove:hover{
    background-color: red;
}

.middleAdd{
    display: flex;
    width: 100%;
    padding: 15px;
}
.footer{
    margin-left: auto;
    margin-right: 30px;
    display: flex;
}
.label{
    font-weight: 600;
}
.groupDes{
    margin: 20px;
}
.Description{
    border-radius: 12.5px;
    border:none;
    width: 80%;
    height: 25px;
    padding: 5px;
    margin: 10px;
}
.title{
    font-size: 25px;
    font-weight: 700;
    margin: auto;
    margin-top: 25px;
}
.table{
    display: flex;
    flex-direction: column;
    list-style: none;
    padding: 0;
    background-color: #fff;
    border-radius: 5px;
    margin: auto;
    margin-top: 25px;
    margin-bottom: 25px;
    width: 90%;
}
.line{
    display: flex;
}
.lineItem{
    margin: auto;
}
</style>