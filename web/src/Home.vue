<template lang="pug">
.container
  h1 Home

</template>

<script lang="ts">
import axios from "axios";
axios.defaults.baseURL = window.location.href;
export default {
  data() {
    return {
      tasks: [],
    };
  },
  mounted() {
    this.init();
  },
  methods: {
    async init() {
      const result = await axios.get("/task");
      this.tasks = result.data.list;
    },
  },
};
</script>

<style lang="styl" scoped></style>
