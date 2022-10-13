<template lang="pug">
.container
  .control
    button.btn(@click="submit") submit
    button.btn(@click="clear") clear
  .content
    .item(v-for="(url, index) in urls")
      input(type="checkbox"  v-model="targets" :value="url")
      label {{title-index}}
      label {{url}}
    textarea(cols='30' rows='25' v-model="info" style='color:red;')
    //- textarea(cols='30' rows='4' style='color:red;' v-model="result")
</template>

<script lang="ts">
interface IMsg {
  m3u8s: string[];
}
export default {
  data() {
    return {
      title: "", // title
      urls: [], // 捕获的urls
      targets: [], // 选中的urls
      info: "{}", // 日志信息
      result: "{}", // 结果信息
    };
  },
  mounted() {
    this.init();
  },
  methods: {
    async init() {
      const { m3u8 } = await chrome.storage.local.get("m3u8");
      console.log(m3u8);
      if (!!m3u8) {
        this.urls = JSON.parse(m3u8);
        this.targets = this.urls;
      }
    },
    async clear() {
      const result = confirm("确定清空内容?");
      if (result) {
        await chrome.storage.local.remove(["m3u8"]);
        this.urls = [];
        this.targets = [];
      }
    },
    async sendToContent(params: IMsg) {
      let [tab] = await chrome.tabs.query({
        active: true,
        currentWindow: true,
      });
      if (tab && tab.id) {
        const port = chrome.tabs.connect(tab.id, {
          name: "ffmpeger",
        });
        port.postMessage(params);
        port.onMessage.addListener((msg) => {
          console.log("response:", msg);
          this.info = JSON.stringify(msg, null, 2);
        });
      }
    },
    async submit() {
      await this.sendToContent({ m3u8s: this.targets });
      await chrome.storage.local.remove(["m3u8"]);
      this.urls = [];
      this.targets = [];
    },
  },
};
</script>

<style lang="styl" scoped>
html,
body
  margin: 0;
  padding: 0;
.container
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
.content
  display: flex;
  flex-direction: column;
  flex-wrap: wrap;
body
  margin: 0;
  padding: 10px;
textarea
  width: 700px;
  margin-top: 5px;
  border-radius: 4px;
  padding: 5px;
  outline: none;

button
  cursor: pointer;
  padding: 8px 18px;
  border: none;
  border-radius: 4px;
  outline: none;
  color: white;
  margin: 0 5px;;
  background-color: cornflowerblue;
.item
  display: flex;
  flex-direction: row;
  height: 30px;
  justify-content:flex-start;
  algin-items: center;
  line-height: 30px;
</style>
