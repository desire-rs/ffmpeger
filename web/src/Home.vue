<template lang="pug">
.container
  h1 Home
</template>

<script lang="ts">
import * as axios from 'axios';
axios.default.defaults.baseURL = window.location.hostname;
export default {
  data() {
    return {
    };
  },
  methods: {

  },
};
</script>

<style lang="styl" scoped>

</style>