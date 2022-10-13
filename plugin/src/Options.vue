<template lang="pug">
.container
  .options
    h1 {{title}}
    label(for="baseUrl") baseUrl
    input(type="text", name="baseUrl" v-model="baseUrl" placeholder="baseUrl")
    label(for="storagePath") storagePath
    input(type="text", name="storagePath" v-model="storagePath" placeholder="storagePath")
    label(for="taskLimit") taskLimit
    input(type="text", name="taskLimit" v-model="taskLimit" placeholder="taskLimit")
    button(@click="submit") Submit
</template>

<script lang="ts">
export default {
  data() {
    return {
      title: "Options",
      baseUrl: "",
      storagePath: "",
      taskLimit: 0,
    };
  },
  mounted() {
    this.init();
  },
  methods: {
    async init() {
      const { baseUrl, storagePath, taskLimit } =
        await chrome.storage.local.get(["baseUrl", "storagePath", "taskLimit"]);
      if (!!baseUrl) {
        this.baseUrl = baseUrl;
      }
      if (!!storagePath) {
        this.storagePath = storagePath;
      }
      if (!!taskLimit) {
        this.taskLimit = taskLimit;
      }
    },
    async submit() {
      await chrome.storage.local.set({
        baseUrl: this.baseUrl,
        storagePath: this.storagePath,
        taskLimit: this.taskLimit,
      });
      alert("done!");
    },
  },
};
</script>

<style lang="styl" scoped>
.container
  display: flex;
.options
  display: flex;
  height:500px;
  width: 800px;
  flex-direction: column;
  padding: 100px;
input
  margin: 10px 0;
  width: 400px;
  height: 32px;
  border-radius: 5px;
  outline: none;
  padding: 2px 5px;
  border: 1px #ccc solid;
button
  cursor: pointer;
  padding: 10px;
  border: none;
  border-radius: 4px;
  outline: none;
  color: white;
  width 80px;
  background-color: cornflowerblue;
</style>
