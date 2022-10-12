<template lang="pug">
.container
  textarea(v-model="value")
  button(@click="submit") 确定
</template>

<script lang="ts">

export default {
  data() {
    return {
      name: 'Home',
      value: "{}",
      m3u8s: [],
    };
  },
  methods: {
    async sendToContent(params) {
      let [tab] = await chrome.tabs.query({
        active: true,
        currentWindow: true
      });
      params.url = tab.url;
      params.title = tab.title;
      if (tab && tab.id) {
        const port = chrome.tabs.connect(tab.id, {
          name: 'ffmpeger'
        });
        port.postMessage(params);
        port.onMessage.addListener((msg) => {
          console.log('response:', msg);
          this.value = JSON.stringify(msg, null, 2);
        });
      }
    },
    async submit() {
      await this.sendToContent({ msg: "Hello", source: "popup" })
    }
  },
};
</script>

<style lang="styl" scoped>

</style>